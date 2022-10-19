use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U64;
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey};

/// LookupMap keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    NumNodes,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Node {
    pub account_id: AccountId,
    pub ip_address: String,
    pub port: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    num_nodes: u64,
    nodes: LookupMap<u64, Node>,
}

#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new() -> Self {
        Self {
            num_nodes: 0,
            nodes: LookupMap::new(MainchainStorageKeys::NumNodes),
        }
    }

    #[payable]
    pub fn register_node(&mut self, ip_address: String, port: U64) {
        // keep track of storage usage
        let initial_storage_usage = env::storage_usage();

        self.num_nodes += 1;
        let node_id = self.num_nodes;
        let account_id = env::signer_account_id();

        log!("{} registered node_id {}", account_id, node_id);
        let node = Node {
            account_id,
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

    pub fn update_node_ip_address(&mut self, node_id: U64, new_ip_address: String) {
        let account_id = env::signer_account_id();
        let mut node = self.nodes.get(&node_id.into()).expect("node not found");

        assert_eq!(
            node.account_id, account_id,
            "only associated `account_id` can update node"
        );

        log!(
            "{} updated node with id {} ip address to {}",
            account_id,
            u64::from(node_id),
            new_ip_address
        );
        node.ip_address = new_ip_address;
        self.nodes.insert(&node_id.into(), &node);
    }

    pub fn update_node_port(&mut self, node_id: U64, new_port: U64) {
        let account_id = env::signer_account_id();
        let mut node = self.nodes.get(&node_id.into()).expect("node not found");

        assert_eq!(
            node.account_id, account_id,
            "only associated `account_id` can update node"
        );

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
        let node = self.nodes.get(&node_id.into()).expect("node not found");

        assert_eq!(
            node.account_id, account_id,
            "only associated `account_id` can remove node"
        );

        log!("{} removed node with id {}", account_id, u64::from(node_id));
        self.nodes.remove(&node_id.into());
    }

    pub fn get_node_account_id(&self, node_id: U64) -> Option<AccountId> {
        log!("get_node_account_id for node_id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(node.account_id),
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
            .attached_deposit(800_000_000_000_000_000_000) // required for register_node()
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

        // check account_id, ip_address, and port
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        assert_eq!(
            "bob_near".to_string(),
            contract.get_node_account_id(U64(1)).unwrap().to_string()
        );
        assert_eq!("0.0.0.0".to_string(), contract.get_node_ip_address(U64(1)).unwrap());
        assert_eq!(U64(8080), contract.get_node_port(U64(1)).unwrap());
        assert_eq!(
            get_logs(),
            vec![
                "get_node_account_id for node_id 1",
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
    #[should_panic(expected = "only associated `account_id` can remove node")]
    fn test_remove_node_wrong_account_id() {
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
    fn test_update_node_ip_address_and_port() {
        // register node
        let context = get_context(false, "bob_near".to_string());
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // update the ip address and port
        contract.update_node_ip_address(U64(1), "1.1.1.1".to_string());
        contract.update_node_port(U64(1), U64(8081));

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
    fn get_nonexistent_message() {
        let context = get_context(true, "bob_near".to_string());
        testing_env!(context);
        let contract = MainchainContract::new();
        assert_eq!(None, contract.get_node_ip_address(U64(1)));
        assert_eq!(get_logs(), vec!["get_node_ip_address for node_id 1"])
    }
}
