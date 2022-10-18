use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U64;

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
        self.num_nodes += 1;
        let node_id = self.num_nodes;
        let account_id = env::signer_account_id();
        
        log!("{} registered node with id {}", account_id, node_id);
        let node = Node {
            account_id,
            ip_address,
            port: port.into(),
        };
        self.nodes.insert(&node_id, &node);
    }

    pub fn update_node_ip_address(&mut self, node_id: U64, new_ip_address: String) {
        let account_id = env::signer_account_id();
        let mut node = self.nodes.get(&node_id.into()).expect("node not found");

        assert_eq!(node.account_id, account_id, "only associated `account_id` can update node");

        log!("{} updated node with id {} ip address to {}", account_id, u64::from(node_id), new_ip_address);
        node.ip_address = new_ip_address;
        self.nodes.insert(&node_id.into(), &node);
    }

    pub fn update_node_port(&mut self, node_id: U64, new_port: U64) {
        let account_id = env::signer_account_id();
        let mut node = self.nodes.get(&node_id.into()).expect("node not found");

        assert_eq!(node.account_id, account_id, "only associated `account_id` can update node");

        log!("{} updated node with id {} port to {}", account_id, u64::from(node_id), u64::from(new_port));
        node.port = new_port.into();
        self.nodes.insert(&node_id.into(), &node);
    }

    // pub fn remove_node(&mut self, node_id: U64) {}

    pub fn get_node_account_id(&self, node_id: U64) -> Option<AccountId> {
        log!("get_node_account_id for id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(node.account_id),
            None => None,
        }
    }

    pub fn get_node_ip_address(&self, node_id: U64) -> Option<String> {
        log!("get_node_ip_address for id {}", u64::from(node_id));
        match self.nodes.get(&node_id.into()) {
            Some(node) => Some(node.ip_address),
            None => None,
        }
    }

    pub fn get_node_port(&self, node_id: U64) -> Option<U64> {
        log!("get_node_port for id {}", u64::from(node_id));
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

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn test_register_get_node() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node with id 1"]);

        let context = get_context(true);
        testing_env!(context);
        assert_eq!("bob_near".to_string(), contract.get_node_account_id(U64(1)).unwrap().to_string());
        assert_eq!("0.0.0.0".to_string(), contract.get_node_ip_address(U64(1)).unwrap());
        assert_eq!(U64(8080), contract.get_node_port(U64(1)).unwrap());

        assert_eq!(get_logs(), vec![
            "get_node_account_id for id 1",
            "get_node_ip_address for id 1",
            "get_node_port for id 1"
        ]);
    }

    #[test]
    fn test_update_node_ip_address_port() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = MainchainContract::new();
        contract.register_node("0.0.0.0".to_string(), U64(8080));
        assert_eq!(get_logs(), vec!["bob_near registered node with id 1"]);

        contract.update_node_ip_address(U64(1), "1.1.1.1".to_string());
        contract.update_node_port(U64(1), U64(8081));

        let context = get_context(true);
        testing_env!(context);
        assert_eq!("1.1.1.1".to_string(), contract.get_node_ip_address(U64(1)).unwrap());
        assert_eq!(U64(8081), contract.get_node_port(U64(1)).unwrap());
    }

    // #[test]
    // fn get_nonexistent_message() {
    //     let context = get_context(true);
    //     testing_env!(context);
    //     let contract = MainchainContract::default();
    //     assert_eq!(None, contract.get_status("francis.near".parse().unwrap()));
    //     assert_eq!(get_logs(), vec!["get_status for account_id francis.near"])
    // }
}
