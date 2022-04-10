use crate::constants::*;
use crate::utils::*;
use crate::errors::*;
use crate::enums::{DistributionMod, Token};

use std::collections::{HashMap, HashSet};
use near_sdk::{AccountId, env};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;


#[derive(BorshDeserialize,BorshSerialize,Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RedPacket {
    pub token: Token,
    pub token_id: Option<AccountId>,
    pub owner_id: AccountId,
    init_balance: U128,
    current_balance: U128,
    refund_balance: U128,
    split: usize,
    distribution_mod: DistributionMod,
    msg: Option<String>,
    white_list: Option<HashSet<AccountId>>,
    claimers: HashMap<AccountId, U128>,
    create_timestamp: u64,
    run_out_timestamp: Option<u64>
}

impl RedPacket {
    pub fn new_valid(
        token: Token,
        token_id: Option<AccountId>,
        owner_id: AccountId,
        init_balance: U128,
        split: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    ) -> Result<Self, &'static str> {
        let red_packet = Self {
            token,
            token_id,
            owner_id,
            init_balance,
            current_balance: init_balance,
            refund_balance: U128(0),
            split,
            distribution_mod,
            msg,
            white_list,
            claimers: HashMap::new(),
            create_timestamp: env::block_timestamp(),
            run_out_timestamp: None
        };
        if !red_packet.is_valid() {
            return Err(ERR_04_INVALID_PARAMETER);
        };
        Ok(red_packet)
    }

    pub fn is_run_out(&self) -> bool {
        self.current_balance.0 == 0
    }

    pub fn is_valid(&self) -> bool {
        match self.token {
            Token::NEAR => {
                if self.token_id.is_some() {
                    return false;
                }
            },
            Token::FungibleToken => {
                if self.token_id.is_none() {
                    return false;
                }
            }
        }

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

    pub fn virtual_claim(&mut self, claimer_id: AccountId) -> Result<U128, &'static str> {
        if self.is_run_out() {
            return Ok(U128(0));
        };

        if self.claimers.contains_key(&claimer_id) {
            return Err(ERR_07_NO_DOUBLE_CLAIM);
        }

        if let Some(wl) = &mut self.white_list {
            if !wl.contains(&claimer_id) {
                return Err(ERR_08_CLAIMER_NOT_IN_WHITE_LIST);
            } else {
                wl.remove(&claimer_id);
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

    pub fn virtual_refund(&mut self, owner_id: AccountId) -> Result<U128, &'static str> {
        if self.is_run_out() {
            return Ok(U128(0));
        };

        if self.owner_id != owner_id {
            return Err(ERR_02_NO_PERMISSION_TO_RED_PACKET);
        };

        self.refund_balance = self.current_balance;
        self.current_balance = U128(0);
        if let Some(wl) = &mut self.white_list {
            wl.clear();
        };
        self.run_out_timestamp = Some(env::block_timestamp());

        Ok(self.refund_balance)
    }
}

