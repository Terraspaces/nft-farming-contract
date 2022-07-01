use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey,
    CryptoHash, Gas, PanicOnDefault, Promise,
};
use std::cmp::min;
use std::collections::HashMap;
use std::convert::TryInto;
use crate::external::*;
use crate::internal::*;
use crate::staking::*;
use crate::farmspec::*;
use near_sdk::env::STORAGE_PRICE_PER_BYTE;

mod external;
mod internal;
mod nft_callbacks;
mod staking;
mod staking_views;
mod farmspec;

near_sdk::setup_alloc!();

//I already adjusted this contract and it has basic info for staking.

// TODO check seller supports storage_deposit at ft_token_id they want to post sale in

const GAS_FOR_FT_TRANSFER: Gas = 5_000_000_000_000;
/// greedy max Tgas for resolve_purchase
const GAS_FOR_ROYALTIES: Gas = 115_000_000_000_000;
const GAS_FOR_NFT_TRANSFER: Gas = 15_000_000_000_000;
const GAS_FOR_NFT_APPROVE: Gas = 30_000_000_000_000;
const BID_HISTORY_LENGTH_DEFAULT: u8 = 1;
const NO_DEPOSIT: Balance = 0;
const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;
static DELIMETER: &str = "||";

pub type TokenId = String;
pub type ContractAndTokenId = String;
pub type AccountIdAndContract = String;

// TODO: Capital U128
pub type Payout = HashMap<AccountId, U128>;
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalanceBounds {
    pub min: U128,
    pub max: Option<U128>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub staker_ids: UnorderedSet<AccountId>,
    pub farm_specs: UnorderedMap<AccountId, FarmSpec>,
    pub by_owner_id: LookupMap<AccountIdAndContract, StakeInfo>,
    pub by_contract_id: LookupMap<ContractAndTokenId, AccountId>,
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub token_rate: LookupMap<ContractAndTokenId, u128>,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    StakerIds,
    FarmSpecs,
    ByOwnerId,
    ByContractId,
    StorageDeposits,
    TokenRate,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId
    ) -> Self {
        let mut this = Self {
            owner_id: owner_id.into(),
            staker_ids: UnorderedSet::new(StorageKey::StakerIds),
            farm_specs: UnorderedMap::new(StorageKey::FarmSpecs),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
            by_contract_id: LookupMap::new(StorageKey::ByContractId),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            token_rate: LookupMap::new(StorageKey::TokenRate),
        };
        this.farm_specs.insert(&"terraspaces.near".to_string(), &FarmSpec{
            reward_token_id: "usn".to_string(),
            reward_rate: 192901234568,
            staked_count: 0
        });
        
        this
    }

    pub fn get_farm_contract_ids(&self) -> Vec<AccountId> {
        self.farm_specs.keys_as_vector().to_vec()
    }

    pub fn get_staker_ids(&self, from_index: U64, limit: u64) -> Vec<AccountId> {
        let mut tmp = vec![];
        let start = u64::from(from_index);
        let end = min(start + limit, self.staker_ids.len());
        let staker_ids_vec = self.staker_ids.to_vec();
        for i in start..end {
            let id = staker_ids_vec
            .get(i as usize)
            .unwrap().to_string();
            
            tmp.push(
                id
            );
        }
        tmp
    }

    #[payable]
    pub fn set_token_rate(&mut self, nft_contract_id: AccountId, token_id: String, rate: U128){
        self.assert_owner();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        self.token_rate.insert(&contract_and_token_id, &u128::from(rate));
    }

    pub fn get_token_rate(&self, nft_contract_id: AccountId, token_id: String) -> U128{
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        U128::from(self.token_rate.get(&contract_and_token_id).unwrap_or(0))
    }

    #[payable]
    pub fn insert_farm_spec(&mut self, nft_contract_id: AccountId, reward_token_id: AccountId, reward_rate: U128){
        self.assert_owner();
        self.farm_specs.insert(&nft_contract_id, &FarmSpec{
            reward_token_id: reward_token_id.clone(),
            reward_rate: u128::from(reward_rate.clone()),
            staked_count: 0
        });
    }

    #[payable]
    pub fn remove_farm_spec(&mut self, nft_contract_id: AccountId){
        self.assert_owner();
        self.farm_specs.remove(&nft_contract_id);
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<ValidAccountId>) {
        let storage_account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(env::predecessor_account_id);
        let deposit = env::attached_deposit();
        assert!(
            deposit >= STORAGE_PER_SALE,
            "Requires minimum deposit of {}",
            STORAGE_PER_SALE
        );
        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        balance += deposit;
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    #[payable]
    pub fn storage_withdraw(&mut self) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);
        let len = 0u128;
        let diff = len * STORAGE_PER_SALE;
        amount -= diff;
        if amount > 0 {
            Promise::new(owner_id.clone()).transfer(amount);
        }
        if diff > 0 {
            self.storage_deposits.insert(&owner_id, &diff);
        }
    }

    /// views


    pub fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: U128(STORAGE_PER_SALE),
            max: None,
        }
    }

    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }

    pub fn storage_balance_of(&self, account_id: ValidAccountId) -> U128 {
        U128(self.storage_deposits.get(account_id.as_ref()).unwrap_or(0))
    }

    /// deprecated

    pub fn storage_paid(&self, account_id: ValidAccountId) -> U128 {
        U128(self.storage_deposits.get(account_id.as_ref()).unwrap_or(0))
    }

    pub fn storage_amount(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }
}
