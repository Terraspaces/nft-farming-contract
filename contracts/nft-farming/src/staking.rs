use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeInfo {
    pub token_ids: Vec<String>,
    pub stacked_reward: u128,
    pub updated_at: u64,
}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn remove_stake_info(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        account_id: AccountId,
    ) {
        self.assert_owner();

        self.update_claim(account_id.clone(), nft_contract_id.clone());

        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);

        let mut by_owner_id = self.by_owner_id.get(&account_id_and_contract_id).unwrap();
        for index in 0..by_owner_id.token_ids.len() {
            if by_owner_id.token_ids.get(index).unwrap() == &token_id {
                by_owner_id.token_ids.remove(index);
                break;
            }
        }
        self.by_owner_id.insert(&account_id_and_contract_id, &by_owner_id);

        let mut by_contract_id = self.by_contract_id.get(&contract_and_token_id).unwrap();
        if by_contract_id == account_id{
            self.by_contract_id.remove(&contract_and_token_id);
            let mut farm_spec = self.farm_specs.get(&nft_contract_id).unwrap();
            farm_spec.staked_count = farm_spec.staked_count - 1;
            self.farm_specs.insert(&nft_contract_id, &farm_spec);
        }
    }

    #[payable]
    pub fn remove_stake_info_for_owner(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        account_id: AccountId,
    ) {
        self.assert_owner();

        self.internal_remove_stake_info_for_owner(nft_contract_id, token_id, account_id);
    }

    #[payable]
    pub fn claim_reward(
        &mut self,
        nft_contract_id: AccountId
    ) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();

        self.update_claim(account_id.clone(), nft_contract_id.clone());

        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);

        let now = 1667461564;
        let mut by_owner_id = self.by_owner_id.get(&account_id_and_contract_id.clone()).unwrap_or_else(|| {
            StakeInfo {
                token_ids: Vec::new(),
                stacked_reward: 0,
                updated_at: now,
            }    
        });

        let mut farm_spec = self.farm_specs.get(&nft_contract_id).unwrap();

        Promise::new(account_id.clone()).transfer(by_owner_id.stacked_reward.checked_div(3.try_into().unwrap()).unwrap());

        by_owner_id.stacked_reward = 0;
        by_owner_id.updated_at = now;
        self.by_owner_id.insert(&account_id_and_contract_id.clone(), &by_owner_id);
    }
    
    #[payable]
    pub fn unstake(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
    ) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();

        self.update_claim(account_id.clone(), nft_contract_id.clone());

        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);

        let mut by_owner_id = self.by_owner_id.get(&account_id_and_contract_id).unwrap();
        for index in 0..by_owner_id.token_ids.len() {
            if by_owner_id.token_ids.get(index).unwrap() == &token_id {
                by_owner_id.token_ids.remove(index);
                break;
            }
        }
        self.by_owner_id.insert(&account_id_and_contract_id, &by_owner_id);

        let mut by_contract_id = self.by_contract_id.get(&contract_and_token_id).unwrap();
        if by_contract_id == account_id{
            self.by_contract_id.remove(&contract_and_token_id);
            let mut farm_spec = self.farm_specs.get(&nft_contract_id).unwrap();
            farm_spec.staked_count = farm_spec.staked_count - 1;
            self.farm_specs.insert(&nft_contract_id, &farm_spec);
        }
    }
}
