use super::StorageUsageChange;

use std::cmp::min;
use near_sdk::{env, Balance, StorageUsage};
use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::json_types::U128;


#[derive(BorshDeserialize,BorshSerialize)]
pub struct AccountStorageUsage {
    current_storage_usage: StorageUsage,
    total_storage_balance: Balance
}

impl AccountStorageUsage {
    pub fn clear_current_storage_usage(&mut self) {
        self.current_storage_usage = 0;
    }

    pub fn update_current_storage_usage(&mut self, storage_usage_change: StorageUsageChange) {
        let current_storage_usage = StorageUsageChange::from(self.current_storage_usage);
        let new_storage_usage = current_storage_usage + storage_usage_change;
        if new_storage_usage >= 0 {
            self.current_storage_usage = StorageUsage::try_from(new_storage_usage).unwrap();
        } else {
            self.current_storage_usage = 0;
        }
    }

    pub fn deposit_storage_balance(&mut self, deposit_balance: Balance) {
        self.total_storage_balance += deposit_balance;
    }

    pub fn withdraw_storage_balance(&mut self, amount: Option<U128>) -> Balance {
        let amount = amount.unwrap_or(U128(u128::MAX));
        let (total, current) = self.storage_balance();
        if total <= current {
            return 0;
        };
        let available_balance = total - current;
        let withdraw_balance = min(available_balance, amount.0);
        self.total_storage_balance -= withdraw_balance;
        withdraw_balance
    }

    pub fn storage_balance(&self) -> (Balance, Balance) {
        let total = self.total_storage_balance;
        let current = Balance::from(self.current_storage_usage) * env::storage_byte_cost();
        (total, current)
    }
}

impl Default for AccountStorageUsage {
    fn default() -> Self {
        Self {
            current_storage_usage: 0,
            total_storage_balance: 0
        }
    }
}