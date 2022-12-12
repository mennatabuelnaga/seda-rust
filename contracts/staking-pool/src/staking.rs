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
    PromiseResult,
    PublicKey,
};

use crate::{StakingContract, StakingContractExt, U256};

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

/// Interface for the contract itself.
#[ext_contract(ext_self)]
pub trait SelfContract {
    /// A callback to check the result of the staking action.
    /// In case the stake amount is less than the minimum staking threshold, the
    /// staking action fails, and the stake amount is not changed. This
    /// might lead to inconsistent state and the follow withdraw calls might
    /// fail. To mitigate this, the contract will issue a new unstaking
    /// action in case of the failure of the first staking action.
    fn on_stake_action(&mut self);
}

/// Contract public methods
#[near_bindgen]
impl StakingContract {
    /// Distributes rewards and restakes if needed.
    pub fn ping(&mut self) {
        if self.internal_ping() {
            self.internal_restake();
        }
    }

    /// Deposits the attached amount into the inner account of the predecessor.
    pub fn deposit(&mut self, amount: U128, account_id: AccountId) {
        // TODO: only callable by seda_token

        let need_to_restake = self.internal_ping();

        self.internal_deposit(amount.into(), account_id);

        if need_to_restake {
            self.internal_restake();
        }
    }

    /// Deposits the attached amount into the inner account of the predecessor
    /// and stakes it.
    #[payable]
    pub fn deposit_and_stake(&mut self, amount: U128, account_id: AccountId) {
        // TODO: only callable by seda_token

        self.internal_ping();

        let amount = self.internal_deposit(amount.into(), account_id);
        self.internal_stake(amount);

        self.internal_restake();
    }

    /// Withdraws the entire unstaked balance from the predecessor account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw_all(&mut self) {
        let need_to_restake = self.internal_ping();

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        self.internal_withdraw(account.unstaked);

        if need_to_restake {
            self.internal_restake();
        }
    }

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw(&mut self, amount: U128) {
        let need_to_restake = self.internal_ping();

        let amount: Balance = amount.into();
        self.internal_withdraw(amount);

        if need_to_restake {
            self.internal_restake();
        }
    }

    /// Stakes all available unstaked balance from the inner account of the
    /// predecessor.
    pub fn stake_all(&mut self) {
        // Stake action always restakes
        self.internal_ping();

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        self.internal_stake(account.unstaked);

        self.internal_restake();
    }

    /// Stakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough unstaked balance.
    pub fn stake(&mut self, amount: U128) {
        // Stake action always restakes
        self.internal_ping();

        let amount: Balance = amount.into();
        self.internal_stake(amount);

        self.internal_restake();
    }

    /// Unstakes all staked balance from the inner account of the predecessor.
    /// The new total unstaked balance will be available for withdrawal in four
    /// epochs.
    pub fn unstake_all(&mut self) {
        // Unstake action always restakes
        self.internal_ping();

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        let amount = self.staked_amount_from_num_shares_rounded_down(account.stake_shares);
        self.inner_unstake(amount);

        self.internal_restake();
    }

    /// Unstakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough staked balance.
    /// The new total unstaked balance will be available for withdrawal in four
    /// epochs.
    pub fn unstake(&mut self, amount: U128) {
        // Unstake action always restakes
        self.internal_ping();

        let amount: Balance = amount.into();
        self.inner_unstake(amount);

        self.internal_restake();
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

    /************ */
    /* Callbacks */
    /************ */

    pub fn on_stake_action(&mut self) {
        assert_eq!(
            env::current_account_id(),
            env::predecessor_account_id(),
            "Can be called only as a callback"
        );

        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );
        let stake_action_succeeded = matches!(env::promise_result(0), PromiseResult::Successful(_));

        // If the stake action failed and the current locked amount is positive, then
        // the contract has to unstake.
        if !stake_action_succeeded && env::account_locked_balance() > 0 {
            Promise::new(env::current_account_id()).stake(0, self.stake_public_key.clone());
        }
    }

    /****************** */
    /* Owner's methods */
    /****************** */

    /// Owner's method.
    /// Updates current public key to the new given public key.
    pub fn update_staking_key(&mut self, stake_public_key: PublicKey) {
        self.assert_owner();
        // When updating the staking key, the contract has to restake.
        let _need_to_restake = self.internal_ping();
        self.stake_public_key = stake_public_key;
        self.internal_restake();
    }

    /// Owner's method.
    /// Updates current reward fee fraction to the new given fraction.
    pub fn update_reward_fee_fraction(&mut self, reward_fee_fraction: RewardFeeFraction) {
        self.assert_owner();
        reward_fee_fraction.assert_valid();

        let need_to_restake = self.internal_ping();
        self.reward_fee_fraction = reward_fee_fraction;
        if need_to_restake {
            self.internal_restake();
        }
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

        self.internal_ping();
        self.paused = true;
        Promise::new(env::current_account_id()).stake(0, self.stake_public_key.clone());
    }

    /// Owner's method.
    /// Resumes pool staking.
    pub fn resume_staking(&mut self) {
        self.assert_owner();
        assert!(self.paused, "The staking is not paused");

        self.internal_ping();
        self.paused = false;
        self.internal_restake();
    }
}
