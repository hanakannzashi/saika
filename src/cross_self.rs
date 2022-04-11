#[allow(unused_imports)]
use near_sdk::{AccountId, ext_contract};
use near_sdk::json_types::U128;
use near_sdk::PublicKey;


#[ext_contract(ext_self)]
pub trait SaikaRedPacketResolver {
    fn claim_fungible_token_red_packet_callback(
        &mut self,
        claimer_id: AccountId,
        owner_id: AccountId,
        amount: U128,
        token_id: AccountId,
        public_key: PublicKey
    );
}