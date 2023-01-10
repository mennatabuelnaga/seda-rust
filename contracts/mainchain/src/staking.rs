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
        let mut account = self.get_account(&account_id);
        account.unstaked += amount;
        self.save_account(&account_id, &account);
        self.last_total_balance += amount;

        env::log_str(
            format!(
                "@{} deposited {}. New unstaked balance is {}",
                account_id, amount, account.unstaked
            )
            .as_str(),
        );

        PromiseOrValue::Value(U128::from(0)) // no refund
    }

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four
    /// most recent epochs.
    pub fn withdraw(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        assert!(amount > 0, "Withdrawal amount should be positive");

        let account_id = env::predecessor_account_id();
        let account = self.get_account(&account_id);
        assert!(account.unstaked >= amount, "Not enough unstaked balance to withdraw");
        assert!(
            account.unstaked_available_epoch_height <= env::epoch_height(),
            "The unstaked balance is not yet available due to unstaking delay"
        );

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

    pub fn stake(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        assert!(amount > 0, "Staking amount should be positive");
        let account_id = env::predecessor_account_id(); // TODO: env::signer_account_id()?
        let mut account = self.get_account(&account_id);
        // Calculate the number of "stake" shares that the account will receive for
        // staking the given amount.
        let num_shares = self.num_shares_from_staked_amount_rounded_down(amount);
        assert!(
            num_shares > 0,
            "The calculated number of \"stake\" shares received for staking should be positive"
        );
        // The amount of tokens the account will be charged from the unstaked balance.
        // Rounded down to avoid overcharging the account to guarantee that the account
        // can always unstake at least the same amount as staked.
        let charge_amount = self.staked_amount_from_num_shares_rounded_down(num_shares);
        assert!(
            charge_amount > 0,
            "Invariant violation. Calculated staked amount must be positive, because \"stake\" share price should be at least 1"
        );

        assert!(
            account.unstaked >= charge_amount,
            "Not enough unstaked balance to stake"
        );
        account.unstaked -= charge_amount;
        account.stake_shares += num_shares;
        self.save_account(&account_id, &account);

        // The staked amount that will be added to the total to guarantee the "stake"
        // share price never decreases. The difference between `stake_amount`
        // and `charge_amount` is paid from the allocated
        // STAKE_SHARE_PRICE_GUARANTEE_FUND.
        let stake_amount = self.staked_amount_from_num_shares_rounded_up(num_shares);

        self.total_staked_balance += stake_amount;
        self.total_stake_shares += num_shares;

        env::log_str(
            format!(
                "@{} staking {}. Received {} new staking shares. Total {} unstaked balance and {} staking shares",
                account_id, charge_amount, num_shares, account.unstaked, account.stake_shares
            )
            .as_str(),
        );
        env::log_str(
            format!(
                "Contract total staked balance is {}. Total number of shares {}",
                self.total_staked_balance, self.total_stake_shares
            )
            .as_str(),
        );
    }

    pub fn unstake(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        assert!(amount > 0, "Unstaking amount should be positive");

        let account_id = env::predecessor_account_id();
        let mut account = self.get_account(&account_id);

        assert!(
            self.total_staked_balance > 0,
            "The contract doesn't have staked balance"
        );
        // Calculate the number of shares required to unstake the given amount.
        // NOTE: The number of shares the account will pay is rounded up.
        let num_shares = self.num_shares_from_staked_amount_rounded_up(amount);
        assert!(
            num_shares > 0,
            "Invariant violation. The calculated number of \"stake\" shares for unstaking should be positive"
        );
        assert!(
            account.stake_shares >= num_shares,
            "Not enough staked balance to unstake"
        );

        // Calculating the amount of tokens the account will receive by unstaking the
        // corresponding number of "stake" shares, rounding up.
        let receive_amount = self.staked_amount_from_num_shares_rounded_up(num_shares);
        assert!(
            receive_amount > 0,
            "Invariant violation. Calculated staked amount must be positive, because \"stake\" share price should be at least 1"
        );

        account.stake_shares -= num_shares;
        account.unstaked += receive_amount;
        account.unstaked_available_epoch_height = env::epoch_height() + NUM_EPOCHS_TO_UNLOCK;
        self.save_account(&account_id, &account);

        // The amount tokens that will be unstaked from the total to guarantee the
        // "stake" share price never decreases. The difference between
        // `receive_amount` and `unstake_amount` is paid from the allocated
        // STAKE_SHARE_PRICE_GUARANTEE_FUND.
        let unstake_amount = self.staked_amount_from_num_shares_rounded_down(num_shares);

        self.total_staked_balance -= unstake_amount;
        self.total_stake_shares -= num_shares;

        env::log_str(
            format!(
                "@{} unstaking {}. Spent {} staking shares. Total {} unstaked balance and {} staking shares",
                account_id, receive_amount, num_shares, account.unstaked, account.stake_shares
            )
            .as_str(),
        );
        env::log_str(
            format!(
                "Contract total staked balance is {}. Total number of shares {}",
                self.total_staked_balance, self.total_stake_shares
            )
            .as_str(),
        );
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
        let mut account = self.get_account(&account_id);
        account.unstaked -= amount.0;
        self.save_account(&account_id, &account);

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
        let account = self.get_account(&account_id);
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
