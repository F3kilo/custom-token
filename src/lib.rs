use candid::{Nat, Principal};

pub trait BalancesStorage {
    fn balance_of(&self, user: Principal) -> Nat;
    fn credit(&mut self, to: Principal, amount: Nat);
    fn debit(&mut self, from: Principal, amount: Nat) -> Option<Nat>;
}

pub struct Balances<S> {
    storage: S,
}

impl<S: BalancesStorage> Balances<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn mint(&mut self, to: Principal, amount: Nat) -> Nat {
        self.storage.credit(to, amount.clone());
        amount
    }

    pub fn transfer(&mut self, from: Principal, to: Principal, amount: Nat) -> Option<Nat> {
        self.storage.debit(from, amount.clone())?;
        self.storage.credit(to, amount.clone());
        Some(amount)
    }

    pub fn balance_of(&self, user: Principal) -> Nat {
        self.storage.balance_of(user)
    }
}
