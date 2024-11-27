use std::cell::RefCell;
use std::collections::HashMap;

use candid::{Nat, Principal};

use crate::balance::{Balances, BalancesStorage};

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, Nat>> = RefCell::new(HashMap::default());
    static OWNER: RefCell<Principal> = const { RefCell::new(Principal::anonymous()) };
}

#[derive(Default)]
pub struct Storage {}

impl Storage {
    pub fn set_owner(&mut self, user: Principal) {
        OWNER.with(|storage| *storage.borrow_mut() = user);
    }

    pub fn get_owner(&self) -> Principal {
        OWNER.with(|storage| *storage.borrow())
    }

    pub fn check_owner(&self, user: Principal) -> bool {
        user == self.get_owner()
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

pub fn balances() -> Balances<Storage> {
    Balances::new(Storage::default())
}
