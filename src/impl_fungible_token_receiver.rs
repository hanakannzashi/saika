use crate::Contract;
use crate::errors::*;
use crate::enums::DistributionMod;

use std::collections::HashSet;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{AccountId, PromiseOrValue, near_bindgen, serde_json, env, PublicKey};
use near_sdk::json_types::U128;
use near_sdk::serde::Deserialize;


#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        if msg.is_empty() {
            PromiseOrValue::Value(amount)
        } else {
            let receiver_message = serde_json::from_str::<ReceiverMessage>(msg.as_str())
                .expect(ERR_11_WRONG_RECEIVER_MESSAGE);
            match receiver_message {
                ReceiverMessage::FungibleTokenRedPacket {
                    public_key,
                    init_copies,
                    distribution_mod,
                    msg,
                    white_list
                } => {
                    self.internal_create_fungible_token_red_packet(
                        env::predecessor_account_id(),
                        sender_id,
                        amount,
                        public_key,
                        init_copies,
                        distribution_mod,
                        msg,
                        white_list
                    )
                }
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum ReceiverMessage {
    FungibleTokenRedPacket {
        public_key: PublicKey,
        init_copies: usize,
        distribution_mod: DistributionMod,
        msg: Option<String>,
        white_list: Option<HashSet<AccountId>>
    }
}
