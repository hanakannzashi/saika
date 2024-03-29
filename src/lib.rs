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
use near_sdk::collections::{UnorderedMap};
use near_sdk::{AccountId, PublicKey, PanicOnDefault, near_bindgen, env};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
struct OldContract {
    red_packets: UnorderedMap<PublicKey, RedPacket>,
    owners: UnorderedMap<AccountId, HashSet<PublicKey>>,
    storage_manager: DynamicStorageManager
}

#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
struct Contract {
    red_packets: UnorderedMap<PublicKey, RedPacket>,
    owners: UnorderedMap<AccountId, HashSet<PublicKey>>,
    storage_manager: DynamicStorageManager,
    helper_contract_id: AccountId
}


#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(helper_contract_id: AccountId) -> Self {
        Self {
            red_packets: UnorderedMap::new(StorageKey::RedPackets),
            owners: UnorderedMap::new(StorageKey::Owners),
            storage_manager: DynamicStorageManager::new(StorageKey::DynamicStorageManager),
            helper_contract_id
        }
    }

    #[init(ignore_state)]
    #[private]
    pub fn upgrade(helper_contract_id: AccountId) -> Self {
        let contract = env::state_read::<OldContract>().unwrap();
        Self {
            red_packets: contract.red_packets,
            owners: contract.owners,
            storage_manager: contract.storage_manager,
            helper_contract_id
        }
    }
}