use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize,Deserialize};
use near_sdk::BorshStorageKey;


#[derive(BorshStorageKey,BorshDeserialize,BorshSerialize)]
pub enum StorageKey {
    RedPackets,
    Owners,
    DynamicStorageManager
}

#[derive(BorshDeserialize,BorshSerialize,Serialize,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Token {
    NEAR,
    FungibleToken
}

#[derive(BorshDeserialize,BorshSerialize,Serialize,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum DistributionMod {
    Average,
    Random
}