#[allow(unused_imports)]
use near_sdk::{AccountId, ext_contract};
use near_sdk::json_types::U128;
use near_sdk::{Promise, PublicKey};


#[ext_contract(ext_ft)]
trait ExtFt {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_helper)]
trait ExtHelper {
    #[payable]
    fn create_account(&mut self, new_account_id: AccountId, new_public_key: PublicKey) -> Promise;
}