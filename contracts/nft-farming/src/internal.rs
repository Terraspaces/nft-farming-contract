use crate::*;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}
// You can define internal functions here you can see here all the functions are declared like "pub(crate)"

impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
    }

    pub(crate) fn update_claim(&mut self, account_id: AccountId, nft_contract_id: AccountId) {
        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);


        let now = env::block_timestamp() / 1000000000;

        let mut by_owner_id = self.by_owner_id.get(&account_id_and_contract_id.clone()).unwrap_or_else(|| {
            StakeInfo {
                token_ids: Vec::new(),
                stacked_reward: 0,
                updated_at: now.clone()
            }    
        });
        
        if by_owner_id.token_ids.len() == 0 {
            return;
        }
        
        let farm_spec = self.farm_specs.get(&nft_contract_id).unwrap();
        let mut rate = 0;
        for index in 0..by_owner_id.token_ids.len() {
            let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, by_owner_id.token_ids.get(index).unwrap());
            rate = rate + self.token_rate.get(&contract_and_token_id.clone()).unwrap_or(1);
        }

        by_owner_id.stacked_reward = by_owner_id.stacked_reward.checked_add(u128::from(now - by_owner_id.updated_at).checked_mul(farm_spec.reward_rate).unwrap().checked_mul(rate).unwrap()).unwrap();
        by_owner_id.updated_at = now.clone();
        self.by_owner_id.insert(&account_id_and_contract_id.clone(), &by_owner_id);
    }

    pub(crate) fn internal_remove_stake_info_for_owner(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        account_id: AccountId,
    ) {
        self.update_claim(account_id.clone(), nft_contract_id.clone());

        let account_id_and_contract_id = format!("{}{}{}", account_id.clone(), DELIMETER, nft_contract_id);
        
        let mut by_owner_id = self.by_owner_id.get(&account_id_and_contract_id).unwrap();
        for index in 0..by_owner_id.token_ids.len() {
            if by_owner_id.token_ids.get(index).unwrap() == &token_id {
                by_owner_id.token_ids.remove(index);
                break;
            }
        }
        self.by_owner_id.insert(&account_id_and_contract_id, &by_owner_id);
    }

}
