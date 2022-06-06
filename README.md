# saika red packet

Support：
* NEAR
* FungibleToken

Mod：
* Average
* Rondom

## Methods
```rust
fn create_near_red_packet(
    &mut self,
    public_key: PublicKey,
    split: usize,
    split_mod: SplitMod,
    msg: Option<String>,
    white_list: Option<HashSet<AccountId>>
);

fn claim_red_packet(&mut self, claimer_id: AccountId) -> U128;

fn refund(&mut self, public_key: PublicKey) -> U128;

fn remove_history(&mut self, public_key: PublicKey);

fn clear_history(&mut self);
```

## View Methods
```rust
fn get_red_packets_by_owner_id(&self, owner_id: AccountId) -> Vec<RedPacketView>;

fn get_pks_by_owner_id(&self, owner_id: AccountId) -> HashSet<PublicKey>;

fn get_red_packet_by_pk(&self, public_key: PublicKey) -> Option<RedPacketView>;
```

## FungibleTokenReceiver
```rust
fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
```

## StorageManagement
```rust
fn storage_deposit(
    &mut self,
    account_id: Option<AccountId>,
    registration_only: Option<bool>,
) -> StorageBalance;

fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance;

fn storage_unregister(&mut self, force: Option<bool>) -> bool;

fn storage_balance_bounds(&self) -> StorageBalanceBounds;

fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance>;
```
