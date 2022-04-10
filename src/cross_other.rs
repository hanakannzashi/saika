#[allow(unused_imports)]
use near_sdk::{AccountId, ext_contract};
use near_sdk::json_types::U128;


#[ext_contract(ext_ft)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}