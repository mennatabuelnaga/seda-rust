use near_sdk::{
    env,
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
    Promise,
    PromiseError,
    PromiseOrValue,
    PublicKey,
};

use crate::{
    fungible_token::{ft, GAS_FOR_FT_ON_TRANSFER},
    MainchainContract,
    MainchainContractExt,
    RewardFeeFraction,
    NUM_EPOCHS_TO_UNLOCK,
};

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
/// Contract public methods
#[near_bindgen]
impl MainchainContract {
    #[private] // require caller to be this contract
    pub fn deposit(&mut self, amount: u128, account_id: AccountId) -> PromiseOrValue<U128> {
        self.internal_deposit(amount, account_id)
    }

    /// Deposits the attached amount into the inner account of the predecessor
    /// and stakes it.
    #[private] // require caller to be this contract
    pub fn deposit_and_stake(&mut self, amount: u128, account_id: AccountId) {
        self.internal_deposit(amount, account_id);
        self.internal_stake(amount);
    }

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        self.internal_withdraw(amount);
    }

    /// Withdraws the entire unstaked balance from the predecessor account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw_all(&mut self) {
        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        self.internal_withdraw(account.unstaked);
    }

    pub fn stake(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        self.internal_stake(amount);
    }

    pub fn stake_all(&mut self) {
        let account_id = env::predecessor_account_id(); // TODO: env::signer_account_id()??
        let account = self.internal_get_account(&account_id);
        let amount: Balance = account.unstaked;
        self.internal_stake(amount);
    }

    pub fn unstake(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        self.internal_unstake(amount);
    }

    pub fn unstake_all(&mut self) {
        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        let amount = self.staked_amount_from_num_shares_rounded_down(account.stake_shares);
        self.internal_unstake(amount);
    }

    #[private] // require caller to be this contract
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

        // update account
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

        // update global balance
        self.last_total_balance -= amount.0;
    }

    /*************** */
    /* View methods */
    /*************** */

    /// Returns the unstaked balance of the given account.
    pub fn get_account_unstaked_balance(&self, account_id: AccountId) -> U128 {
        self.get_human_readable_account(account_id).unstaked_balance
    }

    /// Returns the staked balance of the given account.
    /// NOTE: This is computed from the amount of "stake" shares the given
    /// account has and the current amount of total staked balance and total
    /// stake shares on the account.
    pub fn get_account_staked_balance(&self, account_id: AccountId) -> U128 {
        self.get_human_readable_account(account_id).staked_balance
    }

    /// Returns the total balance of the given account (including staked and
    /// unstaked balances).
    pub fn get_account_total_balance(&self, account_id: AccountId) -> U128 {
        let account = self.get_human_readable_account(account_id);
        (account.unstaked_balance.0 + account.staked_balance.0).into()
    }

    /// Returns `true` if the given account can withdraw tokens in the current
    /// epoch.
    pub fn is_account_unstaked_balance_available(&self, account_id: AccountId) -> bool {
        self.get_human_readable_account(account_id).can_withdraw
    }

    /// Returns the total staking balance.
    pub fn get_total_staked_balance(&self) -> U128 {
        self.total_staked_balance.into()
    }

    // /// Returns account ID of the staking pool owner.
    // pub fn get_owner_id(&self) -> AccountId {
    //     self.owner_id.clone()
    // }

    // /// Returns the current reward fee as a fraction.
    // pub fn get_reward_fee_fraction(&self) -> RewardFeeFraction {
    //     self.reward_fee_fraction.clone()
    // }

    // /// Returns the staking public key
    // pub fn get_staking_key(&self) -> PublicKey {
    //     self.stake_public_key.clone()
    // }

    /// Returns true if the staking is paused
    pub fn is_staking_paused(&self) -> bool {
        self.paused
    }

    /// Returns human readable representation of the account for the given
    /// account ID.
    pub fn get_human_readable_account(&self, account_id: AccountId) -> HumanReadableAccount {
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
    pub fn get_human_readable_accounts(&self, from_index: u64, limit: u64) -> Vec<HumanReadableAccount> {
        let keys = self.accounts.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| self.get_human_readable_account(keys.get(index).unwrap()))
            .collect()
    }
}
