use super::StorageUsageChange;
use super::errors::{ERROR_REPEATED_START_STORAGE_MEASUREMENT, ERROR_MISSING_START_STORAGE_MEASUREMENT, ERROR_PENDING_STORAGE_MEASUREMENT};

use near_sdk::{env, StorageUsage};

enum MeasurementState {
    Pending,
    Idle
}

pub struct StorageMeasurement {
    storage_usage_reference: StorageUsage,
    storage_usage_change: StorageUsageChange,
    state: MeasurementState
}

impl StorageMeasurement {
    pub fn reset(&mut self) {
        self.storage_usage_reference = 0;
        self.storage_usage_change = 0;
        self.state = MeasurementState::Idle;
    }

    pub fn start(&mut self) {
        match self.state {
            MeasurementState::Pending => {
                panic!("{}", ERROR_REPEATED_START_STORAGE_MEASUREMENT);
            }
            MeasurementState::Idle => {
                self.storage_usage_reference = env::storage_usage();
                self.state = MeasurementState::Pending;
            }
        }
    }

    pub fn stop(&mut self) {
        match self.state {
            MeasurementState::Pending => {
                self.storage_usage_change +=
                    StorageUsageChange::from(env::storage_usage()) -
                        StorageUsageChange::from(self.storage_usage_reference);
                self.storage_usage_reference = 0;
                self.state = MeasurementState::Idle;
            }
            MeasurementState::Idle => {
                panic!("{}", ERROR_MISSING_START_STORAGE_MEASUREMENT);
            }
        }
    }

    pub fn storage_usage_change(&self) -> StorageUsageChange {
        match self.state {
            MeasurementState::Pending => {
                panic!("{}", ERROR_PENDING_STORAGE_MEASUREMENT)
            }
            MeasurementState::Idle => {
                self.storage_usage_change
            }
        }
    }
}

impl Default for StorageMeasurement {
    fn default() -> Self {
        Self {
            storage_usage_reference: 0,
            storage_usage_change: 0,
            state: MeasurementState::Idle
        }
    }
}