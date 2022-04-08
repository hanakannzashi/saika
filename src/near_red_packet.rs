use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Serialize};
use crate::{DistributionMod, transfer};
use crate::red_packet::RedPacket;


#[derive(BorshDeserialize,BorshSerialize,Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NearRedPacket {
    red_packet: RedPacket
}

impl NearRedPacket {
    pub fn new(
        owner_id: AccountId,
        init_balance: U128,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>,
        create_timestamp: u64
    ) -> Self {
        Self {
            red_packet: RedPacket {
                owner_id,
                init_balance,
                current_balance: init_balance,
                refund_balance: U128(0),
                split,
                distribution_mod,
                msg,
                white_list,
                claimers: HashMap::new(),
                create_timestamp,
                run_out_timestamp: None
            }
        }
    }

    pub fn is_run_out(&self) -> bool {
        self.red_packet.is_run_out()
    }

    pub fn is_valid(&self) -> bool {
        self.red_packet.is_vaild()
    }

    pub fn claim(&mut self, claimer_id: AccountId) -> Result<U128, &str> {
        let claim_amount = self.red_packet.virtual_claim(claimer_id.clone())?;
        if claim_amount.0 != 0 {
            transfer(claimer_id, claim_amount.0);
        };
        Ok(claim_amount)
    }

    pub fn refund(&mut self, owner_id: AccountId) -> Result<U128, &str> {
        let refund_amount = self.red_packet.virtual_refund(owner_id.clone())?;
        if refund_amount.0 != 0 {
            transfer(owner_id, refund_amount.0);
        };
        Ok(refund_amount)
    }

    pub fn owner_id(&self) -> AccountId {
        self.red_packet.owner_id.clone()
    }

}