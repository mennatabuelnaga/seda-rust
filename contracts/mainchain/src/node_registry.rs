use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U64,
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
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    /// Registers a new node while charging for storage usage
    #[payable]
    pub fn register_node(&mut self, socket_address: String) {
        log!("register_node");
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

    /*************** */
    /* View methods  */
    /*************** */

    pub fn get_node(&self, node_id: AccountId) -> Option<HumanReadableNode> {
        let node = self.nodes.get(&node_id);
        if let Some(node) = node {
            Some(HumanReadableNode {
                account_id:     node_id,
                socket_address: node.socket_address,
                balance:       node.balance,
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
                    account_id:     node_id,
                    socket_address: node.socket_address,
                    balance:       node.balance,
                    epoch_when_eligible: node.epoch_when_eligible.into(),
                };
                nodes.push(human_readable_node);
            }
            index -= 1;
        }
        nodes
    }
}
