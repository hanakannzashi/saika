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
//!         if !self.storage_manager.account_registered(&account_id) {
//!             self.storage_manager.register_account(
//!                 account_id,
//!                 amount
//!             );
//!         } else {
//!             log!("Account is already registered, refund balance");
//!             Promise::new(env::predecessor_account_id())
//!                 .transfer(amount);
//!         }
//!     }
//!
//!     // storage change method
//!     pub fn set_status(&mut self, message: String) {
//!         let account_id = env::predecessor_account_id();
//!         if !self.storage_manager.account_registered(&account_id) {
//!             panic!("Account is not registered");
//!         }
//!
//!         // start
//!         self.storage_manager.start_measure_storage();
//!
//!         // your storage change action
//!         self.records.insert(&account_id, &message);
//!
//!         // stop and update
//!         self.storage_manager
//!             .stop_measure_and_update_account_storage_usage(&account_id);
//!     }
//! }
//! ```

mod dynamic_storage_manager;
pub use dynamic_storage_manager::DynamicStorageManager;

mod account_storage_usage;
mod storage_measurement;
mod errors;

mod dynamic_storage_core;
pub use dynamic_storage_core::DynamicStorageCore;

mod dynamic_storage_basic;
pub use dynamic_storage_basic::DynamicStorageBasic;

type StorageUsageChange = i128;