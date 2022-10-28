#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{
        json_types::U64,
        test_utils::{get_logs, VMContextBuilder},
        testing_env,
        VMContext, env::account_balance,
    };

    use crate::mainchain::MainchainContract;

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
            .attached_deposit(780_000_000_000_000_000_000) // required for register_node()
            .build()
    }

    #[test]
    fn register_and_get_node() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        let node_id = contract.register_node("0.0.0.0:8080".to_string()).unwrap();
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);
        assert_eq!("1".to_string(), node_id);
        // check owner and socket address
        testing_env!(get_context_view());
        assert_eq!(
            "bob_near".to_string(),
            contract.get_node_owner(U64(1)).unwrap().to_string()
        );
        assert_eq!(
            "0.0.0.0:8080".to_string(),
            contract.get_node_socket_address(U64(1)).unwrap()
        );
        assert_eq!(
            get_logs(),
            vec!["get_node_owner for node_id 1", "get_node_socket_address for node_id 1",]
        );
    }

    #[test]
    #[should_panic(expected = "Insufficient storage, need 780000000000000000000")]
    fn register_not_enough_storage() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
    }

    #[test]
    fn remove_node() {
        let mut contract = MainchainContract::new();
        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());

        // check the socket address after registering
        testing_env!(get_context_view());
        assert_eq!(
            Some("0.0.0.0:8080".to_string()),
            contract.get_node_socket_address(U64(1))
        );

        // remove the node
        testing_env!(get_context("bob_near".to_string()));
        let balance_before = account_balance();
        contract.remove_node(U64(1));
        assert_eq!(
            get_logs(),
            vec![
                "bob_near removed node_id 1",
                "Refunding 780000000000000000000 for storage deposit to bob_near"
            ]
        );
        let balance_after = account_balance();
        assert_eq!(balance_before - balance_after, 780000000000000000000);
        // check the socket address after removing
        testing_env!(get_context_view());
        assert_eq!(None, contract.get_node_socket_address(U64(1)));
    }

    #[test]
    #[should_panic(expected = "Only bob_near can call this method")]
    fn remove_node_wrong_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check the socket address after registering
        testing_env!(get_context_view());
        assert_eq!(
            Some("0.0.0.0:8080".to_string()),
            contract.get_node_socket_address(U64(1))
        );

        // try removing the node
        testing_env!(get_context("alice_near".to_string()));
        contract.remove_node(U64(1));
    }

    #[test]
    fn set_node_socket_address() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // update the socket address
        contract.set_node_socket_address(U64(1), "1.1.1.1:8081".to_string());

        // check the socket address after updating
        testing_env!(get_context_view());
        assert_eq!(
            "1.1.1.1:8081".to_string(),
            contract.get_node_socket_address(U64(1)).unwrap()
        );
        assert_eq!(get_logs(), vec!["get_node_socket_address for node_id 1"]);
    }

    #[test]
    fn new_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
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
    fn wrong_owner_simple() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // accept ownership from wrong owner
        testing_env!(get_context("alice_near".to_string()));
        contract.become_node_owner(U64(1));
    }

    #[test]
    #[should_panic(expected = "Only alice_near can call this method")]
    fn wrong_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
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
        assert_eq!(None, contract.get_node_socket_address(U64(1)));
        assert_eq!(get_logs(), vec!["get_node_socket_address for node_id 1"])
    }
}
