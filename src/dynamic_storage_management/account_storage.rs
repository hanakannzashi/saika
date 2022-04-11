use super::StorageUsageChange;

use std::cmp::min;
use near_sdk::{env, Balance, StorageUsage};
use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::json_types::U128;


#[derive(BorshDeserialize,BorshSerialize)]
pub struct AccountStorage {
    storage_usage: StorageUsage,
    storage_balance: Balance
}

impl AccountStorage {
    pub fn reset_storage_usage(&mut self) {
        self.storage_usage = 0;
    }

    pub fn update_storage_usage(&mut self, storage_usage_change: StorageUsageChange) {
        let storage_usage = StorageUsageChange::from(self.storage_usage);
        let new_storage_usage = storage_usage + storage_usage_change;
        if new_storage_usage > 0 {
            self.storage_usage = StorageUsage::try_from(new_storage_usage).unwrap();
        } else {
            self.storage_usage = 0;
        }
    }

    pub fn deposit_storage_balance(&mut self, amount: Balance) {
        self.storage_balance += amount;
    }

    pub fn withdraw_storage_balance(&mut self, amount: Option<U128>) -> Balance {
        let amount = amount.unwrap_or(U128(u128::MAX));
        let (total, used) = self.storage_balance();
        if total <= used {
            return 0;
        };
        let available_amount = total - used;
        let withdraw_amount = min(available_amount, amount.0);
        self.storage_balance -= withdraw_amount;
        withdraw_amount
    }

    pub fn storage_balance(&self) -> (Balance, Balance) {
        let total = self.storage_balance;
        let used = Balance::from(self.storage_usage) * env::storage_byte_cost();
        (total, used)
    }
}

impl Default for AccountStorage {
    fn default() -> Self {
        Self {
            storage_usage: 0,
            storage_balance: 0
        }
    }
}