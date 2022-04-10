mod red_packet;
mod constants;
mod utils;
mod enums;
mod cross_other;
mod fungible_token_receiver;
mod errors;
mod impl_storage_management;
mod dynamic_storage_management;
mod cross_self;
mod resolver;
mod impl_saika_red_packet;
mod view;
mod saika_red_packet;

use crate::dynamic_storage_management::DynamicStorageManager;
use crate::enums::StorageKey;
use crate::red_packet::RedPacket;

use std::collections::HashSet;
use near_sdk::collections::LookupMap;
use near_sdk::{AccountId, PublicKey, PanicOnDefault, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};


#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
struct Contract {
    red_packets: LookupMap<PublicKey, RedPacket>,
    owners: LookupMap<AccountId, HashSet<PublicKey>>,
    storage_manager: DynamicStorageManager
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self {
            red_packets: LookupMap::new(StorageKey::RedPackets),
            owners: LookupMap::new(StorageKey::Owners),
            storage_manager: DynamicStorageManager::new(StorageKey::DynamicStorageManager)
        }
    }
}






