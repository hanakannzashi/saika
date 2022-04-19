use crate::enums::{DistributionMod, Token};
use crate::RedPacket;

use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId, PublicKey, serde_json, Timestamp};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RedPacketView {
    pub public_key: Option<PublicKey>,
    pub token: Token,
    pub token_id: Option<AccountId>,
    pub owner_id: AccountId,
    pub init_balance: U128,
    pub current_balance: U128,
    pub refunded_balance: U128,
    pub init_split: usize,
    pub current_split: usize,
    pub distribution_mod: DistributionMod,
    pub msg: Option<String>,
    pub white_list: Option<HashSet<AccountId>>,
    pub claimers: HashMap<AccountId, U128>,
    pub failed_claimers: HashMap<AccountId, U128>,
    pub create_timestamp: Timestamp,
    pub run_out_timestamp: Option<Timestamp>
}

impl From<RedPacket> for RedPacketView {
    fn from(red_packet: RedPacket) -> Self {
        serde_json::from_value(json!(red_packet)).unwrap()
    }
}