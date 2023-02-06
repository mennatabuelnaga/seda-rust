use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::{U128, U64},
    log,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
    PromiseError,
    PromiseOrValue,
};

use crate::{
    consts::{EPOCH_DELAY_FOR_ELECTION, GAS_FOR_FT_ON_TRANSFER, MINIMUM_STAKE},
    fungible_token::ft,
    manage_storage_deposit,
    MainchainContract,
    MainchainContractExt,
};

/// Node information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct Node {
    /// The IP address and port of the node
    pub socket_address:      String,
    pub balance:             Balance,
    pub epoch_when_eligible: u64,
}

/// Human-readable node information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct HumanReadableNode {
    /// The NEAR account id of the node
    pub account_id:          AccountId,
    /// The IP address and port of the node
    pub socket_address:      String,
    pub balance:             Balance,
    pub epoch_when_eligible: U64,
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

    pub fn assert_valid_socket_address(&self, socket_address: &String) {
        for c in socket_address.chars() {
            assert!(
                c.is_numeric() || c.is_alphabetic() || c == '.' || c == ':',
                "Invalid socket address"
            );
        }
    }

    pub(crate) fn is_eligible_for_current_epoch(&self, node: &Node) -> bool {
        node.epoch_when_eligible > 0 && node.epoch_when_eligible <= self.get_current_epoch()
    }

    pub(crate) fn has_minimum_stake(&self, node: &Node) -> bool {
        node.balance >= MINIMUM_STAKE
    }

    pub(crate) fn assert_eligible_to_propose(&self, account_id: &AccountId) {
        let node = self.internal_get_node(&account_id);
        assert!(
            self.is_eligible_for_current_epoch(&node),
            "Account is not eligible for this epoch"
        );
        assert!(
            self.has_minimum_stake(&node),
            "Account balance is less than minimum stake"
        );
    }

    pub(crate) fn internal_deposit(&mut self, amount: u128, account_id: AccountId) -> PromiseOrValue<U128> {
        let mut node = self.get_expect_node(account_id.clone());
        node.balance += amount;
        if node.balance >= MINIMUM_STAKE {
            node.epoch_when_eligible = env::epoch_height() + EPOCH_DELAY_FOR_ELECTION;
        }
        self.nodes.insert(&account_id, &node);
        self.last_total_balance += amount;

        env::log_str(format!("@{} deposited {}. New balance is {}", account_id, amount, node.balance).as_str());

        PromiseOrValue::Value(U128::from(0)) // no refund
    }

    pub fn internal_withdraw(&mut self, amount: Balance) {
        assert!(amount > 0, "Withdrawal amount should be positive");
        let account_id = env::predecessor_account_id();
        let mut node = self.internal_get_node(&account_id);
        assert!(node.balance >= amount, "Not enough balance to withdraw");

        // update account
        node.balance -= amount;
        if node.balance < MINIMUM_STAKE {
            node.epoch_when_eligible = 0;
        }
        self.nodes.insert(&account_id, &node);

        // transfer the tokens, then validate/update state in `withdraw_callback()`
        ft::ext(self.seda_token.clone())
            .with_static_gas(GAS_FOR_FT_ON_TRANSFER)
            .with_attached_deposit(1)
            .ft_transfer(account_id.clone(), amount.into(), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER)
                    .withdraw_callback(account_id, amount.into()),
            );
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    /// Registers a new node while charging for storage usage
    #[payable]
    pub fn register_node(&mut self, socket_address: String) {
        // require valid socket address characters
        self.assert_valid_socket_address(&socket_address);

        manage_storage_deposit!(self, "require", {
            // create a new node
            let account_id = env::signer_account_id();
            log!("{} registered node", account_id);
            let node = Node {
                socket_address,
                balance: 0,
                epoch_when_eligible: 0,
            };
            self.nodes.insert(&account_id, &node);
        }); // end manage_storage_deposit
    }

    /// Updates one of the node's fields
    pub fn update_node(&mut self, command: UpdateNode) {
        manage_storage_deposit!(self, {
            let account_id = env::signer_account_id();
            let mut node = self.get_expect_node(account_id.clone());

            match command {
                UpdateNode::SetSocketAddress(new_socket_address) => {
                    self.assert_valid_socket_address(&new_socket_address);
                    log!("{} updated node socket address to {}", account_id, new_socket_address);
                    node.socket_address = new_socket_address;
                }
            }

            self.nodes.insert(&account_id, &node);
        }); // end manage_storage_deposit
    }

    pub fn is_eligible_to_propose(&self, account_id: AccountId) -> bool {
        let node = self.internal_get_node(&account_id);
        self.is_eligible_for_current_epoch(&node) && self.has_minimum_stake(&node)
    }

    #[private] // require caller to be this contract
    pub fn deposit(&mut self, amount: u128, account_id: AccountId) -> PromiseOrValue<U128> {
        self.internal_deposit(amount, account_id)
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

    #[private] // require caller to be this contract
    pub fn withdraw_callback(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) {
        let mut node = self.internal_get_node(&account_id);
        if call_result.is_err() {
            env::log_str("withdraw failed");
            // revert withdrawal
            node.balance += amount.0;
            if node.balance >= MINIMUM_STAKE {
                node.epoch_when_eligible = self.get_current_epoch() + EPOCH_DELAY_FOR_ELECTION;
            }
            self.nodes.insert(&account_id, &node);
            return;
        }

        env::log_str(
            format!(
                "@{} withdrawing {}. New balance is {}",
                account_id, amount.0, node.balance
            )
            .as_str(),
        );

        // update global balance
        self.last_total_balance -= amount.0;
    }

    /*************** */
    /* View methods */
    /*************** */

    /// Returns the balance of the given account.
    pub fn get_node_balance(&self, account_id: AccountId) -> U128 {
        U128(self.internal_get_node(&account_id).balance)
    }

    pub fn get_node(&self, node_id: AccountId) -> Option<HumanReadableNode> {
        let node = self.nodes.get(&node_id);
        if let Some(node) = node {
            Some(HumanReadableNode {
                account_id:          node_id,
                socket_address:      node.socket_address,
                balance:             node.balance,
                epoch_when_eligible: node.epoch_when_eligible.into(),
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
                    socket_address:      node.socket_address,
                    balance:             node.balance,
                    epoch_when_eligible: node.epoch_when_eligible.into(),
                };
                nodes.push(human_readable_node);
            }
            index -= 1;
        }
        nodes
    }
}
