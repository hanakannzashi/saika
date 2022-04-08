use near_sdk::{AccountId, Balance};
use near_sdk::json_types::U128;

pub trait DynamicStorageBasic {
    fn register_account(&mut self, account_id: AccountId, deposit_balance: Balance);

    fn unregister_account(&mut self, account_id: &AccountId) -> Balance;

    fn deposit_storage_balance(&mut self, account_id: &AccountId, deposit_balance: Balance);

    fn withdraw_storage_balance(&mut self, account_id: &AccountId, amount: Option<U128>) -> Balance;

    fn register_account_or_deposit_storage_balance(&mut self, account_id: AccountId, deposit_balance: Balance);

    fn account_registered(&self, account_id: &AccountId) -> bool;

    fn enough_storage_balance(&self, account_id: &AccountId) -> bool;

    fn storage_balance(&self, account_id: &AccountId) -> Option<(Balance, Balance)>;
}