use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::{U128, U64},
    log,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
};

use crate::{manage_storage_deposit, MainchainContract, MainchainContractExt};

/// Node information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct Node {
    /// The IP address and port of the node
    pub multi_addr:          String,
    pub balance:             Balance,
    pub epoch_when_eligible: u64,
    pub bn254_public_key:    Vec<u8>,
}

/// Human-readable node information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct HumanReadableNode {
    /// The NEAR account id of the node
    pub account_id:          AccountId,
    /// The IP address and port of the node
    pub multi_addr:          String,
    pub balance:             Balance,
    pub epoch_when_eligible: U64,
    pub bn254_public_key:    Vec<u8>,
}

/// Update node commands
#[derive(Deserialize, Serialize)]
pub enum UpdateNode {
    SetSocketAddress(String),
}

/// Contract private methods
impl MainchainContract {
    pub fn internal_get_node(&self, account_id: &AccountId) -> Node {
        self.nodes.get(account_id).unwrap_or_default()
    }

    pub fn get_expect_node(&self, node_id: AccountId) -> Node {
        self.nodes.get(&node_id).expect("Node does not exist")
    }

    pub(crate) fn is_eligible_for_current_epoch(&self, node: &Node) -> bool {
        node.epoch_when_eligible > 0 && node.epoch_when_eligible <= self.get_current_epoch()
    }

    pub(crate) fn has_minimum_stake(&self, node: &Node) -> bool {
        node.balance >= self.config.minimum_stake
    }

    pub(crate) fn assert_eligible_for_current_epoch(&self, account_id: &AccountId) {
        let node = self.internal_get_node(account_id);
        assert!(
            self.is_eligible_for_current_epoch(&node),
            "Account is not eligible for this epoch"
        );
        assert!(
            self.has_minimum_stake(&node),
            "Account balance is less than minimum stake"
        );
    }

    pub fn internal_deposit(&mut self, amount: Balance) {
        manage_storage_deposit!(self, "require", {
            let account_id = env::signer_account_id();

            // subtract from user balance and add to contract balance
            let new_user_balance = self.token.accounts.get(&account_id).unwrap() - amount;
            self.token.accounts.insert(&account_id, &new_user_balance);
            let mut node = self.get_expect_node(account_id.clone());
            node.balance += amount;

            // set epoch when the node is eligible if minimum stake is reached
            if node.balance >= self.config.minimum_stake {
                node.epoch_when_eligible = env::epoch_height() + self.config.epoch_delay_for_election;
            }

            // update the node entry and total balance of the contract
            self.nodes.insert(&account_id, &node);
            self.last_total_balance += amount;

            env::log_str(format!("@{} deposited {}. New balance is {}", account_id, amount, node.balance).as_str());
        });
    }

    pub fn internal_withdraw(&mut self, amount: Balance) {
        // TODO: epoch delay for withdrawal
        manage_storage_deposit!(self, "require", {
            assert!(amount > 0, "Withdrawal amount should be positive");
            let account_id = env::predecessor_account_id();
            let mut node = self.internal_get_node(&account_id);
            env::log_str(format!("{} balance is {}", account_id, node.balance).as_str());
            assert!(node.balance >= amount, "Not enough balance to withdraw");

            // subtract from contract balance and add to user balance
            node.balance -= amount;
            if node.balance < self.config.minimum_stake {
                node.epoch_when_eligible = 0;
            }
            self.nodes.insert(&account_id, &node);
            let new_user_balance = self.token.accounts.get(&account_id).unwrap() + amount;
            self.token.accounts.insert(&account_id, &new_user_balance);

            // update global balance
            self.last_total_balance -= amount;

            env::log_str(
                format!(
                    "@{} withdrawing {}. New balance is {}",
                    account_id, amount, node.balance
                )
                .as_str(),
            );
        });
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    /// Registers a new node while charging for storage usage
    #[payable]
    pub fn register_node(&mut self, multi_addr: String, bn254_public_key: Vec<u8>, signature: Vec<u8>) {
        let account_id = env::signer_account_id();

        // assert unique bn254_public_key
        assert!(
            !self.nodes_by_bn254_public_key.contains_key(&bn254_public_key.clone()),
            "bn254_public_key already exists"
        );

        // verify the signature
        assert!(
            self.bn254_verify(account_id.as_bytes().to_vec(), signature, bn254_public_key.clone()),
            "Invalid signature"
        );

        // create a new node
        log!("{} registered node", account_id);
        let node = Node {
            multi_addr,
            balance: 0,
            epoch_when_eligible: 0,
            bn254_public_key: bn254_public_key.clone(),
        };

        manage_storage_deposit!(self, "require", {
            // insert in nodes
            self.nodes.insert(&account_id, &node);

            // insert in nodes_by_bn254_public_key
            self.nodes_by_bn254_public_key.insert(&bn254_public_key, &account_id);
        });
    }

    /// Updates one of the node's fields
    pub fn update_node(&mut self, command: UpdateNode) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(account_id.clone());

        match command {
            UpdateNode::SetSocketAddress(new_multi_addr) => {
                log!("{} updated node multi_addr to {}", account_id, new_multi_addr);
                node.multi_addr = new_multi_addr;
            }
        }

        manage_storage_deposit!(self, {
            self.nodes.insert(&account_id, &node);
        });
    }

    pub fn deposit(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        self.internal_deposit(amount);
    }

    /// Withdraws the balance for given account.
    pub fn withdraw(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        self.internal_withdraw(amount);
    }

    /// Withdraws the entire balance from the predecessor account.
    pub fn withdraw_all(&mut self) {
        let account_id = env::predecessor_account_id();
        let account = self.internal_get_node(&account_id);
        self.internal_withdraw(account.balance);
    }

    /*************** */
    /* View methods */
    /*************** */

    pub fn is_node_active(&self, account_id: AccountId) -> bool {
        let node = self.internal_get_node(&account_id);
        self.is_eligible_for_current_epoch(&node) && self.has_minimum_stake(&node)
    }

    /// Returns the balance of the given account.
    pub fn get_node_balance(&self, account_id: AccountId) -> U128 {
        U128(self.internal_get_node(&account_id).balance)
    }

    pub fn get_node(&self, node_id: AccountId) -> Option<HumanReadableNode> {
        let node = self.nodes.get(&node_id);
        if let Some(node) = node {
            Some(HumanReadableNode {
                account_id:          node_id,
                multi_addr:          node.multi_addr,
                balance:             node.balance,
                epoch_when_eligible: node.epoch_when_eligible.into(),
                bn254_public_key:    node.bn254_public_key,
            })
        } else {
            None
        }
    }

    pub fn get_nodes(&self, limit: U64, offset: U64) -> Vec<HumanReadableNode> {
        let mut nodes = Vec::new();
        let mut index = self.nodes.len() - u64::from(offset);
        let limit = u64::from(limit);
        while index > 0 && nodes.len() < limit.try_into().unwrap() {
            if let Some(node_id) = self.nodes.keys().nth(index as usize - 1) {
                let node = self.nodes.get(&node_id).unwrap();
                let human_readable_node = HumanReadableNode {
                    account_id:          node_id,
                    multi_addr:          node.multi_addr,
                    balance:             node.balance,
                    epoch_when_eligible: node.epoch_when_eligible.into(),
                    bn254_public_key:    node.bn254_public_key,
                };
                nodes.push(human_readable_node);
            }
            index -= 1;
        }
        nodes
    }
}
