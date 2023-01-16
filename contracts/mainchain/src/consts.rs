use near_sdk::{Balance, EpochHeight, Gas};

pub const BASE_GAS: u64 = 5_000_000_000_000;
pub const PROMISE_CALL: u64 = 5_000_000_000_000;
pub const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);

/// The amount of yocto NEAR the contract dedicates to guarantee that the
/// "share" price never decreases. It's used during rounding errors for share ->
/// amount conversions.
/// The amount of yocto NEAR the contract dedicates to guarantee that the
/// "share" price never decreases. It's used during rounding errors for share ->
/// amount conversions.
pub const STAKE_SHARE_PRICE_GUARANTEE_FUND: Balance = 1_000_000_000_000;

/// The number of epochs required for the locked balance to become unlocked.
/// NOTE: The actual number of epochs when the funds are unlocked is 3. But
/// there is a corner case when the unstaking promise can arrive at the next
/// epoch, while the inner state is already updated in the previous epoch. It
/// will not unlock the funds for 4 epochs.
pub const NUM_EPOCHS_TO_UNLOCK: EpochHeight = 4; // TODO: set our own epoch logic
