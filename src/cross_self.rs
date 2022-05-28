#[allow(unused_imports)]
use near_sdk::{AccountId, ext_contract};
use near_sdk::json_types::U128;
use near_sdk::PublicKey;


#[ext_contract(ext_self)]
trait ExtSelf {
    #[private]
    fn resolve_claim_fungible_token_red_packet(
        &mut self,
        claimer_id: AccountId,
        owner_id: AccountId,
        amount: U128,
        token_id: AccountId,
        public_key: PublicKey
    );
}