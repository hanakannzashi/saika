use near_sdk::{Balance, Gas};

pub const MAX_RED_PACKET_SPLIT: usize = 100;
pub const MAX_RED_PACKET_MSG_LEN: usize = 100;

pub const NO_DEPOSIT: Balance = 0;
pub const ONE_YOCTO: Balance = 1;

pub const GAS_FOR_FT_TRANSFER: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_CLAIM_FUNGIBLE_TOKEN_RED_PACKET_CALLBACK: Gas = Gas(10_000_000_000_000);