use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId, env};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use crate::constant::*;
use crate::DistributionMod;
use crate::utils::*;
use crate::errors::*;


#[derive(BorshDeserialize,BorshSerialize,Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RedPacket {
    pub owner_id: AccountId,
    pub init_balance: U128,
    pub current_balance: U128,
    pub refund_balance: U128,
    pub split: usize,
    pub distribution_mod: DistributionMod,
    pub msg: Option<String>,
    pub white_list: Option<HashSet<AccountId>>,
    pub claimers: HashMap<AccountId, U128>,
    pub create_timestamp: u64,
    pub run_out_timestamp: Option<u64>
}

impl RedPacket {
    pub fn is_run_out(&self) -> bool {
        self.current_balance.0 == 0
    }

    pub fn is_vaild(&self) -> bool {
        if self.split == 0 || self.split > MAX_RED_PACKET_SPLIT {
            return false;
        }
        if self.init_balance.0 < self.split as u128 {
            return false;
        }
        if let Some(msg) = &self.msg {
            if msg.len() > MAX_RED_PACKET_MSG_LEN {
                return false;
            }
        }
        if let Some(wl) = &self.white_list {
            if wl.len() != self.split as usize {
                return false;
            }
        }
        true
    }

    pub fn virtual_claim(&mut self, claimer_id: AccountId) -> Result<U128, &str> {
        if self.is_run_out() {
            return Ok(U128(0));
        };

        if self.claimers.contains_key(&claimer_id) {
            return Err(ERR_07_NO_DOUBLE_CLAIM);
        }

        match &mut self.white_list {
            None => (),
            Some(wl) => {
                if !wl.contains(&claimer_id) {
                    return Err(ERR_08_CLAIMER_NOT_IN_WHITE_LIST);
                } else {
                    wl.remove(&claimer_id);
                }
            }
        };

        let claim_amount: u128;

        match self.distribution_mod {
            DistributionMod::Average => {
                claim_amount = average_sub(
                    self.current_balance.0,
                    self.split - self.claimers.len()
                );
            },
            DistributionMod::Random => {
                claim_amount = random_sub(
                    self.current_balance.0,
                    self.split - self.claimers.len(),
                    None
                );
            }
        };

        self.current_balance.0 -= claim_amount;
        self.claimers.insert(claimer_id, claim_amount.into());

        if self.is_run_out() {
            self.run_out_timestamp = Some(env::block_timestamp());
        };

        Ok(claim_amount.into())
    }

    pub fn virtual_refund(&mut self, owner_id: AccountId) -> Result<U128, &str> {
        if self.is_run_out() {
            return Ok(U128(0));
        };

        if self.owner_id != owner_id {
            return Err(ERR_02_NO_PERMISSION_TO_RED_PACKET);
        };

        self.refund_balance = self.current_balance;
        self.current_balance = U128(0);
        match &mut self.white_list {
            None => (),
            Some(wl) => {
                wl.clear();
            }
        };
        self.run_out_timestamp = Some(env::block_timestamp());
        Ok(self.refund_balance)
    }

}