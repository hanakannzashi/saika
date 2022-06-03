use crate::enums::{SplitMod, Token};
use crate::RedPacket;

use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId, PublicKey};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RedPacketView {
    pub public_key: PublicKey,
    pub token: Token,
    pub token_id: Option<AccountId>,
    pub owner_id: AccountId,
    pub init_balance: U128,
    pub current_balance: U128,
    pub refunded_balance: U128,
    pub init_split: usize,
    pub current_split: usize,
    pub split_mod: SplitMod,
    pub msg: Option<String>,
    pub white_list: Option<HashSet<AccountId>>,
    pub claimers: HashMap<AccountId, U128>,
    pub failed_claimers: HashMap<AccountId, U128>,
    pub create_timestamp: U64,
    pub run_out_timestamp: Option<U64>,
    pub is_run_out: bool
}

pub fn parse_red_packet_view(red_packet: RedPacket, public_key: PublicKey) -> RedPacketView {
    RedPacketView {
        public_key,
        is_run_out: red_packet.is_run_out(),
        token: red_packet.token,
        token_id: red_packet.token_id,
        owner_id: red_packet.owner_id,
        init_balance: red_packet.init_balance,
        current_balance: red_packet.current_balance,
        refunded_balance: red_packet.refunded_balance,
        init_split: red_packet.init_split,
        current_split: red_packet.current_split,
        split_mod: red_packet.split_mod,
        msg: red_packet.msg,
        white_list: red_packet.white_list,
        claimers: red_packet.claimers,
        failed_claimers: red_packet.failed_claimers,
        create_timestamp:red_packet.create_timestamp,
        run_out_timestamp: red_packet.run_out_timestamp
    }
}