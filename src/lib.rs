mod red_packet;
mod constant;
mod near_red_packet;
mod utils;
mod ft_red_packet;
mod enums;
mod ext_others;
mod ft_receiver;
mod errors;
mod storage_management_impl;
mod dynamic_storage_management;

use crate::enums::*;
use crate::utils::*;
use crate::ft_red_packet::{FungibleTokenRedPacket};
use crate::near_red_packet::NearRedPacket;
use crate::errors::*;
use crate::dynamic_storage_management::{DynamicStorageBasic, DynamicStorageCore, DynamicStorageManager};
use std::collections::HashSet;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::*;
use near_sdk::{AccountId, env, log, near_bindgen, PanicOnDefault, PublicKey, PromiseOrValue, require};
use near_sdk::json_types::U128;


#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
struct Contract {
    saika_red_packets: LookupMap<PublicKey, SaikaRedPacket>,
    owners: LookupMap<AccountId, HashSet<PublicKey>>,
    storage_manager: DynamicStorageManager
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self {
            saika_red_packets: LookupMap::new(StorageKey::SaikaRedPackets),
            owners: LookupMap::new(StorageKey::Owners),
            storage_manager: DynamicStorageManager::new(StorageKey::DynamicStorageManager)
        }
    }
}


#[near_bindgen]
impl Contract {
    /// create a near red packet
    #[payable]
    pub fn create_near_red_packet(
        &mut self,
        public_key: PublicKey,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    ) -> bool {
        self.internal_create_near_red_packet(
            public_key,
            split,
            distribution_mod,
            msg,
            white_list
        )
    }
    /// claim near red Packet and fungible token red packet with private key
    pub fn claim_saika_red_packet(&mut self, claimer_id: AccountId) -> U128 {
        self.internal_claim_saika_red_packet(claimer_id)
    }
    /// refund balance
    pub fn refund(&mut self, public_key: PublicKey) -> U128 {
        self.internal_refund(public_key)
    }
    /// remove red packet run out
    pub fn remove_history(&mut self, public_key: PublicKey) -> bool {
        self.internal_remove_history(public_key)
    }
    /// remove all red packet run out
    pub fn clear_history(&mut self) {
        self.internal_clear_history();
    }
    /// view owner's red packets detail
    pub fn get_saika_red_packets_by_owner_id(&self, owner_id: AccountId) -> Vec<SaikaRedPacket> {
        self.owners.get(&owner_id).unwrap_or(HashSet::new())
            .into_iter()
            .map(|public_key|{self.saika_red_packets.get(&public_key).unwrap()})
            .collect()
    }
    /// view owner's red packet public keys
    pub fn get_pks_by_owner_id(&self, owner_id: AccountId) -> HashSet<PublicKey> {
        self.owners.get(&owner_id).unwrap_or(HashSet::new())
    }
    /// view the red packet detail related to public key
    pub fn get_saika_red_packet_by_pk(&self, public_key: PublicKey) -> Option<SaikaRedPacket> {
        self.saika_red_packets.get(&public_key)
    }
}


impl Contract {

    pub fn internal_create_near_red_packet(
        &mut self,
        public_key: PublicKey,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    ) -> bool {
        self.storage_manager.start_measure_storage();

        let owner_id = env::predecessor_account_id();
        let init_balance = env::attached_deposit();
        require!(init_balance > 0, "No balance for red packet");

        let near_red_packet = SaikaRedPacket::NearRedPacket(
            NearRedPacket::new(
                owner_id.clone(),
                init_balance.into(),
                split,
                distribution_mod,
                msg,
                white_list,
                env::block_timestamp()
            )
        );

        if !self.allow_creation(&owner_id, &public_key, &near_red_packet) {
            transfer(owner_id, init_balance);
            return false;
        }

        self.add_saika_red_packet(
            owner_id.clone(),
            public_key,
            near_red_packet
        );

        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        true
    }

    pub fn internal_create_fungible_token_red_packet(
        &mut self,
        token_id: AccountId,
        owner_id: AccountId,
        init_balance: U128,
        public_key: PublicKey,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    ) -> PromiseOrValue<U128> {
        self.storage_manager.start_measure_storage();

        require!(init_balance.0 > 0, "No balance for red packet");

        let ft_red_packet = SaikaRedPacket::FungibleTokenRedPacket(
            FungibleTokenRedPacket::new(
                token_id,
                owner_id.clone(),
                init_balance,
                split,
                distribution_mod,
                msg,
                white_list,
                env::block_timestamp()
            )
        );

        if !self.allow_creation(&owner_id, &public_key,&ft_red_packet) {
            return PromiseOrValue::Value(init_balance);
        }

        self.add_saika_red_packet(
            owner_id.clone(),
            public_key,
            ft_red_packet
        );

        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        PromiseOrValue::Value(U128(0))
    }

    pub fn internal_claim_saika_red_packet(&mut self, claimer_id: AccountId) -> U128 {
        self.storage_manager.start_measure_storage();

        let public_key = env::signer_account_pk();
        let mut saika_red_packet = self.saika_red_packets
            .get(&public_key)
            .expect(ERR_01_NO_MATCHING_RED_PACKET);
        let claim_amount = saika_red_packet.claim(claimer_id).unwrap();
        self.saika_red_packets.insert(&public_key, &saika_red_packet);

        self.storage_manager.stop_measure_and_update_account_storage_usage(&saika_red_packet.owner_id());

        claim_amount
    }

    pub fn internal_refund(&mut self, public_key: PublicKey) -> U128 {
        self.storage_manager.start_measure_storage();

        let owner_id = env::predecessor_account_id();

        let mut saika_red_packet = self.saika_red_packets
            .get(&public_key)
            .expect(ERR_01_NO_MATCHING_RED_PACKET);
        let refund_amount = saika_red_packet.refund(owner_id.clone()).unwrap();
        self.saika_red_packets.insert(&public_key, &saika_red_packet);

        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        refund_amount
    }

    pub fn internal_remove_history(&mut self, public_key: PublicKey) -> bool {
        self.storage_manager.start_measure_storage();

        let owner_id = env::predecessor_account_id();
        let success = self.remove_saika_red_packet(&public_key,&owner_id,false);

        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        success
    }

    pub fn internal_clear_history(&mut self) {
        self.storage_manager.start_measure_storage();

        let owner_id = env::predecessor_account_id();
        self.clear_saika_red_packets(&owner_id,false);

        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);
    }
}


impl Contract {
    pub fn add_saika_red_packet(
        &mut self,
        owner_id: AccountId,
        public_key: PublicKey,
        saika_red_packet: SaikaRedPacket
    ) {
        let mut public_keys = self.owners.get(&owner_id).unwrap_or(HashSet::new());
        public_keys.insert(public_key.clone());
        self.owners.insert(&owner_id, &public_keys);
        self.saika_red_packets.insert(&public_key, &saika_red_packet);
    }

    pub fn remove_saika_red_packet(&mut self, public_key: &PublicKey, owner_id: &AccountId, force: bool) -> bool {
        match self.saika_red_packets.get(public_key) {
            None => {
                return true;
            }
            Some(saika_red_packet) => {
                if *owner_id != saika_red_packet.owner_id() {
                    log!("{}", ERR_02_NO_PERMISSION_TO_RED_PACKET);
                    return false;
                };
                if !saika_red_packet.is_run_out() && !force {
                    log!("{}", ERR_03_NOT_RUN_OUT);
                    return false;
                };

                let mut public_keys = self.owners.get(&owner_id).unwrap();
                public_keys.remove(public_key);

                if public_keys.is_empty() {
                    self.owners.remove(owner_id);
                } else {
                    self.owners.insert(owner_id, &public_keys);
                }
                self.saika_red_packets.remove(public_key);
            }
        };

        true
    }

    pub fn clear_saika_red_packets(&mut self, owner_id: &AccountId, force: bool) {
        match self.owners.get(owner_id) {
            None => {
                return;
            }
            Some(mut public_keys) => {
                public_keys.retain(|public_key| {
                    let saika_red_packet = self.saika_red_packets.get(public_key).unwrap();
                    let is_run_out = saika_red_packet.is_run_out();
                    if is_run_out || force {
                        self.saika_red_packets.remove(public_key);
                    };
                    !is_run_out && !force
                });
                if public_keys.is_empty() {
                    self.owners.remove(owner_id);
                } else {
                    self.owners.insert(owner_id, &public_keys);
                }
            }
        };
    }

    pub fn unique_public_key(&self, public_key: &PublicKey) -> bool {
        self.saika_red_packets.get(public_key).is_none()
    }

    pub fn saika_red_packet_count(&self, owner_id: &AccountId) -> (usize, usize) {
        let mut total = 0;
        let mut run_out = 0;

        match self.owners.get(owner_id) {
            None => (),
            Some(public_keys) => {
                total = public_keys.len();
                for public_key in public_keys {
                    let saika_red_packet = self.saika_red_packets.get(&public_key).unwrap();
                    if saika_red_packet.is_run_out() {
                        run_out += 1;
                    };
                };
            }
        };
        (total, run_out)
    }

    pub fn all_saika_red_packets_run_out(&self, owner_id: &AccountId) -> bool {
        let count = self.saika_red_packet_count(owner_id);
        count.0 == count.1
    }

    pub fn allow_creation(&self, owner_id: &AccountId, public_key: &PublicKey, saika_red_packet: &SaikaRedPacket) -> bool {
        if !saika_red_packet.is_valid() {
            log!("{}", ERR_04_INVALID_PARAMETER);
            return false;
        }

        if !self.unique_public_key(public_key) {
            log!("{}", ERR_05_NOT_UNIQUE_PUBLIC_KEY);
            return false;
        }

        if !self.storage_manager.account_registered(owner_id) {
            log!("{}", ERR_21_ACCOUNT_NOT_REGISTERED);
            return false;
        }

        if !self.storage_manager.enough_storage_balance(owner_id) {
            log!("{}", ERR_22_NOT_ENOUGH_STORAGE_BALANCE);
            return false;
        }

        true
    }
}






