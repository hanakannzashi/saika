use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{AccountId, PromiseOrValue, near_bindgen, serde_json, env};
use near_sdk::json_types::U128;
use crate::{Contract, ReceiverMessage};
use crate::ft_red_packet::FungibleTokenRedPacketMessage;


#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        if msg.is_empty() {
            PromiseOrValue::Value(amount)
        } else {
            let msg = serde_json::from_str::<ReceiverMessage>(msg.as_str()).unwrap();
            match msg {
                ReceiverMessage::FungibleTokenRedPacketMessage(ft_red_packet_message) => {
                    self.handle_fungible_token_red_packet_message(sender_id, amount, ft_red_packet_message)
                }
            }
        }
    }
}

impl Contract {
    pub fn handle_fungible_token_red_packet_message(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        ft_red_packet_message: FungibleTokenRedPacketMessage
    ) -> PromiseOrValue<U128> {
        let token_id = env::predecessor_account_id();
        let owner_id = sender_id;
        let init_balance = amount;
        let public_key = ft_red_packet_message.public_key.clone();
        let split = ft_red_packet_message.split;
        let distribution_mod = ft_red_packet_message.distribution_mod.clone();
        let msg = ft_red_packet_message.msg.clone();
        let white_list = ft_red_packet_message.white_list.clone();

        self.internal_create_fungible_token_red_packet(
            token_id,
            owner_id,
            init_balance,
            public_key,
            split,
            distribution_mod,
            msg,
            white_list
        )
    }

}