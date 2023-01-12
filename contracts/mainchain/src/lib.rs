pub mod account;
pub mod block;
pub mod data_request;
pub mod data_request_test;
pub mod fungible_token;
pub mod merkle;
pub mod node_registry;
pub mod node_registry_test;
pub mod staking;
pub mod storage;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, Vector},
    env,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
    BorshStorageKey,
    EpochHeight,
};
use uint::construct_uint;

use crate::{
    account::Account,
    block::{Block, BlockHeight, BlockId},
    node_registry::Node,
    staking::NumStakeShares,
};

/// Collection keys
#[derive(BorshStorageKey, BorshSerialize)]
enum MainchainStorageKeys {
    NumNodes,
    DataRequestAccumulator,
    BlockIdsByHeight,
    BlocksById,
    Accounts,
}

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

/// The amount of yocto NEAR the contract dedicates to guarantee that the
/// "share" price never decreases. It's used during rounding errors for share ->
/// amount conversions.
/// The amount of yocto NEAR the contract dedicates to guarantee that the
/// "share" price never decreases. It's used during rounding errors for share ->
/// amount conversions.
const STAKE_SHARE_PRICE_GUARANTEE_FUND: Balance = 1_000_000_000_000;

/// The number of epochs required for the locked balance to become unlocked.
/// NOTE: The actual number of epochs when the funds are unlocked is 3. But
/// there is a corner case when the unstaking promise can arrive at the next
/// epoch, while the inner state is already updated in the previous epoch. It
/// will not unlock the funds for 4 epochs.
const NUM_EPOCHS_TO_UNLOCK: EpochHeight = 4; // TODO: set our own epoch logic

/// Contract global state
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainchainContract {
    seda_token:               AccountId,
    num_nodes:                u64,
    nodes:                    LookupMap<u64, Node>,
    data_request_accumulator: Vector<String>,
    num_blocks:               BlockHeight,
    block_ids_by_height:      LookupMap<BlockHeight, BlockId>,
    blocks_by_id:             LookupMap<BlockId, Block>,
    /// The last total balance of the account (consists of staked and unstaked
    /// balances).
    pub last_total_balance:   Balance,
    /// The total amount of shares. It should be equal to the total amount of
    /// shares across all accounts.
    pub total_stake_shares:   NumStakeShares,
    /// The total staked balance.
    pub total_staked_balance: Balance,
    /// Persistent map from an account ID to the corresponding account.
    pub accounts:             UnorderedMap<AccountId, Account>,
}

impl Default for MainchainContract {
    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
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

/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[init]
    pub fn new(seda_token: AccountId) -> Self {
        let account_balance = STAKE_SHARE_PRICE_GUARANTEE_FUND; // TODO: fetch ft_balance_of this contract on initialization
        assert!(!env::state_exists(), "Already initialized");
        let total_staked_balance = account_balance;
        assert_eq!(
            env::account_locked_balance(),
            0,
            "The staking pool shouldn't be staking at the initialization"
        );
        assert!(
            env::is_valid_account_id(seda_token.as_bytes()),
            "The SEDA token account ID is invalid"
        );
        Self {
            seda_token,
            num_nodes: 0,
            nodes: LookupMap::new(MainchainStorageKeys::NumNodes),
            data_request_accumulator: Vector::<String>::new(MainchainStorageKeys::DataRequestAccumulator),
            num_blocks: 0,
            block_ids_by_height: LookupMap::new(MainchainStorageKeys::BlockIdsByHeight),
            blocks_by_id: LookupMap::new(MainchainStorageKeys::BlocksById),
            last_total_balance: account_balance,
            total_staked_balance,
            total_stake_shares: total_staked_balance,
            accounts: UnorderedMap::new(MainchainStorageKeys::Accounts),
        }
    }
}
