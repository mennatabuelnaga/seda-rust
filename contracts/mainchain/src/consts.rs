use near_sdk::Gas;

pub const BASE_GAS: u64 = 5_000_000_000_000;
pub const PROMISE_CALL: u64 = 5_000_000_000_000;
pub const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);
