use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, PromiseError, PromiseOrValue};

use crate::{consts::{GAS_FOR_FT_ON_TRANSFER, MINIMUM_STAKE, EPOCH_DELAY_FOR_ELECTION}, fungible_token::ft, MainchainContract, MainchainContractExt, node_registry::Node};

/// Contract private methods
impl MainchainContract {
    pub(crate) fn is_eligible_for_current_epoch(&self, node: &Node) -> bool {
        node.epoch_when_eligible > 0 && node.epoch_when_eligible <= env::epoch_height()
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

        env::log_str(
            format!(
                "@{} deposited {}. New balance is {}",
                account_id, amount, node.balance
            )
            .as_str(),
        );

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
}
