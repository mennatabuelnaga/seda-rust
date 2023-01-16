use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{env, ext_contract, json_types::U128, log, near_bindgen, require, AccountId, PromiseOrValue};

use crate::{consts::GAS_FOR_FT_ON_TRANSFER, MainchainContract, MainchainContractExt};

#[ext_contract(ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

/// Public contract methods
#[near_bindgen]
impl FungibleTokenReceiver for MainchainContract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        // Verifying that we were called by fungible token contract that we expect.
        require!(
            env::predecessor_account_id() == self.seda_token,
            "Unsupported fungible token contract"
        );
        log!(
            "in {} tokens from @{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );
        let account_id = env::signer_account_id();
        let prepaid_gas = env::prepaid_gas();
        match msg.as_str() {
            "deposit" => Self::ext(env::current_account_id())
                .with_static_gas(prepaid_gas - GAS_FOR_FT_ON_TRANSFER)
                .deposit(amount.0, account_id)
                .into(),
            "deposit-and-stake" => Self::ext(env::current_account_id())
                .with_static_gas(prepaid_gas - GAS_FOR_FT_ON_TRANSFER)
                .deposit_and_stake(amount.0, account_id)
                .into(),
            _ => {
                panic!("Unexpected message");
            }
        }
    }
}
