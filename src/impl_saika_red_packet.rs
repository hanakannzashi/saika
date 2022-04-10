use crate::enums::*;
use crate::utils::*;
use crate::dynamic_storage_management::{DynamicStorageBasic, DynamicStorageCore};
use crate::errors::*;
use crate::red_packet::RedPacket;
use crate::Contract;
use crate::view::RedPacketView;
use crate::saika_red_packet::SaikaRedPacket;

use std::collections::HashSet;
use near_sdk::{AccountId, env, near_bindgen, PublicKey, PromiseOrValue, require, Balance};
use near_sdk::json_types::U128;


#[near_bindgen]
impl SaikaRedPacket for Contract {
    /// create a near red packet
    #[payable]
    fn create_near_red_packet(
        &mut self,
        public_key: PublicKey,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    ) {
        self.internal_create_near_red_packet(
            env::predecessor_account_id(),
            env::attached_deposit(),
            public_key,
            split,
            distribution_mod,
            msg,
            white_list
        );
    }
    /// claim near red Packet and fungible token red packet with private key
    fn claim_red_packet(&mut self, claimer_id: AccountId) -> U128 {
        self.internal_claim_red_packet(claimer_id)
    }
    /// refund balance
    fn refund(&mut self, public_key: PublicKey) -> U128 {
        self.internal_refund(public_key)
    }
    /// remove red packet run out
    fn remove_history(&mut self, public_key: PublicKey) {
        self.internal_remove_history(public_key)
    }
    /// remove all red packet run out
    fn clear_history(&mut self) {
        self.internal_clear_history();
    }
    /// view owner's red packets detail
    fn get_red_packets_by_owner_id(&self, owner_id: AccountId) -> Vec<RedPacketView> {
        self.owners.get(&owner_id).unwrap_or(HashSet::new())
            .into_iter()
            .map(|public_key|{
                let mut red_packet_view:RedPacketView = self.red_packets
                    .get(&public_key)
                    .unwrap()
                    .into();
                red_packet_view.public_key = Some(public_key);
                red_packet_view
            })
            .collect()
    }
    /// view owner's red packet public keys
    fn get_pks_by_owner_id(&self, owner_id: AccountId) -> HashSet<PublicKey> {
        self.owners.get(&owner_id).unwrap_or(HashSet::new())
    }
    /// view the red packet detail related to public key
    fn get_red_packet_by_pk(&self, public_key: PublicKey) -> Option<RedPacketView> {
        let mut red_packet_view: RedPacketView = self.red_packets.get(&public_key)?.into();
        red_packet_view.public_key = Some(public_key);
        Some(red_packet_view)
    }
}


impl Contract {
    pub fn internal_create_near_red_packet(
        &mut self,
        owner_id: AccountId,
        init_balance: Balance,
        public_key: PublicKey,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    ) {
        assert_zero_deposit(init_balance);
        self.assert_storage_before(&owner_id);
        self.assert_creation(&public_key);

        let near_red_packet = RedPacket::new_valid(
            Token::NEAR,
            None,
            owner_id.clone(),
            init_balance.into(),
            split,
            distribution_mod,
            msg,
            white_list
        ).unwrap();

        self.storage_manager.start_measure_storage();
        self.add_red_packet(owner_id.clone(), public_key, near_red_packet);
        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        self.assert_storage_after(&owner_id);
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
        assert_zero_deposit(init_balance.0.into());
        self.assert_storage_before(&owner_id);
        self.assert_creation(&public_key);

        let ft_red_packet = RedPacket::new_valid(
            Token::FungibleToken,
            Some(token_id),
            owner_id.clone(),
            init_balance,
            split,
            distribution_mod,
            msg,
            white_list,
        ).unwrap();

        self.storage_manager.start_measure_storage();
        self.add_red_packet(owner_id.clone(), public_key, ft_red_packet);
        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        self.assert_storage_after(&owner_id);

        PromiseOrValue::Value(U128(0))
    }

    pub fn internal_claim_red_packet(&mut self, claimer_id: AccountId) -> U128 {
        let public_key = env::signer_account_pk();
        let mut red_packet = self.red_packets
            .get(&public_key)
            .expect(ERR_01_NO_MATCHING_RED_PACKET);
        let claim_amount = red_packet.virtual_claim(claimer_id.clone()).unwrap();

        self.storage_manager.start_measure_storage();
        self.red_packets.insert(&public_key, &red_packet);
        self.storage_manager.stop_measure_and_update_account_storage_usage(&red_packet.owner_id);

        if claim_amount.0 != 0 {
            match red_packet.token {
                Token::NEAR => {
                    transfer(claimer_id, claim_amount.0);
                },
                Token::FungibleToken => {
                    transfer_ft_with_claim_fungible_token_red_packet_callback(
                        claimer_id,
                        claim_amount,
                        red_packet.token_id.clone().unwrap(),
                        red_packet.owner_id
                    );
                }
            };
        };

        claim_amount
    }

    pub fn internal_refund(&mut self, public_key: PublicKey) -> U128 {
        let owner_id = env::predecessor_account_id();

        let mut red_packet = self.red_packets
            .get(&public_key)
            .expect(ERR_01_NO_MATCHING_RED_PACKET);
        let refund_amount = red_packet.virtual_refund(owner_id.clone()).unwrap();

        self.storage_manager.start_measure_storage();
        self.red_packets.insert(&public_key, &red_packet);
        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);

        if refund_amount.0 != 0 {
            match red_packet.token {
                Token::NEAR => {
                    transfer(owner_id, refund_amount.0);
                },
                Token::FungibleToken => {
                    transfer_ft(owner_id, refund_amount, red_packet.token_id.unwrap());
                }
            }
        }

        refund_amount
    }

    pub fn internal_remove_history(&mut self, public_key: PublicKey) {
        let owner_id = env::predecessor_account_id();

        self.storage_manager.start_measure_storage();
        self.remove_red_packet(&public_key,&owner_id,false);
        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);
    }

    pub fn internal_clear_history(&mut self) {
        let owner_id = env::predecessor_account_id();

        self.storage_manager.start_measure_storage();
        self.clear_red_packets(&owner_id,false);
        self.storage_manager.stop_measure_and_update_account_storage_usage(&owner_id);
    }
}


impl Contract {
    pub fn add_red_packet(
        &mut self,
        owner_id: AccountId,
        public_key: PublicKey,
        red_packet: RedPacket
    ) {
        let mut public_keys = self.owners.get(&owner_id).unwrap_or(HashSet::new());
        public_keys.insert(public_key.clone());
        self.owners.insert(&owner_id, &public_keys);
        self.red_packets.insert(&public_key, &red_packet);
    }

    pub fn remove_red_packet(&mut self, public_key: &PublicKey, owner_id: &AccountId, force: bool) {
        match self.red_packets.get(public_key) {
            None => {
                return;
            }
            Some(red_packet) => {
                if *owner_id != red_packet.owner_id {
                    panic!("{}", ERR_02_NO_PERMISSION_TO_RED_PACKET);
                };
                if !red_packet.is_run_out() && !force {
                    panic!("{}", ERR_03_RED_PACKET_NOT_RUN_OUT);
                };

                let mut public_keys = self.owners.get(&owner_id).unwrap();
                public_keys.remove(public_key);

                if public_keys.is_empty() {
                    self.owners.remove(owner_id);
                } else {
                    self.owners.insert(owner_id, &public_keys);
                }
                self.red_packets.remove(public_key);
            }
        };
    }

    pub fn clear_red_packets(&mut self, owner_id: &AccountId, force: bool) {
        match self.owners.get(owner_id) {
            None => {
                return;
            }
            Some(mut public_keys) => {
                public_keys.retain(|public_key| {
                    let red_packet = self.red_packets.get(public_key).unwrap();
                    let is_run_out = red_packet.is_run_out();
                    if is_run_out || force {
                        self.red_packets.remove(public_key);
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
        self.red_packets.get(public_key).is_none()
    }

    pub fn red_packet_count(&self, owner_id: &AccountId) -> (usize, usize) {
        let mut total = 0;
        let mut run_out = 0;

        match self.owners.get(owner_id) {
            None => (),
            Some(public_keys) => {
                total = public_keys.len();
                for public_key in public_keys {
                    let red_packet = self.red_packets.get(&public_key).unwrap();
                    if red_packet.is_run_out() {
                        run_out += 1;
                    };
                };
            }
        };
        (total, run_out)
    }

    pub fn all_red_packets_run_out(&self, owner_id: &AccountId) -> bool {
        let count = self.red_packet_count(owner_id);
        count.0 == count.1
    }

    pub fn assert_creation(&self, public_key: &PublicKey) {
        require!(self.unique_public_key(public_key), ERR_05_NOT_UNIQUE_PUBLIC_KEY);
    }

    pub fn assert_storage_before(&self, account_id: &AccountId) {
        self.storage_manager.assert_registration(account_id);
        self.storage_manager.assert_storage_balance(account_id);
    }

    pub fn assert_storage_after(&self, account_id: &AccountId) {
        self.storage_manager.assert_storage_balance(account_id);
    }
}