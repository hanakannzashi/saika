use std::cmp::min;
use crate::ext_others::*;
use crate::constant::*;
use near_sdk::{AccountId, Balance, env, Promise};
use near_sdk::json_types::U128;


pub fn average_sub(number: u128, split: usize) -> u128 {
    let split = split as u128;
    assert!(number >= split, "number must >= split");
    return number / split
}

pub fn random_sub(number: u128, split: usize, min_sub: Option<u128>) -> u128 {
    // The closer min_sub gets to 0, the fairer it is
    let min_sub = min_sub.unwrap_or(1);
    let split = split as u128;
    assert!(number >= split * min_sub, "number must >= split * min_sub, default min_sub == 1");
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
