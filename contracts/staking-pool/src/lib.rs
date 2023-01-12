use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env,
    near_bindgen,
    AccountId,
    Balance,
    EpochHeight,
    Promise,
    PublicKey,
};
use uint::construct_uint;

use crate::staking::RewardFeeFraction;

mod internal;
pub mod staking;
mod staking_test;
mod token_receiver;

/// The amount of gas given to complete `vote` call.
const _VOTE_GAS: u64 = 100_000_000_000_000;

/// The amount of gas given to complete internal `on_stake_action` call.
const _ON_STAKE_ACTION_GAS: u64 = 20_000_000_000_000;

/// The amount of yocto NEAR the contract dedicates to guarantee that the
/// "share" price never decreases. It's used during rounding errors for share ->
/// amount conversions.
const STAKE_SHARE_PRICE_GUARANTEE_FUND: Balance = 1_000_000_000_000;

/// There is no deposit balance attached.
const _NO_DEPOSIT: Balance = 0;

/// A type to distinguish between a balance and "stake" shares for better
/// readability.
pub type NumStakeShares = Balance;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[cfg(test)]
mod test_utils;

/// Inner account data of a delegate.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Account {
    /// The unstaked balance. It represents the amount the account has on this
    /// contract that can either be staked or withdrawn.
    pub unstaked:                        Balance,
    /// The amount of "stake" shares. Every stake share corresponds to the
    /// amount of staked balance. NOTE: The number of shares should always
    /// be less or equal than the amount of staked balance. This means the
    /// price of stake share should always be at least `1`. The price of
    /// stake share can be computed as `total_staked_balance` /
    /// `total_stake_shares`.
    pub stake_shares:                    NumStakeShares,
    /// The minimum epoch height when the withdrawn is allowed.
    /// This changes after unstaking action, because the amount is still locked
    /// for 3 epochs.
    pub unstaked_available_epoch_height: EpochHeight,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            unstaked:                        0,
            stake_shares:                    0,
            unstaked_available_epoch_height: 0,
        }
    }
}

/// The number of epochs required for the locked balance to become unlocked.
/// NOTE: The actual number of epochs when the funds are unlocked is 3. But
/// there is a corner case when the unstaking promise can arrive at the next
/// epoch, while the inner state is already updated in the previous epoch. It
/// will not unlock the funds for 4 epochs.
const NUM_EPOCHS_TO_UNLOCK: EpochHeight = 4;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakingContract {
    /// The account ID of the owner who's running the staking validator node.
    /// NOTE: This is different from the current account ID which is used as a
    /// validator account. The owner of the staking pool can change staking
    /// public key and adjust reward fees.
    pub owner_id:             AccountId,
    /// The public key which is used for staking action. It's the public key of
    /// the validator node that validates on behalf of the pool.
    pub stake_public_key:     PublicKey,
    /// The last epoch height when `ping` was called.
    pub last_epoch_height:    EpochHeight,
    /// The last total balance of the account (consists of staked and unstaked
    /// balances).
    pub last_total_balance:   Balance,
    /// The total amount of shares. It should be equal to the total amount of
    /// shares across all accounts.
    pub total_stake_shares:   NumStakeShares,
    /// The total staked balance.
    pub total_staked_balance: Balance,
    /// The fraction of the reward that goes to the owner of the staking pool
    /// for running the validator node.
    pub reward_fee_fraction:  RewardFeeFraction,
    /// Persistent map from an account ID to the corresponding account.
    pub accounts:             UnorderedMap<AccountId, Account>,
    /// Whether the staking is paused.
    /// When paused, the account unstakes everything (stakes 0) and doesn't
    /// restake. It doesn't affect the staking shares or reward
    /// distribution. Pausing is useful for node maintenance. Only the owner
    /// can pause and resume staking. The contract is not paused by default.
    pub paused:               bool,
    pub seda_token:           AccountId,
}

impl Default for StakingContract {
    fn default() -> Self {
        panic!("Staking contract should be initialized before usage")
    }
}

#[near_bindgen]
impl StakingContract {
    /// Initializes the contract with the given owner_id, initial staking public
    /// key (with ED25519 curve) and initial reward fee fraction that owner
    /// charges for the validation work.
    ///
    /// The entire current balance of this contract will be used to stake. This
    /// allows contract to always maintain staking shares that can't be
    /// unstaked or withdrawn. It prevents inflating the price of the share
    /// too much.
    #[init]
    pub fn new(
        owner_id: AccountId,
        stake_public_key: PublicKey,
        reward_fee_fraction: RewardFeeFraction,
        seda_token: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        reward_fee_fraction.assert_valid();
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        let account_balance = env::account_balance();
        let total_staked_balance = account_balance - STAKE_SHARE_PRICE_GUARANTEE_FUND;
        assert_eq!(
            env::account_locked_balance(),
            0,
            "The staking pool shouldn't be staking at the initialization"
        );
        assert!(
            env::is_valid_account_id(seda_token.as_bytes()),
            "The SEDA token account ID is invalid"
        );
        let mut this = Self {
            owner_id,
            stake_public_key,
            last_epoch_height: env::epoch_height(),
            last_total_balance: account_balance,
            total_staked_balance,
            total_stake_shares: total_staked_balance,
            reward_fee_fraction,
            accounts: UnorderedMap::new(b"u".to_vec()),
            paused: false,
            seda_token,
        };
        // Staking with the current pool to make sure the staking key is valid.
        this.internal_restake();
        this
    }
}
