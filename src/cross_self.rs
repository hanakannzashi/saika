#[allow(unused_imports)]
use near_sdk::{AccountId, ext_contract};
use near_sdk::json_types::U128;


#[ext_contract(ext_self)]
pub trait SaikaResolver {
    fn claim_fungible_token_red_packet_callback(&mut self, owner_id: AccountId, amount: U128, token_id: AccountId);
}