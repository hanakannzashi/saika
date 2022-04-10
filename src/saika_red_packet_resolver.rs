use near_sdk::AccountId;
use near_sdk::json_types::U128;


pub trait SaikaRedPacketResolver {
    fn claim_fungible_token_red_packet_callback(&mut self, owner_id: AccountId, amount: U128, token_id: AccountId);
}

