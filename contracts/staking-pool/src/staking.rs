use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    ext_contract,
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
    Promise,
    PromiseOrValue,
    PublicKey,
    PromiseError
};

use crate::{callbacks::PingAction, StakingContract, StakingContractExt, U256};

/// Represents an account structure readable by humans.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableAccount {
    pub account_id:       AccountId,
    /// The unstaked balance that can be withdrawn or staked.
    pub unstaked_balance: U128,
    /// The amount balance staked at the current "stake" share price.
    pub staked_balance:   U128,
    /// Whether the unstaked balance is available for withdrawal now.
    pub can_withdraw:     bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardFeeFraction {
    pub numerator:   u32,
    pub denominator: u32,
}

impl RewardFeeFraction {
    pub fn assert_valid(&self) {
        assert_ne!(self.denominator, 0, "Denominator must be a positive number");
        assert!(
            self.numerator <= self.denominator,
            "The reward fee must be less or equal to 1"
        );
    }

    pub fn multiply(&self, value: Balance) -> Balance {
        (U256::from(self.numerator) * U256::from(value) / U256::from(self.denominator)).as_u128()
    }
}

/// Interface for a voting contract.
#[ext_contract(ext_voting)]
pub trait VoteContract {
    /// Method for validators to vote or withdraw the vote.
    /// Votes for if `is_vote` is true, or withdraws the vote if `is_vote` is
    /// false.
    fn vote(&mut self, is_vote: bool);
}

/// Contract public methods
#[near_bindgen]
impl StakingContract {
    /// Distributes rewards and restakes if needed.
    pub fn ping(&mut self) {
        self.internal_ping(PingAction::Ping);
    }

    /// Deposits the attached amount into the inner account of the predecessor.
    pub fn deposit(&mut self, amount: U128, account_id: AccountId) -> PromiseOrValue<U128> {
        // TODO: only callable by this contract

        self.internal_ping(PingAction::Deposit(amount.into(), account_id.clone()));
    }

    #[private]
    pub fn proceed_deposit(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) -> PromiseOrValue<U128> {
        if call_result.is_err() {
            env::log_str("ping failed");
            return PromiseOrValue::Value(amount.into()); // return full amount TODO: test
        }
        self.internal_deposit(amount.into(), account_id);
        PromiseOrValue::Value(U128::from(0)) // no refund
    }

    /// Deposits the attached amount into the inner account of the predecessor
    /// and stakes it.
    pub fn deposit_and_stake(&mut self, amount: U128, account_id: AccountId) -> PromiseOrValue<U128> {
        // TODO: only callable by this contract

        self.internal_ping();

        let amount = self.internal_deposit(amount.into(), account_id.clone());
        self.internal_stake(amount, account_id);

        PromiseOrValue::Value(U128::from(0)) // no refund
    }

    /// Withdraws the entire unstaked balance from the predecessor account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw_all(&mut self) {
        self.internal_ping(PingAction::WithdrawAll(env::predecessor_account_id()));
    }

    /// Proceeds with `withdraw_all` call after ping (epoch and balance are
    /// successfully fetched)
    #[private]
    pub fn proceed_withdraw_all(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        let account = self.internal_get_account(&account_id);
        self.internal_withdraw(account.unstaked, account_id);
    }

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    #[payable]
    pub fn withdraw(&mut self, amount: U128) {
        self.internal_ping(PingAction::Withdraw(env::predecessor_account_id(), amount));
    }

    /// Proceeds with `withdraw` call after ping (epoch and balance are
    /// successfully fetched)
    #[private]
    pub fn proceed_withdraw(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        let amount: Balance = amount.into();
        self.internal_withdraw(amount, account_id);
    }

    /// Stakes all available unstaked balance from the inner account of the
    /// predecessor.
    pub fn stake_all(&mut self) {
        // Stake action always restakes
        self.internal_ping(PingAction::StakeAll(env::predecessor_account_id()));
    }

    /// Proceeds with `stake_all` call after ping (epoch and balance are
    /// successfully fetched)
    #[private]
    pub fn proceed_stake_all(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        let account = self.internal_get_account(&account_id);
        self.internal_stake(account.unstaked, account_id);
    }

    /// Stakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough unstaked balance.
    pub fn stake(&mut self, amount: U128) {
        // Stake action always restakes
        self.internal_ping(PingAction::Stake(env::predecessor_account_id(), amount));
    }

    /// Proceeds with `stake` call after ping (epoch and balance are
    /// successfully fetched)
    #[private]
    pub fn proceed_stake(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        let amount: Balance = amount.into();
        self.internal_stake(amount, account_id);
    }

    /// Unstakes all staked balance from the inner account of the predecessor.
    /// The new total unstaked balance will be available for withdrawal in four
    /// epochs.
    pub fn unstake_all(&mut self) {
        // Unstake action always restakes
        self.internal_ping(PingAction::UnstakeAll(env::predecessor_account_id()));
    }

    /// Proceeds with `unstake_all` call after ping (epoch and balance are
    /// successfully fetched)
    #[private]
    pub fn proceed_unstake_all(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        let account = self.internal_get_account(&account_id);
        let amount = self.staked_amount_from_num_shares_rounded_down(account.stake_shares);
        self.inner_unstake(amount);
    }

    /// Unstakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough staked balance.
    /// The new total unstaked balance will be available for withdrawal in four
    /// epochs.
    pub fn unstake(&mut self, amount: U128) {
        // Unstake action always restakes
        self.internal_ping(PingAction::Unstake(env::predecessor_account_id(), amount));
    }

    /// Proceeds with `stake` call after ping (epoch and balance are
    /// successfully fetched)
    #[private]
    pub fn proceed_unstake(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        account_id: AccountId,
        amount: U128,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        let amount: Balance = amount.into();
        self.inner_unstake(amount);
    }

    /*************** */
    /* View methods */
    /*************** */

    /// Returns the unstaked balance of the given account.
    pub fn get_account_unstaked_balance(&self, account_id: AccountId) -> U128 {
        self.get_account(account_id).unstaked_balance
    }

    /// Returns the staked balance of the given account.
    /// NOTE: This is computed from the amount of "stake" shares the given
    /// account has and the current amount of total staked balance and total
    /// stake shares on the account.
    pub fn get_account_staked_balance(&self, account_id: AccountId) -> U128 {
        self.get_account(account_id).staked_balance
    }

    /// Returns the total balance of the given account (including staked and
    /// unstaked balances).
    pub fn get_account_total_balance(&self, account_id: AccountId) -> U128 {
        let account = self.get_account(account_id);
        (account.unstaked_balance.0 + account.staked_balance.0).into()
    }

    /// Returns `true` if the given account can withdraw tokens in the current
    /// epoch.
    pub fn is_account_unstaked_balance_available(&self, account_id: AccountId) -> bool {
        self.get_account(account_id).can_withdraw
    }

    /// Returns the total staking balance.
    pub fn get_total_staked_balance(&self) -> U128 {
        self.total_staked_balance.into()
    }

    /// Returns account ID of the staking pool owner.
    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// Returns the current reward fee as a fraction.
    pub fn get_reward_fee_fraction(&self) -> RewardFeeFraction {
        self.reward_fee_fraction.clone()
    }

    /// Returns the staking public key
    pub fn get_staking_key(&self) -> PublicKey {
        self.stake_public_key.clone()
    }

    /// Returns true if the staking is paused
    pub fn is_staking_paused(&self) -> bool {
        self.paused
    }

    /// Returns human readable representation of the account for the given
    /// account ID.
    pub fn get_account(&self, account_id: AccountId) -> HumanReadableAccount {
        let account = self.internal_get_account(&account_id);
        HumanReadableAccount {
            account_id,
            unstaked_balance: account.unstaked.into(),
            staked_balance: self
                .staked_amount_from_num_shares_rounded_down(account.stake_shares)
                .into(),
            can_withdraw: account.unstaked_available_epoch_height <= env::epoch_height(),
        }
    }

    /// Returns the number of accounts that have positive balance on this
    /// staking pool.
    pub fn get_number_of_accounts(&self) -> u64 {
        self.accounts.len()
    }

    /// Returns the list of accounts
    pub fn get_accounts(&self, from_index: u64, limit: u64) -> Vec<HumanReadableAccount> {
        let keys = self.accounts.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| self.get_account(keys.get(index).unwrap()))
            .collect()
    }

    /****************** */
    /* Owner's methods */
    /****************** */

    /// Owner's method.
    /// Updates current public key to the new given public key.
    pub fn update_staking_key(&mut self, stake_public_key: PublicKey) {
        self.assert_owner();
        self.stake_public_key = stake_public_key;
    }

    /// Owner's method.
    /// Updates current reward fee fraction to the new given fraction.
    pub fn update_reward_fee_fraction(&mut self, reward_fee_fraction: RewardFeeFraction) {
        self.assert_owner();
        reward_fee_fraction.assert_valid();

        self.internal_ping(PingAction::UpdateRewardFeeFraction(reward_fee_fraction));
    }

    pub fn proceed_update_reward_fee_fraction(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        reward_fee_fraction: RewardFeeFraction,
    ) {
        if call_result.is_err() {
            env::log_str("ping failed");
            return;
        }

        self.reward_fee_fraction = reward_fee_fraction;
    }

    /// Owner's method.
    /// Calls `vote(is_vote)` on the given voting contract account ID on behalf
    /// of the pool.
    pub fn vote(&mut self, voting_account_id: AccountId, is_vote: bool) -> Promise {
        self.assert_owner();
        assert!(
            env::is_valid_account_id(voting_account_id.as_bytes()),
            "Invalid voting account ID"
        );

        ext_voting::ext(voting_account_id).vote(is_vote)
    }

    /// Owner's method.
    /// Pauses pool staking.
    pub fn pause_staking(&mut self) {
        self.assert_owner();
        assert!(!self.paused, "The staking is already paused");
        self.paused = true;
    }

    /// Owner's method.
    /// Resumes pool staking.
    pub fn resume_staking(&mut self) {
        self.assert_owner();
        assert!(self.paused, "The staking is not paused");
        self.paused = false;
    }
}
