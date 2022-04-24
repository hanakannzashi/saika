use crate::cross_other::*;
use crate::cross_self::*;
use crate::constants::*;

use std::cmp::min;
use near_sdk::{AccountId, Balance, env, Promise, PublicKey, require};
use near_sdk::json_types::U128;


pub fn assert_zero_deposit(amount: Balance) {
    require!(amount > 0, "Deposit amount is 0");
}

pub fn average_sub(number: u128, split: usize) -> u128 {
    let split = u128::try_from(split).unwrap();
    require!(number >= split, "number must >= split");
    return number / split
}

pub fn random_sub(number: u128, split: usize, min_sub: Option<u128>) -> u128 {
    // The closer min_sub gets to 0, the fairer it is
    let min_sub = min_sub.unwrap_or(1);
    let split = u128::try_from(split).unwrap();
    require!(number >= split * min_sub, "number must >= split * min_sub, default min_sub == 1");
    if split == 1 {
        return number;
    };
    let max_sub = min(number - min_sub * (split - 1), 2 * (number / split));
    gen_range(min_sub, max_sub + 1)
}

pub fn rand_u128() -> u128 {
    let seed = env::random_seed();
    let mut arr: [u8; 16] = Default::default();
    arr.copy_from_slice(&seed[..16]);
    u128::from_le_bytes(arr)
}

pub fn gen_range(start: u128, end: u128) -> u128 {
    rand_u128() % (end - start) + start
}

pub fn transfer(to: AccountId, amount: Balance) -> Promise {
    Promise::new(to).transfer(amount)
}

pub fn transfer_ft(to: AccountId, amount: U128, token_id: AccountId) -> Promise {
    ext_ft::ft_transfer(to, amount, None, token_id, ONE_YOCTO, GAS_FOR_FT_TRANSFER)
}

pub fn transfer_ft_with_claim_fungible_token_red_packet_callback(
    to: AccountId,
    amount: U128,
    token_id: AccountId,
    owner_id: AccountId,
    public_key: PublicKey
) -> Promise {
    transfer_ft(to.clone(), amount, token_id.clone())
        .then(
            ext_self::claim_fungible_token_red_packet_callback(
                to,
                owner_id,
                amount,
                token_id,
                public_key,
                env::current_account_id(),
                NO_DEPOSIT,
                GAS_FOR_CLAIM_FUNGIBLE_TOKEN_RED_PACKET_CALLBACK
            )
        )
}
