use crate::*;
extern crate chrono;
// use chrono::prelude::*;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use std::time::SystemTime;
use std::convert::TryFrom;

// we have to define view functions here.
// but maybe I already adjusted and you would add little funtions.

#[near_bindgen]
impl Contract {
    /// views
    pub fn get_supply_stakers(&self) -> U64 {
        U64(self.staker_ids.len())
    }

    pub fn get_farm_spec(&self, nft_contract_id: AccountId) -> FarmSpec {
        self.farm_specs.get(&nft_contract_id.clone()).unwrap()
    }

    pub fn get_supply_farm(&self, nft_contract_id: AccountId) -> U64 {
        U64(self.farm_specs.get(&nft_contract_id.clone()).unwrap().staked_count)
    }

    pub fn get_supply_by_owner_id(&self, account_id: AccountId, nft_contract_id: AccountId) -> U64 {
        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);

        let by_owner_id = self.by_owner_id.get(&account_id_and_contract_id);
        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.token_ids.len().try_into().unwrap())
        } else {
            U64(0)
        }
    }

    pub fn get_claim_amount_by_owner_id(&self, account_id: AccountId, nft_contract_id: AccountId) -> U128 {
        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);

        let now = 1667461564;

        let by_owner_id = self.by_owner_id.get(&account_id_and_contract_id);
        if let Some(by_owner_id) = by_owner_id {
            let farm_spec = self.farm_specs.get(&nft_contract_id).unwrap();
            let mut rate = 0;
            for index in 0..by_owner_id.token_ids.len() {
                let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, by_owner_id.token_ids.get(index).unwrap());
                rate = rate + self.token_rate.get(&contract_and_token_id.clone()).unwrap_or(1);
            }
            U128(by_owner_id.stacked_reward.checked_add(u128::from(now - by_owner_id.updated_at).checked_mul(farm_spec.reward_rate).unwrap().checked_mul(rate).unwrap()).unwrap().checked_div(3.try_into().unwrap()).unwrap())
        } else {
            U128(0)
        }
    }

    pub fn get_staking_informations_by_owner_id(
        &self,
        account_id: AccountId,
        nft_contract_id: AccountId,
    ) -> StakeInfo {
        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);

        let by_owner_id = self.by_owner_id.get(&account_id_and_contract_id).unwrap_or(StakeInfo {
            token_ids: vec![],
            stacked_reward: 0,
            updated_at: 1667461564
        });
        by_owner_id
    }
}
