#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use near_sdk::{
        mock::VmAction,
        test_utils::{get_created_receipts, VMContextBuilder},
        testing_env,
        Balance,
        EpochHeight,
        PromiseResult,
        VMContext,
    };

    use crate::{assert_eq_in_near, test_utils::*, RewardFeeFraction, StakingContract, U256};

    struct Emulator {
        pub contract:              StakingContract,
        pub epoch_height:          EpochHeight,
        pub amount:                Balance,
        pub locked_amount:         Balance,
        last_total_staked_balance: Balance,
        last_total_stake_shares:   Balance,
        context:                   VMContext,
    }

    fn zero_fee() -> RewardFeeFraction {
        RewardFeeFraction {
            numerator:   0,
            denominator: 1,
        }
    }

    impl Emulator {
        pub fn new(owner: String, stake_public_key: String, reward_fee_fraction: RewardFeeFraction) -> Self {
            let context = VMContextBuilder::new()
                .current_account_id(owner.clone().try_into().unwrap())
                .account_balance(ntoy(30))
                .build();
            testing_env!(context.clone());
            let contract = StakingContract::new(
                owner.try_into().unwrap(),
                stake_public_key.parse().unwrap(),
                reward_fee_fraction,
            );
            let last_total_staked_balance = contract.total_staked_balance;
            let last_total_stake_shares = contract.total_stake_shares;
            Emulator {
                contract,
                epoch_height: 0,
                amount: ntoy(30),
                locked_amount: 0,
                last_total_staked_balance,
                last_total_stake_shares,
                context,
            }
        }

        fn verify_stake_price_increase_guarantee(&mut self) {
            let total_staked_balance = self.contract.total_staked_balance;
            let total_stake_shares = self.contract.total_stake_shares;
            assert!(
                U256::from(total_staked_balance) * U256::from(self.last_total_stake_shares)
                    >= U256::from(self.last_total_staked_balance) * U256::from(total_stake_shares),
                "Price increase guarantee was violated."
            );
            self.last_total_staked_balance = total_staked_balance;
            self.last_total_stake_shares = total_stake_shares;
        }

        pub fn update_context(&mut self, predecessor_account_id: String, deposit: Balance) {
            self.verify_stake_price_increase_guarantee();
            self.context = VMContextBuilder::new()
                .current_account_id(staking().try_into().unwrap())
                .predecessor_account_id(predecessor_account_id.clone().try_into().unwrap())
                .signer_account_id(predecessor_account_id.try_into().unwrap())
                .attached_deposit(deposit)
                .account_balance(self.amount)
                .account_locked_balance(self.locked_amount)
                .epoch_height(self.epoch_height)
                .build();
            testing_env!(self.context.clone());
            println!(
                "Epoch: {}, Deposit: {}, amount: {}, locked_amount: {}",
                self.epoch_height, deposit, self.amount, self.locked_amount
            );
        }

        pub fn simulate_stake_call(&mut self) {
            let total_stake = self.contract.total_staked_balance;
            // Stake action
            self.amount = self.amount + self.locked_amount - total_stake;
            self.locked_amount = total_stake;
            // Second function call action
            self.update_context(staking(), 0);
        }

        pub fn skip_epochs(&mut self, num: EpochHeight) {
            self.epoch_height += num;
            self.locked_amount = (self.locked_amount * (100 + u128::from(num))) / 100;
        }
    }

    #[test]
    fn test_restake_fail() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        emulator.update_context(bob(), 0);
        emulator.contract.internal_restake();
        let receipts = get_created_receipts();
        assert_eq!(receipts.len(), 2);
        assert_eq!(
            &receipts[0].actions[0],
            &VmAction::Stake {
                stake:      29999999999999000000000000,
                public_key: "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7"
                    .to_string()
                    .parse()
                    .unwrap(),
            }
        );
        emulator.simulate_stake_call();

        emulator.update_context(staking(), 0);
        testing_env_with_promise_results(emulator.context.clone(), PromiseResult::Failed);
        emulator.contract.on_stake_action();
        let receipts = get_created_receipts();
        assert_eq!(receipts.len(), 1);
        assert_eq!(
            &receipts[0].actions[0],
            &VmAction::Stake {
                stake:      0,
                public_key: "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7"
                    .to_string()
                    .parse()
                    .unwrap(),
            }
        );
    }

    #[test]
    fn test_deposit_withdraw() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        let deposit_amount = ntoy(1_000_000);
        emulator.update_context(bob(), deposit_amount);
        emulator.contract.deposit();
        emulator.amount += deposit_amount;
        emulator.update_context(bob(), 0);
        assert_eq!(
            emulator
                .contract
                .get_account_unstaked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount
        );
        emulator.contract.withdraw(deposit_amount.into());
        assert_eq!(
            emulator
                .contract
                .get_account_unstaked_balance(bob().try_into().unwrap())
                .0,
            0u128
        );
    }

    #[test]
    fn test_stake_with_fee() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            RewardFeeFraction {
                numerator:   10,
                denominator: 100,
            },
        );
        let deposit_amount = ntoy(1_000_000);
        emulator.update_context(bob(), deposit_amount);
        emulator.contract.deposit();
        emulator.amount += deposit_amount;
        emulator.update_context(bob(), 0);
        emulator.contract.stake(deposit_amount.into());
        emulator.simulate_stake_call();
        assert_eq!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount
        );

        let locked_amount = emulator.locked_amount;
        let n_locked_amount = yton(locked_amount);
        emulator.skip_epochs(10);
        // Overriding rewards (+ 100K reward)
        emulator.locked_amount = locked_amount + ntoy(100_000);
        emulator.update_context(bob(), 0);
        emulator.contract.ping();
        let expected_amount =
            deposit_amount + ntoy((yton(deposit_amount) * 90_000 + n_locked_amount / 2) / n_locked_amount);
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            expected_amount
        );
        // Owner got 10% of the rewards
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(owner().try_into().unwrap())
                .0,
            ntoy(10_000)
        );

        let locked_amount = emulator.locked_amount;
        let n_locked_amount = yton(locked_amount);
        emulator.skip_epochs(10);
        // Overriding rewards (another 100K reward)
        emulator.locked_amount = locked_amount + ntoy(100_000);

        emulator.update_context(bob(), 0);
        emulator.contract.ping();
        // previous balance plus (1_090_000 / 1_100_030)% of the 90_000 reward (rounding
        // to nearest).
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            expected_amount + ntoy((yton(expected_amount) * 90_000 + n_locked_amount / 2) / n_locked_amount)
        );
        // owner earns 10% with the fee and also small percentage from restaking.
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(owner().try_into().unwrap())
                .0,
            ntoy(10_000) + ntoy(10_000) + ntoy((10_000u128 * 90_000 + n_locked_amount / 2) / n_locked_amount)
        );

        assert_eq!(emulator.contract.get_number_of_accounts(), 2);
    }

    #[test]
    fn test_stake_unstake() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        let deposit_amount = ntoy(1_000_000);
        emulator.update_context(bob(), deposit_amount);
        emulator.contract.deposit();
        emulator.amount += deposit_amount;
        emulator.update_context(bob(), 0);
        emulator.contract.stake(deposit_amount.into());
        emulator.simulate_stake_call();
        assert_eq!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount
        );
        let locked_amount = emulator.locked_amount;
        // 10 epochs later, unstake half of the money.
        emulator.skip_epochs(10);
        // Overriding rewards
        emulator.locked_amount = locked_amount + ntoy(10);
        emulator.update_context(bob(), 0);
        emulator.contract.ping();
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount + ntoy(10)
        );
        emulator.contract.unstake((deposit_amount / 2).into());
        emulator.simulate_stake_call();
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount / 2 + ntoy(10)
        );
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_unstaked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount / 2
        );
        let acc = emulator.contract.get_account(bob().try_into().unwrap());
        assert_eq!(acc.account_id, bob().try_into().unwrap());
        assert_eq_in_near!(acc.unstaked_balance.0, deposit_amount / 2);
        assert_eq_in_near!(acc.staked_balance.0, deposit_amount / 2 + ntoy(10));
        assert!(!acc.can_withdraw);

        assert!(
            !emulator
                .contract
                .is_account_unstaked_balance_available(bob().try_into().unwrap()),
        );
        emulator.skip_epochs(4);
        emulator.update_context(bob(), 0);
        assert!(
            emulator
                .contract
                .is_account_unstaked_balance_available(bob().try_into().unwrap()),
        );
    }

    #[test]
    fn test_stake_all_unstake_all() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        let deposit_amount = ntoy(1_000_000);
        emulator.update_context(bob(), deposit_amount);
        emulator.contract.deposit_and_stake();
        emulator.amount += deposit_amount;
        emulator.simulate_stake_call();
        assert_eq!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount
        );
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_unstaked_balance(bob().try_into().unwrap())
                .0,
            0
        );
        let locked_amount = emulator.locked_amount;

        // 10 epochs later, unstake all.
        emulator.skip_epochs(10);
        // Overriding rewards
        emulator.locked_amount = locked_amount + ntoy(10);
        emulator.update_context(bob(), 0);
        emulator.contract.ping();
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount + ntoy(10)
        );
        emulator.contract.unstake_all();
        emulator.simulate_stake_call();
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            0
        );
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_unstaked_balance(bob().try_into().unwrap())
                .0,
            deposit_amount + ntoy(10)
        );
    }

    /// Test that two can delegate and then undelegate their funds and rewards
    /// at different time.
    #[test]
    fn test_two_delegates() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        emulator.update_context(alice(), ntoy(1_000_000));
        emulator.contract.deposit();
        emulator.amount += ntoy(1_000_000);
        emulator.update_context(alice(), 0);
        emulator.contract.stake(ntoy(1_000_000).into());
        emulator.simulate_stake_call();
        emulator.skip_epochs(3);
        emulator.update_context(bob(), ntoy(1_000_000));

        emulator.contract.deposit();
        emulator.amount += ntoy(1_000_000);
        emulator.update_context(bob(), 0);
        emulator.contract.stake(ntoy(1_000_000).into());
        emulator.simulate_stake_call();
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            ntoy(1_000_000)
        );
        emulator.skip_epochs(3);
        emulator.update_context(alice(), 0);
        emulator.contract.ping();
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(alice().try_into().unwrap())
                .0,
            ntoy(1_060_900) - 1
        );
        assert_eq_in_near!(
            emulator
                .contract
                .get_account_staked_balance(bob().try_into().unwrap())
                .0,
            ntoy(1_030_000)
        );

        // Checking accounts view methods
        // Should be 2, because the pool has 0 fee.
        assert_eq!(emulator.contract.get_number_of_accounts(), 2);
        let accounts = emulator.contract.get_accounts(0, 10);
        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].account_id, alice().try_into().unwrap());
        assert_eq!(accounts[1].account_id, bob().try_into().unwrap());

        let accounts = emulator.contract.get_accounts(1, 10);
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].account_id, bob().try_into().unwrap());

        let accounts = emulator.contract.get_accounts(0, 1);
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].account_id, alice().try_into().unwrap());

        let accounts = emulator.contract.get_accounts(2, 10);
        assert_eq!(accounts.len(), 0);
    }

    #[test]
    fn test_low_balances() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        let initial_balance = 100;
        emulator.update_context(alice(), initial_balance);
        emulator.contract.deposit();
        emulator.amount += initial_balance;
        let mut remaining = initial_balance;
        let mut amount = 1;
        while remaining >= 4 {
            emulator.update_context(alice(), 0);
            amount = 2 + (amount - 1) % 3;
            emulator.contract.stake(amount.into());
            emulator.simulate_stake_call();
            remaining -= amount;
        }
    }

    #[test]
    fn test_rewards() {
        let mut emulator = Emulator::new(
            owner(),
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7".to_string(),
            zero_fee(),
        );
        let initial_balance = ntoy(100);
        emulator.update_context(alice(), initial_balance);
        emulator.contract.deposit();
        emulator.amount += initial_balance;
        let mut remaining = 100;
        let mut amount = 1;
        while remaining >= 4 {
            emulator.skip_epochs(3);
            emulator.update_context(alice(), 0);
            emulator.contract.ping();
            emulator.update_context(alice(), 0);
            amount = 2 + (amount - 1) % 3;
            emulator.contract.stake(ntoy(amount).into());
            emulator.simulate_stake_call();
            remaining -= amount;
        }
    }
}
