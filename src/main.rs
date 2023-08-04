use std::cell::RefCell;
use std::collections::HashMap;

use candid::{candid_method, CandidType, Nat, Principal};
use custom_token::{Balances, BalancesStorage};
use ic_cdk::{init, query, update};
use serde::Deserialize;

thread_local! {
    static BALANCES: RefCell<Storage> = RefCell::new(Storage::default());
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

fn check_owner(user: Principal) -> Result<(), TokenError> {
    if user != OWNER.with(|storage| *storage.borrow()) {
        return Err(TokenError::NotAuthorized);
    }
    Ok(())
}

#[derive(Debug, Default)]
struct Storage(HashMap<Principal, Nat>);

impl Storage {
    pub fn balances(&mut self) -> Balances<&mut Self> {
        Balances::new(self)
    }
}

impl BalancesStorage for &mut Storage {
    fn balance_of(&self, user: Principal) -> Nat {
        self.0.get(&user).cloned().unwrap_or_default()
    }

    fn credit(&mut self, to: Principal, amount: Nat) {
        *self.0.entry(to).or_default() += amount;
    }

    fn debit(&mut self, from: Principal, amount: Nat) -> Option<Nat> {
        let balance = self.0.get_mut(&from)?;

        if *balance < amount {
            return None;
        }

        *balance -= amount.clone();

        Some(amount)
    }
}

#[derive(Debug, CandidType, Deserialize)]
pub enum TokenError {
    InsufficientBalance,
    NotAuthorized,
}

#[init]
#[candid_method(init)]
fn init(init_balance: Nat) {
    let owner = ic_cdk::caller();
    OWNER.with(|storage| *storage.borrow_mut() = owner);

    BALANCES.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut balances = storage.balances();
        balances.mint(owner, init_balance);
    })
}

#[update]
#[candid_method(update)]
fn mint(to: Principal, amount: Nat) -> Result<Nat, TokenError> {
    check_owner(ic_cdk::caller())?;

    let amount = BALANCES.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut balances = storage.balances();
        balances.mint(to, amount)
    });
    Ok(amount)
}

#[update]
#[candid_method(update)]
fn transfer(to: Principal, amount: Nat) -> Result<Nat, TokenError> {
    let from = ic_cdk::caller();
    let mb_amount = BALANCES.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut balances = storage.balances();
        balances.transfer(from, to, amount)
    });

    let amount = mb_amount.ok_or(TokenError::InsufficientBalance)?;
    Ok(amount)
}

#[query]
#[candid_method(query)]
fn balance_of(user: Option<Principal>) -> Nat {
    let user = user.unwrap_or(ic_cdk::caller());

    BALANCES.with(|storage| {
        let mut storage = storage.borrow_mut();
        let balances = storage.balances();
        balances.balance_of(user)
    })
}

candid::export_service!();

fn main() {
    println!("{}", __export_service());
}
