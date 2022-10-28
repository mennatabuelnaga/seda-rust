use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env,
    json_types::U64,
    log,
    near_bindgen,
    AccountId,
    BorshStorageKey, Promise,
};

/// LookupMap keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    NumNodes,
}
/// Node information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Node {
    pub owner:          AccountId,
    pub pending_owner:  Option<AccountId>,
    pub socket_address: String, // ip address and port
}

/// Contract global state
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    num_nodes: u64,
    nodes:     LookupMap<u64, Node>,
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

impl Default for MainchainContract {
    fn default() -> Self {
        Self::new()
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new() -> Self {
        Self {
            num_nodes: 0,
            nodes:     LookupMap::new(MainchainStorageKeys::NumNodes),
        }
    }

    /// Registers a new node while charging for storage usage
    #[payable]
    pub fn register_node(&mut self, socket_address: String) -> Option<String> {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();

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

        // check for storage deposit
        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
        Some(self.num_nodes.to_string())
    }

    /// Removes a node and refunds storage deposit
    pub fn remove_node(&mut self, node_id: U64) {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();

        let account_id = env::signer_account_id();
        let node = self.get_expect_node(node_id.into());

        self.assert_node_owner(&account_id, &node.owner);

        log!("{} removed node_id {}", account_id, u64::from(node_id));
        self.nodes.remove(&node_id.into());

        // refund storage deposit
        let storage_cost = env::storage_byte_cost() * u128::from(initial_storage_usage - env::storage_usage());
        Promise::new(account_id.clone()).transfer(storage_cost);
        log!("Refunding {} for storage deposit to {}", storage_cost, account_id);
    }

    /// Updates the pending owner of a node
    pub fn set_node_pending_owner(&mut self, node_id: U64, new_owner: String) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_node_owner(&account_id, &node.owner);

        log!(
            "{} updated node_id {} pending_owner to {}",
            account_id,
            u64::from(node_id),
            new_owner
        );
        node.pending_owner = Some(new_owner.parse().unwrap());
        self.nodes.insert(&u64::from(node_id), &node);
    }

    /// Finalizes the pending owner change
    pub fn become_node_owner(&mut self, node_id: U64) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_node_pending_owner(&account_id, &node.pending_owner);

        log!("{} became owner of node_id {}", account_id, u64::from(node_id),);
        node.owner = account_id;
        node.pending_owner = None;
        self.nodes.insert(&u64::from(node_id), &node);
    }

    pub fn set_node_socket_address(&mut self, node_id: U64, new_socket_address: String) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_node_owner(&account_id, &node.owner);

        log!(
            "{} updated node_id {} socket address to {}",
            account_id,
            u64::from(node_id),
            new_socket_address
        );
        node.socket_address = new_socket_address;
        self.nodes.insert(&node_id.into(), &node);
    }

    pub fn get_node_owner(&self, node_id: U64) -> Option<AccountId> {
        log!("get_node_owner for node_id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(node.owner),
            None => None,
        }
    }

    pub fn get_node_pending_owner(&self, node_id: U64) -> Option<AccountId> {
        log!("get_node_pending_owner for node_id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => node.pending_owner,
            None => None,
        }
    }

    pub fn get_node_socket_address(&self, node_id: U64) -> Option<String> {
        log!("get_node_socket_address for node_id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(node.socket_address),
            None => None,
        }
    }
}
