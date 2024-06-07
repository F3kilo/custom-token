mod balance;

use std::cell::RefCell;
use std::collections::HashMap;

use balance::{Balances, BalancesStorage};
use candid::{CandidType, Nat, Principal};
use ic_cdk::{init, query, update};
use serde::Deserialize;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, Nat>> = RefCell::new(HashMap::default());
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

#[derive(Default)]
struct Storage {}

impl Storage {
    pub fn set_owner(&mut self, user: Principal) {
        OWNER.with(|storage| *storage.borrow_mut() = user);
    }

    pub fn get_owner(&self) -> Principal {
        OWNER.with(|storage| *storage.borrow())
    }

    pub fn check_owner(&self, user: Principal) -> Result<(), TokenError> {
        if user != self.get_owner() {
            return Err(TokenError::NotAuthorized);
        }
        Ok(())
    }
}

impl BalancesStorage for Storage {
    fn balance_of(&self, user: Principal) -> Nat {
        BALANCES.with(|map| map.borrow().get(&user).cloned().unwrap_or_default())
    }

    fn credit(&mut self, to: Principal, amount: Nat) {
        BALANCES.with(|map| {
            let mut map = map.borrow_mut();
            *map.entry(to).or_default() += amount;
        });
    }

    fn debit(&mut self, from: Principal, amount: Nat) -> Option<Nat> {
        BALANCES.with(|map| {
            let mut map = map.borrow_mut();
            let balance = map.entry(from).or_default();

            if *balance < amount {
                return None;
            }

            *balance -= amount;

            Some(balance.clone())
        })
    }
}

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

    Balances::new(storage).mint(owner, init_balance);
}

#[update]
pub fn mint(to: Principal, amount: Nat) -> Result<Nat, TokenError> {
    let storage = Storage::default();
    storage.check_owner(ic_cdk::caller())?;

    let minted = Balances::new(storage).mint(to, amount);
    Ok(minted)
}

#[update]
pub fn transfer(to: Principal, amount: Nat) -> Result<Nat, TokenError> {
    let storage = Storage::default();
    let mut balances = Balances::new(storage);

    let from = ic_cdk::caller();
    let mb_amount = balances.transfer(from, to, amount);
    let amount = mb_amount.ok_or(TokenError::InsufficientBalance)?;

    Ok(amount)
}

#[query]
pub fn balance_of(user: Option<Principal>) -> Nat {
    let user = user.unwrap_or(ic_cdk::caller());

    Balances::new(Storage::default()).balance_of(user)
}

ic_cdk::export_candid!();
