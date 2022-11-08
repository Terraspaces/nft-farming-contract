use crate::*;

/// approval callbacks from NFT Contracts

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingArgs {
    pub staking_status: String,
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: ValidAccountId,
        approval_id: u64,
        msg: String,
    );
}

//I already adjusted this function and you can see if someone try to approve his nft to this contract,
//This contract consider it as staking and transfer the nft from owner to this contract and register info.

#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    /// where we add the sale because we know nft owner can only call nft_approve

    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: ValidAccountId,
        approval_id: u64,
        msg: String,
    ) {
        // enforce cross contract call and owner_id is signer
        assert!(
            false,
            "Staking is not allowed at the moment!"
        );

        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        assert_ne!(
            nft_contract_id, signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(
            owner_id.as_ref(),
            &signer_id,
            "owner_id should be signer_id"
        );

        assert!(
            self.farm_specs.keys_as_vector().to_vec().contains(&nft_contract_id.clone()),
            "Not supported farm"
        );

        // // enforce signer's storage is enough to cover + 1 more sale

        // let storage_amount = self.storage_amount().0;
        // let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
        // let signer_storage_required = (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount;
        // assert!(
        //     owner_paid_storage >= signer_storage_required,
        //     "Insufficient storage paid: {}, for {} sales at {} rate of per sale",
        //     owner_paid_storage, signer_storage_required / STORAGE_PER_SALE, STORAGE_PER_SALE
        // );

        let StakingArgs { staking_status } =
            near_sdk::serde_json::from_str(&msg).expect("Not valid StakingArgs");

        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let account_id_and_contract_id = format!("{}{}{}", owner_id.to_string(), DELIMETER, nft_contract_id);


        self.update_claim(owner_id.to_string(), nft_contract_id.clone());

        let mut by_owner_id = self.by_owner_id.get(&account_id_and_contract_id.clone()).unwrap_or_else(|| {
            StakeInfo {
                token_ids: Vec::new(),
                stacked_reward: 0,
                updated_at: env::block_timestamp() / 1000000000
            }    
        });

        by_owner_id.token_ids.push(token_id.clone());
        self.by_owner_id.insert(&account_id_and_contract_id.clone(), &by_owner_id);

        let mut by_contract_id = self.by_contract_id.get(&contract_and_token_id).unwrap_or_else(|| {
            "".to_string()
        });

        if by_contract_id != "".to_string() {
            self.internal_remove_stake_info_for_owner(owner_id.to_string(), nft_contract_id.clone(), token_id.clone());
            let mut farm_spec = self.farm_specs.get(&nft_contract_id.clone()).unwrap();
            farm_spec.staked_count = farm_spec.staked_count - 1;
            self.farm_specs.insert(&nft_contract_id, &farm_spec);
        }

        self.by_contract_id.insert(&contract_and_token_id, &owner_id.as_ref());

        let mut farm_spec = self.farm_specs.get(&nft_contract_id).unwrap();
        farm_spec.staked_count = farm_spec.staked_count + 1;
        self.farm_specs.insert(&nft_contract_id, &farm_spec);

        if self.staker_ids.contains(owner_id.as_ref()) == false {
            self.staker_ids.insert(owner_id.as_ref());
        }
    }
}
