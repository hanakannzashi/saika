use near_sdk::AccountId;

pub trait DynamicStorageCore {
    fn start_measure_storage(&mut self);

    fn stop_measure_storage(&mut self);

    fn update_account_storage_usage(&mut self, account_id: &AccountId);

    fn stop_measure_and_update_account_storage_usage(&mut self, account_id: &AccountId);
}