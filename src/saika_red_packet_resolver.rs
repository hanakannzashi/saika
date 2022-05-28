use near_sdk::{AccountId, PublicKey};
use near_sdk::json_types::U128;


pub trait SaikaRedPacketResolver {
    fn resolve_claim_fungible_token_red_packet(
        &mut self,
        claimer_id: AccountId,
        owner_id: AccountId,
        amount: U128,
        token_id: AccountId,
        public_key: PublicKey
    );
}

