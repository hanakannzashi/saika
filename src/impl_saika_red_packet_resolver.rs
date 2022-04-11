use crate::Contract;
use crate::errors::*;
use crate::utils::transfer_ft;
use crate::saika_red_packet_resolver::SaikaRedPacketResolver;

use near_sdk::{AccountId, is_promise_success, log, near_bindgen, PublicKey};
use near_sdk::json_types::U128;


#[near_bindgen]
impl SaikaRedPacketResolver for Contract {
    #[private]
    fn claim_fungible_token_red_packet_callback(
        &mut self,
        claimer_id: AccountId,
        owner_id: AccountId,
        amount: U128,
        token_id: AccountId,
        public_key: PublicKey
    ) {
        if !is_promise_success() {
            log!("{}", ERR_09_CLAIM_FT_RED_PACKET_FAILED);
            if let Some(mut red_packet) = self.red_packets.get(&public_key) {
                red_packet.failed_claimer(claimer_id, amount);
                self.red_packets.insert(&public_key, &red_packet);
            };
            log!("Refund balance to red packet owner, owner id: {}, amount: {}, token id: {}", owner_id, amount.0, token_id);
            transfer_ft(owner_id, amount, token_id);
        } else {
            log!("Success claim fungible token red packet, amount: {}, token id: {}", amount.0, token_id);
        }
    }
}