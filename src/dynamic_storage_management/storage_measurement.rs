use super::StorageUsageChange;
use super::errors::{ERROR_REPEATED_START_STORAGE_MEASUREMENT, ERROR_MISSING_START_STORAGE_MEASUREMENT, ERROR_PENDING_STORAGE_MEASUREMENT};

use near_sdk::{env, StorageUsage};


pub struct StorageMeasurement {
    storage_usage_reference: StorageUsage,
    storage_usage_change: StorageUsageChange,
    pending: bool
}

impl StorageMeasurement {
    pub fn reset(&mut self) {
        self.storage_usage_reference = 0;
        self.storage_usage_change = 0;
        self.pending = false;
    }

    pub fn start(&mut self) {
        if self.pending {
            panic!("{}", ERROR_REPEATED_START_STORAGE_MEASUREMENT);
        }
        self.storage_usage_reference = env::storage_usage();
        self.pending = true;
    }

    pub fn stop(&mut self) {
        if !self.pending {
            panic!("{}", ERROR_MISSING_START_STORAGE_MEASUREMENT);
        }
        self.storage_usage_change +=
            StorageUsageChange::from(env::storage_usage()) -
                StorageUsageChange::from(self.storage_usage_reference);
        self.storage_usage_reference = 0;
        self.pending = false;
    }

    pub fn storage_usage_change(&mut self) -> StorageUsageChange {
        if self.pending {
            panic!("{}", ERROR_PENDING_STORAGE_MEASUREMENT)
        }
        self.storage_usage_change
    }
}

impl Default for StorageMeasurement {
    fn default() -> Self {
        Self {
            storage_usage_reference: 0,
            storage_usage_change: 0,
            pending: false
        }
    }
}