use crate::ft_red_packet::{FungibleTokenRedPacket, FungibleTokenRedPacketMessage};
use crate::near_red_packet::NearRedPacket;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize,Deserialize};
use near_sdk::{AccountId, BorshStorageKey};
use near_sdk::json_types::U128;


#[derive(BorshStorageKey,BorshDeserialize,BorshSerialize)]
pub enum StorageKey {
    SaikaRedPackets,
    Owners,
    DynamicStorageManager
}

#[derive(BorshDeserialize,BorshSerialize,Serialize,Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum DistributionMod {
    Average,
    Random
}

#[derive(BorshDeserialize,BorshSerialize,Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum SaikaRedPacket {
    NearRedPacket(NearRedPacket),
    FungibleTokenRedPacket(FungibleTokenRedPacket)
}

impl SaikaRedPacket {
    pub fn is_run_out(&self) -> bool {
        match self {
            SaikaRedPacket::NearRedPacket(near_red_packet) => {
                near_red_packet.is_run_out()
            },
            SaikaRedPacket::FungibleTokenRedPacket(ft_red_packet) => {
                ft_red_packet.is_run_out()
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            SaikaRedPacket::NearRedPacket(near_red_packet) => {
                near_red_packet.is_valid()
            },
            SaikaRedPacket::FungibleTokenRedPacket(ft_red_packet) => {
                ft_red_packet.is_valid()
            }
        }
    }

    pub fn claim(&mut self, claimer_id : AccountId) -> Result<U128, &str> {
        match self {
            SaikaRedPacket::NearRedPacket(near_red_packet) => {
                near_red_packet.claim(claimer_id)
            },
            SaikaRedPacket::FungibleTokenRedPacket(ft_red_packet) => {
                ft_red_packet.claim(claimer_id)
            }
        }
    }

    pub fn refund(&mut self, owner_id: AccountId) -> Result<U128, &str> {
        match self {
            SaikaRedPacket::NearRedPacket(near_red_packet) => {
                near_red_packet.refund(owner_id)
            },
            SaikaRedPacket::FungibleTokenRedPacket(ft_red_packet) => {
                ft_red_packet.refund(owner_id)
            }
        }
    }

    pub fn owner_id(&self) -> AccountId {
        match self {
            SaikaRedPacket::NearRedPacket(near_red_packet) => {
                near_red_packet.owner_id()
            },
            SaikaRedPacket::FungibleTokenRedPacket(ft_red_packet) => {
                ft_red_packet.owner_id()
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ReceiverMessage {
    FungibleTokenRedPacketMessage(FungibleTokenRedPacketMessage)
}