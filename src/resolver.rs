use crate::{Contract};
use crate::errors::*;
use crate::utils::transfer_ft;

use near_sdk::{AccountId, is_promise_success, log, near_bindgen};
use near_sdk::json_types::U128;


trait Resolver {
    fn claim_fungible_token_red_packet_callback(&mut self, owner_id: AccountId, amount: U128, token_id: AccountId);
}

#[near_bindgen]
impl Resolver for Contract {
    #[private]
    fn claim_fungible_token_red_packet_callback(&mut self, owner_id: AccountId, amount: U128, token_id: AccountId) {
        if !is_promise_success() {
            log!("{}", ERR_09_CLAIM_FT_RED_PACKET_FAILED);
            log!("Refund balance to red packet owner, owner id: {}, amount: {}, token id: {}", owner_id, amount.0, token_id);
            transfer_ft(owner_id, amount, token_id);
        } else {
            log!("Success claim fungible token red packet, amount: {}, token id: {}", amount.0, token_id);
        }
    }
}