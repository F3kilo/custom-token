mod balance;
mod storage;

use balance::Balances;
use candid::{CandidType, Nat, Principal};
use ic_cdk::{init, query, update};
use serde::Deserialize;
use storage::{balances, Storage};

#[derive(Debug, CandidType, Deserialize)]
pub enum TokenError {
    InsufficientBalance,
    NotAuthorized,
}

#[init]
pub fn init(init_balance: Nat) {
    let mut storage = Storage::default();

    let owner = ic_cdk::caller();
    storage.set_owner(owner);

    balances().mint(owner, init_balance);
}

#[update]
pub fn mint(to: Principal, amount: Nat) -> Result<Nat, TokenError> {
    let storage = Storage::default();
    if !storage.check_owner(ic_cdk::caller()) {
        return Err(TokenError::NotAuthorized);
    };

    let minted = balances().mint(to, amount);
    Ok(minted)
}

#[update]
pub fn transfer(to: Principal, amount: Nat) -> Result<Nat, TokenError> {
    let from = ic_cdk::caller();
    let mb_amount = balances().transfer(from, to, amount);
    let amount = mb_amount.ok_or(TokenError::InsufficientBalance)?;

    Ok(amount)
}

#[query]
pub fn balance_of(user: Option<Principal>) -> Nat {
    let user = user.unwrap_or(ic_cdk::caller());

    Balances::new(Storage::default()).balance_of(user)
}

ic_cdk::export_candid!();
