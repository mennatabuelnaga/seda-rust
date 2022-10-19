use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env,
    json_types::U64,
    log,
    near_bindgen,
    AccountId,
    BorshStorageKey,
};

/// LookupMap keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    NumNodes,
}
/// Node information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Node {
    pub owner:         AccountId,
    pub pending_owner: Option<AccountId>,
    pub ip_address:    String,
    pub port:          u64,
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
    pub fn register_node(&mut self, ip_address: String, port: U64) {
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
            ip_address,
            port: port.into(),
        };
        self.nodes.insert(&node_id, &node);

        // check for storage deposit
        let storage_cost = env::storage_byte_cost() * u128::from(env::storage_usage() - initial_storage_usage);
        assert!(
            storage_cost <= env::attached_deposit(),
            "Insufficient storage, need {}",
            storage_cost
        );
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

        self.assert_node_pending_owner(&account_id.clone(), &node.pending_owner);

        log!("{} became owner of node_id {}", account_id, u64::from(node_id),);
        node.owner = account_id;
        node.pending_owner = None;
        self.nodes.insert(&u64::from(node_id), &node);
    }

    pub fn set_node_ip_address(&mut self, node_id: U64, new_ip_address: String) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_node_owner(&account_id, &node.owner);

        log!(
            "{} updated node with id {} ip address to {}",
            account_id,
            u64::from(node_id),
            new_ip_address
        );
        node.ip_address = new_ip_address;
        self.nodes.insert(&node_id.into(), &node);
    }

    pub fn set_node_port(&mut self, node_id: U64, new_port: U64) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_node_owner(&account_id, &node.owner);

        log!(
            "{} updated node with id {} port to {}",
            account_id,
            u64::from(node_id),
            u64::from(new_port)
        );
        node.port = new_port.into();
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

    pub fn get_node_ip_address(&self, node_id: U64) -> Option<String> {
        log!("get_node_ip_address for node_id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(node.ip_address),
            None => None,
        }
    }

    pub fn get_node_port(&self, node_id: U64) -> Option<U64> {
        log!("get_node_port for node_id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(U64(node.port)),
            None => None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{
        test_utils::{get_logs, VMContextBuilder},
        testing_env,
        VMContext,
    };

    use super::*;

    fn get_context_view() -> VMContext {
        VMContextBuilder::new().is_view(true).build()
    }
    fn get_context(signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(false)
            .build()
    }
    fn get_context_with_deposit(signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(false)
            .attached_deposit(810_000_000_000_000_000_000) // required for register_node()
            .build()
    }

    #[test]
    fn test_register_and_get_node() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check owner, ip_address, and port
        testing_env!(get_context_view());
        assert_eq!(
            "bob_near".to_string(),
            contract.get_node_owner(U64(1)).unwrap().to_string()
        );
        assert_eq!("0.0.0.0".to_string(), contract.get_node_ip_address(U64(1)).unwrap());
        assert_eq!(U64(8080), contract.get_node_port(U64(1)).unwrap());
        assert_eq!(
            get_logs(),
            vec![
                "get_node_owner for node_id 1",
                "get_node_ip_address for node_id 1",
                "get_node_port for node_id 1"
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Insufficient storage, need 810000000000000000000")]
    fn test_register_not_enough_storage() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
    }

    #[test]
    fn test_remove_node() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));

        // check the ip address after registering
        testing_env!(get_context_view());
        assert_eq!(Some("0.0.0.0".to_string()), contract.get_node_ip_address(U64(1)));

        // remove the node
        testing_env!(get_context("bob_near".to_string()));
        contract.remove_node(U64(1));
        assert_eq!(
            get_logs(),
            vec![
                "bob_near removed node_id 1",
                "Refunding 810000000000000000000 for storage deposit to bob_near"
            ]
        );

        // check the ip address after removing
        testing_env!(get_context_view());
        assert_eq!(None, contract.get_node_ip_address(U64(1)));
    }

    #[test]
    #[should_panic(expected = "Only bob_near can call this method")]
    fn test_remove_node_wrong_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check the ip address after registering
        testing_env!(get_context_view());
        assert_eq!(Some("0.0.0.0".to_string()), contract.get_node_ip_address(U64(1)));

        // try removing the node
        testing_env!(get_context("alice_near".to_string()));
        contract.remove_node(U64(1));
    }

    #[test]
    fn test_set_node_ip_address_and_port() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // update the ip address and port
        contract.set_node_ip_address(U64(1), "1.1.1.1".to_string());
        contract.set_node_port(U64(1), U64(8081));

        // check the ip address and port after updating
        testing_env!(get_context_view());
        assert_eq!("1.1.1.1".to_string(), contract.get_node_ip_address(U64(1)).unwrap());
        assert_eq!(U64(8081), contract.get_node_port(U64(1)).unwrap());
        assert_eq!(
            get_logs(),
            vec!["get_node_ip_address for node_id 1", "get_node_port for node_id 1"]
        );
    }

    #[test]
    fn test_new_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // set pending owner
        contract.set_node_pending_owner(U64(1), "alice_near".to_string());

        // check pending owner
        testing_env!(get_context_view());
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node_pending_owner(U64(1)).unwrap().to_string()
        );

        // accept ownership
        testing_env!(get_context("alice_near".to_string()));
        contract.become_node_owner(U64(1));

        // check owner
        testing_env!(get_context_view());
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node_owner(U64(1)).unwrap().to_string()
        );
    }

    #[test]
    #[should_panic(expected = "Node does not have a pending owner")]
    fn test_wrong_owner_simple() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // accept ownership from wrong owner
        testing_env!(get_context("alice_near".to_string()));
        contract.become_node_owner(U64(1));
    }

    #[test]
    #[should_panic(expected = "Only alice_near can call this method")]
    fn test_wrong_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // set pending owner
        contract.set_node_pending_owner(U64(1), "alice_near".to_string());

        // check pending owner
        testing_env!(get_context_view());
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node_pending_owner(U64(1)).unwrap().to_string()
        );

        // accept ownership from wrong owner
        testing_env!(get_context("franklin_near".to_string()));
        contract.become_node_owner(U64(1));
    }

    #[test]
    fn get_nonexistent_message() {
        let contract = MainchainContract::new();
        testing_env!(get_context_view());
        assert_eq!(None, contract.get_node_ip_address(U64(1)));
        assert_eq!(get_logs(), vec!["get_node_ip_address for node_id 1"])
    }
}
