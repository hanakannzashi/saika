use super::storage_measurement::StorageMeasurement;
use super::account_storage_usage::AccountStorageUsage;
use super::errors::{ERROR_ACCOUNT_ALREADY_REGISTERED, ERROR_ACCOUNT_NOT_REGISTERED};

use near_sdk::{AccountId, Balance, IntoStorageKey};
use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use crate::dynamic_storage_management::dynamic_storage_basic::DynamicStorageBasic;
use crate::dynamic_storage_management::dynamic_storage_core::DynamicStorageCore;


#[derive(BorshDeserialize,BorshSerialize)]
pub struct DynamicStorageManager {
    /// Storage usage of accounts
    accounts: LookupMap<AccountId, AccountStorageUsage>,
    /// Measuring storage usage and saving the storage usage change
    #[borsh_skip]
    storage_measurement: StorageMeasurement
}

impl DynamicStorageManager {
    pub fn new<S>(key_prefix: S) -> Self where S: IntoStorageKey {
        Self {
            accounts: LookupMap::new(key_prefix),
            storage_measurement: StorageMeasurement::default()
        }
    }
}

impl DynamicStorageBasic for DynamicStorageManager {
    /// Register account with any storage balance.
    /// Panic when account is already registered.
    fn register_account(&mut self, account_id: AccountId, deposit_balance: Balance) {
        if self.account_registered(&account_id) {
            panic!("{}", ERROR_ACCOUNT_ALREADY_REGISTERED);
        };

        self.start_measure_storage();

        let mut account_storage_usage = AccountStorageUsage::default();
        account_storage_usage.deposit_storage_balance(deposit_balance);
        self.accounts.insert(&account_id, &account_storage_usage);

        self.stop_measure_and_update_account_storage_usage(&account_id);
    }
    /// Unregister account.
    /// Return remaining balance.
    /// Panic when account is not registered.
    fn unregister_account(&mut self, account_id: &AccountId) -> Balance {
        let mut account_storage_usage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        account_storage_usage.clear_current_storage_usage();
        let withdraw_balance = account_storage_usage.withdraw_storage_balance(None);
        self.accounts.remove(account_id);
        withdraw_balance
    }
    /// Deposit more storage balance.
    /// Panic when account is not registered.
    fn deposit_storage_balance(&mut self, account_id: &AccountId, deposit_balance: Balance) {
        let mut account_storage_usage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        account_storage_usage.deposit_storage_balance(deposit_balance);
        self.accounts.insert(account_id, &account_storage_usage);
    }
    /// Withdraw storage balance.
    /// If amount is [None] or amount is greater than available balance,
    /// withdraw available balance, else withdraw amount.
    /// Return withdraw balance.
    /// Panic when account is not registered.
    fn withdraw_storage_balance(&mut self, account_id: &AccountId, amount: Option<U128>) -> Balance {
        let mut account_storage_usage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        let withdraw_balance = account_storage_usage.withdraw_storage_balance(amount);
        self.accounts.insert(account_id, &account_storage_usage);
        withdraw_balance
    }
    /// Register account if it is not registered, else deposit more storage balance
    fn register_account_or_deposit_storage_balance(&mut self, account_id: AccountId, deposit_balance: Balance) {
        if !self.account_registered(&account_id) {
            self.register_account(account_id, deposit_balance);
        } else {
            self.deposit_storage_balance(&account_id, deposit_balance);
        }
    }
    /// Whether account is registered.
    fn account_registered(&self, account_id: &AccountId) -> bool {
        self.accounts.get(account_id).is_some()
    }
    /// Whether account has enough balance to recover storage usage.
    /// Panic when account is not registered.
    fn enough_storage_balance(&self, account_id: &AccountId) -> bool {
        let account_storage_usage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        let (total, current) = account_storage_usage.storage_balance();
        total >= current
    }
    /// Get storage balance.
    /// If account is not registered, return [None], else return tuple (total, current).
    /// Note: current storage balance may be greater than total due to changing storage prices or incorrect storage ownership.
    fn storage_balance(&self, account_id: &AccountId) -> Option<(Balance, Balance)> {
        let account_storage_usage = self.accounts.get(account_id)?;
        let storage_balance = account_storage_usage.storage_balance();
        Some(storage_balance)
    }
}

impl DynamicStorageCore for DynamicStorageManager {
    /// Start measure storage, it will save current contract storage usage and set measurement pending.
    /// Panic when repeated start measurement.
    fn start_measure_storage(&mut self) {
        self.storage_measurement.start();
    }
    /// Stop measure storage, it will calculate and save the storage change from the latest start to the present and set measurement pause.
    /// Panic when missing start measurement.
    fn stop_measure_storage(&mut self) {
        self.storage_measurement.end();
    }
    /// Update account storage usage, then reset measurement.
    /// Panic when 1.Account is not registered. 2.Measurement is pending.
    fn update_account_storage_usage(&mut self, account_id: &AccountId) {
        let mut account_storage_usage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        let storage_usage_change = self.storage_measurement.storage_usage_change();
        if storage_usage_change != 0 {
            account_storage_usage.update_current_storage_usage(storage_usage_change);
            self.accounts.insert(account_id, &account_storage_usage);
        }
        self.storage_measurement.reset();
    }
    /// Stop measure storage and update account storage usage immediately, then reset measurement.
    /// Panic when 1.Account is not registered 2.Missing start measurement 3.Measurement is pending.
    fn stop_measure_and_update_account_storage_usage(&mut self, account_id: &AccountId) {
        self.stop_measure_storage();
        self.update_account_storage_usage(account_id);
    }
}