use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U64;
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey};

/// LookupMap keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    NumNodes,
}
/// Node information
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Node {
    pub owner: AccountId,
    pub pending_owner: Option<AccountId>,
    pub ip_address: String,
    pub port: u64,
}

/// Contract global state
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    num_nodes: u64,
    nodes: LookupMap<u64, Node>,
}

/// Contract private methods
impl MainchainContract {
    pub fn get_expect_node(&self, node_id: u64) -> Node {
        self.nodes.get(&node_id).expect("Node does not exist")
    }
    pub fn assert_permission(&self, account_id: &AccountId, correct_account_id: &AccountId) {
        assert_eq!(
            account_id, correct_account_id,
            "Only {} can call this method",
            correct_account_id
        );
    }
}

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new() -> Self {
        Self {
            num_nodes: 0,
            nodes: LookupMap::new(MainchainStorageKeys::NumNodes),
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

    /// Update the pending owner of a node
    pub fn set_node_pending_owner(&mut self, node_id: U64, new_owner: String) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_permission(&account_id, &node.owner);

        log!(
            "{} updated node_id {} pending_owner to {}",
            account_id,
            u64::from(node_id),
            new_owner
        );
        node.pending_owner = Some(new_owner.parse().unwrap());
        self.nodes.insert(&u64::from(node_id), &node);
    }

    /// Finalize the pending owner change
    pub fn become_node_owner(&mut self, node_id: U64) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_permission(&account_id.clone(), &node.pending_owner.unwrap());

        log!("{} became owner of node_id {}", account_id, u64::from(node_id),);
        node.owner = account_id;
        node.pending_owner = None;
        self.nodes.insert(&u64::from(node_id), &node);
    }

    pub fn set_node_ip_address(&mut self, node_id: U64, new_ip_address: String) {
        let account_id = env::signer_account_id();
        let mut node = self.get_expect_node(node_id.into());

        self.assert_permission(&account_id, &node.owner);

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

        self.assert_permission(&account_id, &node.owner);

        log!(
            "{} updated node with id {} port to {}",
            account_id,
            u64::from(node_id),
            u64::from(new_port)
        );
        node.port = new_port.into();
        self.nodes.insert(&node_id.into(), &node);
    }

    pub fn remove_node(&mut self, node_id: U64) {
        let account_id = env::signer_account_id();
        let node = self.get_expect_node(node_id.into());

        self.assert_permission(&account_id, &node.owner);

        log!("{} removed node with id {}", account_id, u64::from(node_id));
        self.nodes.remove(&node_id.into());

        // TODO: refund storage deposit
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
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool, signer_account_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer_account_id.parse().unwrap())
            .is_view(is_view)
            .attached_deposit(810_000_000_000_000_000_000) // required for register_node()
            .build()
    }

    #[test]
    fn test_register_and_get_node() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check owner, ip_address, and port
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
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
    fn test_remove_node() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check the ip address after registering
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!(Some("0.0.0.0".to_string()), contract.get_node_ip_address(U64(1)));

        // remove the node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        contract.remove_node(U64(1));

        // check the ip address after removing
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!(None, contract.get_node_ip_address(U64(1)));
    }

    #[test]
    #[should_panic(expected = "Only bob_near can call this method")]
    fn test_remove_node_wrong_owner() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check the ip address after registering
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!(Some("0.0.0.0".to_string()), contract.get_node_ip_address(U64(1)));

        // try removing the node
        let context = get_context(false, "alice_near".to_string());
        testing_env!(context);
        contract.remove_node(U64(1));
    }

    #[test]
    fn test_set_node_ip_address_and_port() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // update the ip address and port
        contract.set_node_ip_address(U64(1), "1.1.1.1".to_string());
        contract.set_node_port(U64(1), U64(8081));

        // check the ip address and port after updating
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!("1.1.1.1".to_string(), contract.get_node_ip_address(U64(1)).unwrap());
        assert_eq!(U64(8081), contract.get_node_port(U64(1)).unwrap());
        assert_eq!(
            get_logs(),
            vec!["get_node_ip_address for node_id 1", "get_node_port for node_id 1"]
        );
    }

    #[test]
    fn test_new_owner() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // set pending owner
        contract.set_node_pending_owner(U64(1), "alice_near".to_string());

        // check pending owner
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node_pending_owner(U64(1)).unwrap().to_string()
        );

        // accept ownership
        let context = get_context(false, "alice_near".to_string());
        testing_env!(context);
        contract.become_node_owner(U64(1));

        // check owner
        let context = get_context(true, "alice_near".to_string());
        testing_env!(context);
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node_owner(U64(1)).unwrap().to_string()
        );
    }

    #[test]
    #[should_panic(expected = "Only alice_near can call this method")]
    fn test_wrong_owner() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // set pending owner
        contract.set_node_pending_owner(U64(1), "alice_near".to_string());

        // check pending owner
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node_pending_owner(U64(1)).unwrap().to_string()
        );

        // accept ownership from wrong owner
        let context = get_context(false, "franklin_near".to_string());
        testing_env!(context);
        contract.become_node_owner(U64(1));
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        let contract = MainchainContract::new();
        assert_eq!(None, contract.get_node_ip_address(U64(1)));
        assert_eq!(get_logs(), vec!["get_node_ip_address for node_id 1"])
    }
}
