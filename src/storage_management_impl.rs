use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
use near_sdk::{near_bindgen, AccountId, env, assert_one_yocto};
use near_sdk::json_types::U128;
use crate::*;
use crate::dynamic_storage_management::DynamicStorageBasic;


#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(&mut self, account_id: Option<AccountId>, registration_only: Option<bool>) -> StorageBalance {
        self.internal_storage_deposit(account_id, registration_only)
    }

    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        self.internal_storage_withdraw(env::predecessor_account_id(), amount)
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        self.internal_storage_unregister(force)
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.internal_storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.internal_storage_balance_of(account_id)
    }
}


impl Contract {
    fn internal_storage_deposit(&mut self, account_id: Option<AccountId>, registration_only: Option<bool>) -> StorageBalance {
        let account_id = account_id.unwrap_or(env::predecessor_account_id());
        let registration_only = registration_only.unwrap_or(false);
        let amount = env::attached_deposit();
        require!(amount > 0, "No balance for storage");

        if registration_only {
            if !self.storage_manager.account_registered(&account_id) {
                self.storage_manager.register_account(account_id.clone(), amount);
            } else {
                log!("{}", ERR_23_ACCOUNT_ALREADY_REGISTERED);
                transfer(account_id.clone(), amount);
            };
        } else {
            self.storage_manager.register_account_or_deposit_storage_balance(account_id.clone(), amount);
        };

        self.internal_storage_balance_of(account_id).unwrap()
    }

    fn internal_storage_withdraw(&mut self, account_id: AccountId, amount: Option<U128>) -> StorageBalance {
        if !self.storage_manager.account_registered(&account_id) {
            panic!("{}", ERR_21_ACCOUNT_NOT_REGISTERED);
        };

        let withdraw_balance = self.storage_manager.withdraw_storage_balance(&account_id, amount);
        if withdraw_balance > 0 {
            transfer(account_id.clone(), withdraw_balance);
        };

        self.internal_storage_balance_of(account_id).unwrap()
    }

    fn internal_storage_unregister(&mut self, force: Option<bool>) -> bool {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let force = force.unwrap_or(false);

        if !self.storage_manager.account_registered(&account_id) {
            panic!("{}", ERR_21_ACCOUNT_NOT_REGISTERED);
        };

        if self.all_saika_red_packets_run_out(&account_id) || force {
            self.clear_saika_red_packets(&account_id, force);
            let withdraw_balance = self.storage_manager.unregister_account(&account_id);
            if withdraw_balance > 0 {
                transfer(account_id, withdraw_balance);
            };
            return true
        };

        false
    }

    pub fn internal_storage_balance_bounds(&self) -> StorageBalanceBounds {
        panic!("{}", ERR_24_NO_STORAGE_BALANCE_BOUNDS);
    }

    pub fn internal_storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        let (total, current) = self.storage_manager.storage_balance(&account_id)?;
        let available;
        if current >= total {
            available = 0;
        } else {
            available = total - current;
        };
        Some(
            StorageBalance {
                total: total.into(),
                available: available.into()
            }
        )
    }
}