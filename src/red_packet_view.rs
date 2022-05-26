use crate::enums::{SplitMod, Token};
use crate::RedPacket;

use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId, PublicKey, serde_json};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::{json, Value};
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
    pub run_out_timestamp: Option<U64>
}

pub fn parse_red_packet_view(red_packet: &RedPacket, public_key: &PublicKey) -> RedPacketView {
    let mut tmp = json!(red_packet);

    let create_timestamp = tmp.as_object_mut()
        .unwrap()
        .get("create_timestamp")
        .unwrap();

    let create_timestamp = match create_timestamp {
        Value::Number(number) => {
            Value::String(number.to_string())
        }
        _ => unreachable!()
    };

    let run_out_timestamp = tmp.as_object_mut()
        .unwrap()
        .get("run_out_timestamp")
        .unwrap();

    let run_out_timestamp = match run_out_timestamp {
        Value::Number(number) => {
            Value::String(number.to_string())
        },
        Value::Null => Value::Null,
        _ => unreachable!()
    };

    tmp.as_object_mut()
        .unwrap()
        .insert("public_key".into(), json!(public_key));

    tmp.as_object_mut()
        .unwrap()
        .insert("create_timestamp".into(), create_timestamp);

    tmp.as_object_mut()
        .unwrap()
        .insert("run_out_timestamp".into(), run_out_timestamp);

    serde_json::from_value::<RedPacketView>(tmp).unwrap()
}