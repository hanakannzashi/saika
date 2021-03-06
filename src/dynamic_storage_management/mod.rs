//! # Example
//! ```
//! #[near_bindgen]
//! #[derive(BorshDeserialize, BorshSerialize)]
//! struct StatusMessage {
//!     records: LookupMap<AccountId, String>,
//!     // add storage manager field
//!     storage_manager: DynamicStorageManager
//! }
//!
//! #[near_bindgen]
//! impl StatusMessage {
//!     // register method
//!     #[payable]
//!     pub fn register_account(&mut self, account_id: AccountId) {
//!         let amount = env::attached_deposit();
//!         require!(amount > 0, "No balance for storage");
//!         self.storage_manager.assert_no_registration(&account_id);
//!         self.storage_manager.register_account(account_id, amount);
//!     }
//!
//!     // storage change method
//!     pub fn set_status(&mut self, message: String) {
//!         self.storage_manager.assert_registration(&account_id);
//!         // start
//!         self.storage_manager.start_measure_storage();
//!         // your storage change operation
//!         self.records.insert(&account_id, &message);
//!         // stop and update
//!         self.storage_manager.stop_measure_and_update_storage_usage(&account_id);
//!         self.storage_manager.assert_storage_balance(&account_id);
//!     }
//! }
//! ```

mod dynamic_storage_manager;
pub use dynamic_storage_manager::DynamicStorageManager;

mod account_storage;
mod storage_measurement;
mod errors;

mod dynamic_storage_core;
pub use dynamic_storage_core::DynamicStorageCore;

mod dynamic_storage_basic;
pub use dynamic_storage_basic::DynamicStorageBasic;

type StorageUsageChange = i128;