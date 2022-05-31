use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FarmSpec {
    pub reward_token_id: AccountId,
    pub reward_rate: u128,
    pub staked_count: u64,
}
