use super::storage_measurement::StorageMeasurement;
use super::account_storage::AccountStorage;
use super::errors::{ERROR_ACCOUNT_ALREADY_REGISTERED, ERROR_ACCOUNT_NOT_REGISTERED};

use near_sdk::{AccountId, Balance, IntoStorageKey, require};
use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use crate::dynamic_storage_management::dynamic_storage_basic::DynamicStorageBasic;
use crate::dynamic_storage_management::dynamic_storage_core::DynamicStorageCore;
use crate::dynamic_storage_management::errors::ERROR_NOT_ENOUGH_STORAGE_BALANCE;


#[derive(BorshDeserialize,BorshSerialize)]
pub struct DynamicStorageManager {
    /// Storage usage of accounts
    accounts: LookupMap<AccountId, AccountStorage>,
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
    /// The storage usage change caused by this method has been calculated.
    /// Panic when account is already registered.
    fn register_account(&mut self, account_id: AccountId, amount: Balance) {
        if self.account_registered(&account_id) {
            panic!("{}", ERROR_ACCOUNT_ALREADY_REGISTERED);
        };
        let mut account_storage = AccountStorage::default();
        account_storage.deposit_storage_balance(amount);

        self.start_measure_storage();
        self.accounts.insert(&account_id, &account_storage);
        self.stop_measure_and_update_storage_usage(&account_id);
    }
    /// Unregister account.
    /// Return remaining balance.
    /// Panic when account is not registered.
    fn unregister_account(&mut self, account_id: &AccountId) -> Balance {
        let mut account_storage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        account_storage.reset_storage_usage();
        let withdraw_amount = account_storage.withdraw_storage_balance(None);
        self.accounts.remove(account_id);
        withdraw_amount
    }
    /// Deposit more storage balance.
    /// Panic when account is not registered.
    fn deposit_storage_balance(&mut self, account_id: &AccountId, amount: Balance) {
        let mut account_storage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        account_storage.deposit_storage_balance(amount);
        self.accounts.insert(account_id, &account_storage);
    }
    /// Withdraw storage balance.
    /// If amount is [None] or amount is greater than available balance,
    /// withdraw available balance, else withdraw amount.
    /// Return withdraw balance.
    /// Panic when account is not registered.
    fn withdraw_storage_balance(&mut self, account_id: &AccountId, amount: Option<U128>) -> Balance {
        let mut account_storage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        let withdraw_amount = account_storage.withdraw_storage_balance(amount);
        self.accounts.insert(account_id, &account_storage);
        withdraw_amount
    }
    /// Register account if it is not registered, else deposit more storage balance
    fn register_account_or_deposit_storage_balance(&mut self, account_id: AccountId, amount: Balance) {
        if !self.account_registered(&account_id) {
            self.register_account(account_id, amount);
        } else {
            self.deposit_storage_balance(&account_id, amount);
        }
    }
    /// Whether account is registered.
    fn account_registered(&self, account_id: &AccountId) -> bool {
        self.accounts.get(account_id).is_some()
    }
    /// Whether account has enough balance to recover storage usage.
    /// If account is not registered or storage balance is not enough, return false, else return true.
    fn enough_storage_balance(&self, account_id: &AccountId) -> bool {
        match self.storage_balance(account_id) {
            None => false,
            Some((total, used)) => total >= used
        }
    }
    /// Get storage balance.
    /// If account is not registered, return [None], else return tuple (total, used).
    /// Note: used storage balance may be greater than total due to changing storage prices or incorrect storage ownership.
    fn storage_balance(&self, account_id: &AccountId) -> Option<(Balance, Balance)> {
        Some(self.accounts.get(account_id)?.storage_balance())
    }

    fn assert_no_registration(&self, account_id: &AccountId) {
        require!(!self.account_registered(account_id), ERROR_ACCOUNT_ALREADY_REGISTERED);
    }

    fn assert_registration(&self, account_id: &AccountId) {
        require!(self.account_registered(account_id), ERROR_ACCOUNT_NOT_REGISTERED);
    }

    fn assert_storage_balance(&self, account_id: &AccountId) {
        require!(self.enough_storage_balance(account_id), ERROR_NOT_ENOUGH_STORAGE_BALANCE);
    }
}

impl DynamicStorageCore for DynamicStorageManager {
    /// Start measure storage, it will save current contract storage usage and set measurement pending.
    /// Panic when repeated start measurement.
    fn start_measure_storage(&mut self) {
        self.storage_measurement.start();
    }
    /// Stop measure storage, it will calculate and save the storage change from the latest start to the present and set measurement idle.
    /// Panic when missing start measurement.
    fn stop_measure_storage(&mut self) {
        self.storage_measurement.stop();
    }
    /// Update storage usage, then reset measurement.
    /// Panic when 1.Account is not registered. 2.Measurement is pending.
    fn update_storage_usage(&mut self, account_id: &AccountId) {
        let mut account_storage = self.accounts
            .get(account_id)
            .expect(ERROR_ACCOUNT_NOT_REGISTERED);
        let storage_usage_change = self.storage_measurement.storage_usage_change();
        if storage_usage_change != 0 {
            account_storage.update_storage_usage(storage_usage_change);
            self.accounts.insert(account_id, &account_storage);
        }
        self.storage_measurement.reset();
    }
    /// Stop measure storage and update storage usage immediately, then reset measurement.
    /// Panic when 1.Account is not registered 2.Missing start measurement 3.Measurement is pending.
    fn stop_measure_and_update_storage_usage(&mut self, account_id: &AccountId) {
        self.stop_measure_storage();
        self.update_storage_usage(account_id);
    }
}