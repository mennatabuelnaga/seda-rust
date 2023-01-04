use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, PromiseError, PromiseOrValue};

use crate::{StakingContract, StakingContractExt};
use crate::fungible_token::GAS_FOR_FT_ON_TRANSFER;
use crate::staking::RewardFeeFraction;

/// `internal_ping` fetches this contract's balance and the SEDA epoch. This
/// enum is used to complete the cross-contract call chain
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PingAction {
    Ping,
    Deposit,
    DepositAndStake,
    Withdraw(AccountId, U128),
    WithdrawAll(AccountId),
    Stake(AccountId, U128),
    StakeAll(AccountId),
    Unstake(AccountId, U128),
    UnstakeAll(AccountId),
    UpdateRewardFeeFraction(RewardFeeFraction),
}

/// Contract public methods
#[near_bindgen]
impl StakingContract {
    #[private]
    pub fn ping_callback(
        &mut self,
        #[callback_result] get_epoch_result: Result<u64, PromiseError>,
        #[callback_result] staking_pool_balance_result: Result<U128, PromiseError>,
        action: PingAction,
    ) -> PromiseOrValue<U128> {
        if get_epoch_result.is_err() || staking_pool_balance_result.is_err() {
            env::log_str("ping failed");
            return PromiseOrValue::Value(U128(0));
        }

        // assert the epoch has increased
        let epoch_height = get_epoch_result.unwrap() as u64;
        if self.last_epoch_height == epoch_height {
            return PromiseOrValue::Value(U128(0)); // TODO: dont return 0, just skip the rest of the function
        }
        self.last_epoch_height = epoch_height;

        let total_balance = staking_pool_balance_result.unwrap();

        assert!(
            total_balance >= self.last_total_balance.into(),
            "The new total balance should not be less than the old total balance"
        );
        let total_reward = total_balance.0 - self.last_total_balance;
        if total_reward > 0 {
            // The validation fee that the contract owner takes.
            let owners_fee = self.reward_fee_fraction.multiply(total_reward);

            // Distributing the remaining reward to the delegators first.
            let remaining_reward = total_reward - owners_fee;
            self.total_staked_balance += remaining_reward;

            // Now buying "stake" shares for the contract owner at the new share price.
            let num_shares = self.num_shares_from_staked_amount_rounded_down(owners_fee);
            if num_shares > 0 {
                // Updating owner's inner account
                let owner_id = self.owner_id.clone();
                let mut account = self.internal_get_account(&owner_id);
                account.stake_shares += num_shares;
                self.internal_save_account(&owner_id, &account);
                // Increasing the total amount of "stake" shares.
                self.total_stake_shares += num_shares;
            }
            // Increasing the total staked balance by the owners fee, no matter whether the
            // owner received any shares or not.
            self.total_staked_balance += owners_fee;

            env::log_str(
                format!(
                    "Epoch {}: Contract received total rewards of {} tokens. New total staked balance is {}. Total number of shares {}",
                    epoch_height, total_reward, self.total_staked_balance, self.total_stake_shares,
                )
                    .as_str(),
            );
            if num_shares > 0 {
                env::log_str(format!("Total rewards fee is {} stake shares.", num_shares).as_str());
            }
        }

        self.last_total_balance = total_balance.into();

        match action {
            PingAction::Ping => return PromiseOrValue::Value(U128(1)), // nothing to do
            PingAction::WithdrawAll(account_id) => {
                PromiseOrValue::Promise(Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                    .proceed_withdraw_all(account_id))
            }
            PingAction::Withdraw(account_id, amount) => {
                PromiseOrValue::Promise(Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                    .proceed_withdraw(account_id, amount))
            }
            PingAction::StakeAll(account_id) => {
                PromiseOrValue::Promise(Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                    .proceed_stake_all(account_id))
            }
            PingAction::Stake(account_id, amount) => {
                PromiseOrValue::Promise(Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                    .proceed_stake(account_id, amount))
            }
            PingAction::UnstakeAll(account_id) => {
                PromiseOrValue::Promise(Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                    .proceed_unstake_all(account_id))
            }
            PingAction::Unstake(account_id, amount) => {
                PromiseOrValue::Promise(Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_FT_ON_TRANSFER) // TODO
                    .proceed_unstake(account_id, amount))
            }
            _ => unreachable!(),
        }
    }

    #[private]
    pub fn withdraw_callback(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) {
        if call_result.is_err() {
            env::log_str("withdraw failed");
            return;
        }

        let mut account = self.internal_get_account(&account_id);
        account.unstaked -= amount.0;
        self.internal_save_account(&account_id, &account);

        env::log_str(
            format!(
                "@{} withdrawing {}. New unstaked balance is {}",
                account_id, amount.0, account.unstaked
            )
            .as_str(),
        );

        self.last_total_balance -= amount.0;
    }
}
