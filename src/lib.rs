/*
* Airdrop PoC contract for the  NEAR protocol
*/
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{BlockHeight, Gas, PanicOnDefault, Promise, PromiseResult};
use near_sdk::json_types::{U128, U64, ValidAccountId};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::{json, from_slice};
use near_sdk::{AccountId, Balance, PromiseOrValue, env, near_bindgen, setup_alloc};
use near_sdk::collections::{LookupMap, UnorderedMap};
use std::collections::HashMap;
use std::fmt::Debug;


setup_alloc!();

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct companyAIRDROP {
    tasks: HashMap<AccountId, Vec<Task>>,
    tokens: UnorderedMap<AccountId, FungibleTokenMetadata>,
    records: Vec<Record>,
    
    
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Record {
    creator: AccountId,
    RECEIVERS: AccountId,
    token: AccountId,
    amount: U128,          
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Task {
    creator: AccountId,
    total_count: u32,
    amount_per_account: U128,
    token: AccountId,
    index: u32,
    deposit_tokens: U128,
    claimed_account: HashMap<AccountId, U128>,
}

#[near_bindgen]
impl companyAIRDROP {
    #[init]
    pub fn new() -> Self {
        Self {
            tokens: UnorderedMap::new(b't'),
            records: Vec::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn get_token_list(&self) -> Vec<FungibleTokenMetadata> {
        self.tokens.values().collect()
    }

    pub fn add_token(&mut self, address: AccountId) {
        assert!(self.tokens.get(&address.clone()).is_none(), "token already exist.");
        let promise = env::promise_create(address.clone(), b"ft_metadata", &json!("{}").to_string().as_bytes(), 0, 0);
        let metadata = match env::promise_result(promise) {
            PromiseResult::Successful(v) => v,
            _ => panic!("Get metadata failed."),
        };
        let metadata: FungibleTokenMetadata = from_slice(&metadata).unwrap();
        self.tokens.insert(&address, &metadata);
    }

    #[payable]
    pub fn add_task(&mut self, total_count: u32, amount_per_account: U128, token: AccountId, deposit_tokens:U128) {
        let total_amount = total_count as u128 * u128::from(amount_per_account);
    }
}

#[near_bindgen]
#[allow(unreachable_code)]
impl FungibleTokenRECEIVERS for companyAIRDROP {
    /// Callback on the NFT holers by this contract.

    fn ft_on_transfer(
        &mut self,
        airdrop_sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        if msg.is_empty() {

            self.internal_deposit(airdrop_sender_id.as_ref(), &token_in, amount.into());
            PromiseOrValue::Value(U128(0))
        } else {
            
            env::panic(b"Not Open Yet");

            let message =
                serde_json::from_str::<TokenRECEIVERSMessage>(&msg).expect("ERR_MSG_WRONG_FORMAT");
            match message {
                TokenRECEIVERSMessage::Execute {
                    reference,
                    force,
                    actions,
                } => {
                    let reference = reference.map(|x| x.to_string());
                    let out_amounts = self.internal_direct_actions(
                        token_in,
                        amount.0,
                        airdrop_sender_id.as_ref(),
                        force != 0,
                        reference
                        &actions,
                    );
                    for (token_out, amount_out) in out_amounts.into_iter() {
                        self.internal_send_tokens(airdrop_sender_id.as_ref(), &token_out, amount_out);
                    }
                    PromiseOrValue::Value(U128(0))
                }
            }
        }
    }
}


impl companyAIRDROP {
    pub(crate) fn internal_deposit(&mut self, token: AccountId, amount: Balance, sender: AccountId) {
        
    }
}



