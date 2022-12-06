use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U64,
    log,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{manage_storage_deposit, MainchainContract, MainchainContractExt};

/// Node information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct Node {
    pub owner:          AccountId,
    pub pending_owner:  Option<AccountId>,
    pub socket_address: String, // ip address and port
}

/// Update node commands
#[derive(Deserialize, Serialize)]
pub enum UpdateNode {
    AcceptOwnership,
    SetPendingOwner(String),
    SetSocketAddress(String),
}

/// Contract private methods
impl MainchainContract {
    pub fn get_expect_node(&self, node_id: u64) -> Node {
        self.nodes.get(&node_id).expect("Node does not exist")
    }

    pub fn assert_node_owner(&self, account_id: &AccountId, node_owner: &AccountId) {
        assert_eq!(account_id, node_owner, "Only {} can call this method", node_owner);
    }

    pub fn assert_node_pending_owner(&self, account_id: &AccountId, node_pending_owner: &Option<AccountId>) {
        let pending_owner = node_pending_owner.as_ref().expect("Node does not have a pending owner");
        assert_eq!(account_id, pending_owner, "Only {} can call this method", pending_owner);
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    /// Registers a new node while charging for storage usage
    #[payable]
    pub fn register_node(&mut self, socket_address: String) -> Option<String> {
        manage_storage_deposit!(self, "require", {
            // require valid socket address characters
            for c in socket_address.chars() {
                assert!(
                    c.is_numeric() || c.is_alphabetic() || c == '.' || c == ':',
                    "Invalid socket address"
                );
            }

            self.num_nodes += 1;
            let node_id = self.num_nodes;
            let account_id = env::signer_account_id();

            // create a new node
            log!("{} registered node_id {}", account_id, node_id);
            let node = Node {
                owner: account_id,
                pending_owner: None,
                socket_address,
            };
            self.nodes.insert(&node_id, &node);
        }); // end manage_storage_deposit

        Some(self.num_nodes.to_string())
    }

    /// Removes a node and refunds storage deposit
    pub fn unregister_node(&mut self, node_id: U64) {
        manage_storage_deposit!(self, "refund", {
            let account_id = env::signer_account_id();
            let node = self.get_expect_node(node_id.into());

            self.assert_node_owner(&account_id, &node.owner);

            log!("{} removed node_id {}", account_id, u64::from(node_id));
            self.nodes.remove(&node_id.into());
        }); // end manage_storage_deposit
    }

    /// Updates one of the node's fields
    pub fn update_node(&mut self, node_id: U64, command: UpdateNode) {
        manage_storage_deposit!(self, {
            let account_id = env::signer_account_id();
            let mut node = self.get_expect_node(node_id.into());

            match command {
                UpdateNode::AcceptOwnership => {
                    self.assert_node_pending_owner(&account_id, &node.pending_owner);
                    log!("{} became owner of node_id {}", account_id, u64::from(node_id),);
                    node.owner = account_id;
                    node.pending_owner = None;
                }
                UpdateNode::SetPendingOwner(new_pending_owner) => {
                    self.assert_node_owner(&account_id, &node.owner);
                    log!(
                        "{} updated node_id {} pending_owner to {}",
                        account_id,
                        u64::from(node_id),
                        new_pending_owner
                    );
                    node.pending_owner = Some(new_pending_owner.parse().unwrap());
                }
                UpdateNode::SetSocketAddress(new_socket_address) => {
                    self.assert_node_owner(&account_id, &node.owner);
                    log!(
                        "{} updated node_id {} socket address to {}",
                        account_id,
                        u64::from(node_id),
                        new_socket_address
                    );
                    node.socket_address = new_socket_address;
                }
            }

            self.nodes.insert(&node_id.into(), &node);
        }); // end manage_storage_deposit
    }

    pub fn get_node(&self, node_id: U64) -> Option<Node> {
        self.nodes.get(&node_id.into())
    }

    pub fn get_nodes(&self, limit: U64, offset: U64) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut node_id = self.num_nodes - u64::from(offset);
        let limit = u64::from(limit);
        while node_id > 0 && nodes.len() < limit.try_into().unwrap() {
            if let Some(node) = self.nodes.get(&node_id) {
                nodes.push(node);
            }
            node_id -= 1;
        }
        nodes
    }
}
