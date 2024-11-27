use candid::{Nat, Principal};

pub trait BalancesStorage {
    fn balance_of(&self, user: Principal) -> Nat;
    fn credit(&mut self, to: Principal, amount: Nat);
    fn debit(&mut self, from: Principal, amount: Nat) -> Option<Nat>;
    fn fee_amount(&self) -> Nat;
    fn fee_recipient(&self) -> Principal;
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
        let fee = self.storage.fee_amount();
        let with_fee = amount.clone() + fee.clone();
        self.storage.debit(from, with_fee)?;
        self.storage.credit(self.storage.fee_recipient(), fee);
        self.storage.credit(to, amount.clone());
        Some(amount)
    }

    pub fn balance_of(&self, user: Principal) -> Nat {
        self.storage.balance_of(user)
    }
}
