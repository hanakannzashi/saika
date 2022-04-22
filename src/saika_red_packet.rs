use crate::enums::SplitMod;
use crate::red_packet_view::RedPacketView;

use std::collections::HashSet;
use near_sdk::{AccountId, PublicKey};
use near_sdk::json_types::U128;


pub trait SaikaRedPacket {
    fn create_near_red_packet(
        &mut self,
        public_key: PublicKey,
        split: usize,
        split_mod: SplitMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    );

    fn claim_red_packet(&mut self, claimer_id: AccountId) -> U128;

    fn refund(&mut self, public_key: PublicKey) -> U128;

    fn remove_history(&mut self, public_key: PublicKey);

    fn clear_history(&mut self);

    fn get_red_packets_by_owner_id(&self, owner_id: AccountId) -> Vec<RedPacketView>;

    fn get_pks_by_owner_id(&self, owner_id: AccountId) -> HashSet<PublicKey>;

    fn get_red_packet_by_pk(&self, public_key: PublicKey) -> Option<RedPacketView>;
}