use crate::enums::{SplitMod, Token};
use crate::RedPacket;

use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId, PublicKey, serde_json};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::{json};
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

pub fn parse_red_packet_view(red_packet: &RedPacket, public_key: &PublicKey) -> RedPacketView {
    let mut tmp = json!(red_packet);
    tmp.as_object_mut()
        .unwrap()
        .insert("public_key".into(), json!(public_key));
    let run_out_timestamp = tmp.as_object()
        .unwrap()
        .get("run_out_timestamp")
        .unwrap();
    if run_out_timestamp.is_null() {
        tmp.as_object_mut().unwrap().insert("is_run_out".into(), json!(false));
    } else {
        tmp.as_object_mut().unwrap().insert("is_run_out".into(), json!(true));
    };
    serde_json::from_value::<RedPacketView>(tmp).unwrap()
}