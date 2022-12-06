#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{
        env::account_balance,
        json_types::U64,
        test_utils::{get_logs, VMContextBuilder},
        testing_env,
        VMContext,
    };

    use crate::{
        node_registry::{Node, UpdateNode},
        MainchainContract,
    };

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
            contract.get_node(U64(1)).unwrap().owner.to_string()
        );
        assert_eq!(
            "0.0.0.0:8080".to_string(),
            contract.get_node(U64(1)).unwrap().socket_address
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
    fn unregister_node() {
        let mut contract = MainchainContract::new();
        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());

        // check the socket address after registering
        testing_env!(get_context_view());
        assert_eq!(
            "0.0.0.0:8080".to_string(),
            contract.get_node(U64(1)).unwrap().socket_address
        );

        // remove the node
        testing_env!(get_context("bob_near".to_string()));
        let balance_before = account_balance();
        contract.unregister_node(U64(1));
        assert_eq!(
            get_logs(),
            vec![
                "bob_near removed node_id 1",
                "Refunding 780000000000000000000 for storage deposit to bob_near"
            ]
        );
        let balance_after = account_balance();
        assert_eq!(balance_before - balance_after, 780000000000000000000);
    }

    #[test]
    #[should_panic(expected = "Only bob_near can call this method")]
    fn unregister_node_wrong_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // check the socket address after registering
        testing_env!(get_context_view());
        assert_eq!(
            "0.0.0.0:8080".to_string(),
            contract.get_node(U64(1)).unwrap().socket_address
        );

        // try removing the node
        testing_env!(get_context("alice_near".to_string()));
        contract.unregister_node(U64(1));
    }

    #[test]
    fn set_node_socket_address() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // update the socket address
        contract.update_node(U64(1), UpdateNode::SetSocketAddress("1.1.1.1:8081".to_string()));

        // check the socket address after updating
        testing_env!(get_context_view());
        assert_eq!(
            "1.1.1.1:8081".to_string(),
            contract.get_node(U64(1)).unwrap().socket_address
        );
    }

    #[test]
    fn new_owner() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        assert_eq!(get_logs(), vec!["bob_near registered node_id 1"]);

        // set pending owner
        contract.update_node(U64(1), UpdateNode::SetPendingOwner("alice_near".to_string()));

        // check pending owner
        testing_env!(get_context_view());
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node(U64(1)).unwrap().pending_owner.unwrap().to_string()
        );

        // accept ownership
        testing_env!(get_context("alice_near".to_string()));
        contract.update_node(U64(1), UpdateNode::AcceptOwnership);

        // check owner
        testing_env!(get_context_view());
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node(U64(1)).unwrap().owner.to_string()
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
        contract.update_node(U64(1), UpdateNode::AcceptOwnership);
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
        contract.update_node(U64(1), UpdateNode::SetPendingOwner("alice_near".to_string()));

        // check pending owner
        testing_env!(get_context_view());
        assert_eq!(
            "alice_near".to_string(),
            contract.get_node(U64(1)).unwrap().pending_owner.unwrap().to_string()
        );

        // accept ownership from wrong owner
        testing_env!(get_context("franklin_near".to_string()));
        contract.update_node(U64(1), UpdateNode::AcceptOwnership);
    }

    #[test]
    #[should_panic(expected = "Invalid socket address")]
    fn register_invalid_characters() {
        let mut contract = MainchainContract::new();

        // register node
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0[".to_string()).unwrap();
    }

    #[test]
    fn get_nodes() {
        let mut contract = MainchainContract::new();

        // register three nodes
        testing_env!(get_context_with_deposit("bob_near".to_string()));
        contract.register_node("0.0.0.0:8080".to_string());
        contract.register_node("1.1.1.1:8080".to_string());
        contract.register_node("2.2.2.2:8080".to_string());
        assert_eq!(
            get_logs(),
            vec![
                "bob_near registered node_id 1",
                "bob_near registered node_id 2",
                "bob_near registered node_id 3",
            ]
        );

        // define expected nodes
        let node1 = Node {
            owner:          "bob_near".parse().unwrap(),
            pending_owner:  None,
            socket_address: "0.0.0.0:8080".to_string(),
        };
        let node2 = Node {
            owner:          "bob_near".parse().unwrap(),
            pending_owner:  None,
            socket_address: "1.1.1.1:8080".to_string(),
        };
        let node3 = Node {
            owner:          "bob_near".parse().unwrap(),
            pending_owner:  None,
            socket_address: "2.2.2.2:8080".to_string(),
        };

        // get the first node
        testing_env!(get_context_view());
        let get_node = contract.get_node(U64(1));
        assert_eq!(get_node.unwrap(), node1);

        // check the latest 2 nodes
        let latest_2_nodes = contract.get_nodes(U64(2), U64(0));
        assert_eq!(latest_2_nodes, vec![node3.clone(), node2.clone()]);

        // check the latest 3 nodes
        let latest_3_nodes = contract.get_nodes(U64(100), U64(0));
        assert_eq!(latest_3_nodes, vec![node3.clone(), node2, node1.clone()]);

        // bob deletes the second node
        testing_env!(get_context("bob_near".to_string()));
        contract.unregister_node(U64(2));

        // check the latest nodes
        testing_env!(get_context_view());
        let latest_nodes = contract.get_nodes(U64(100), U64(0));
        assert_eq!(latest_nodes, vec![node3, node1.clone()]);

        // check offset of 1
        let latest_nodes_offset = contract.get_nodes(U64(100), U64(1));
        assert_eq!(latest_nodes_offset, vec![node1]);
    }
}
