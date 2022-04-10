mod red_packet;
mod constants;
mod utils;
mod enums;
mod cross_other;
mod impl_fungible_token_receiver;
mod errors;
mod impl_storage_management;
mod dynamic_storage_management;
mod cross_self;
mod saika_red_packet_resolver;
mod impl_saika_red_packet;
mod red_packet_view;
mod saika_red_packet;
mod impl_saika_red_packet_resolver;

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
    #[private]
    pub fn init() -> Self {
        Self {
            red_packets: LookupMap::new(StorageKey::RedPackets),
            owners: LookupMap::new(StorageKey::Owners),
            storage_manager: DynamicStorageManager::new(StorageKey::DynamicStorageManager)
        }
    }
}