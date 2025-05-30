#![allow(dead_code)]

use std::collections::HashMap;

type Account = u16;
type Balance = u32;

pub struct BalancesModule {
    balances: HashMap<Account, Balance>,
}

impl BalancesModule {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: Account, amount: Balance) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: Account) -> Balance {
        *self.balances.get(&who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        from: Account,
        to: Account,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let from_balance = self.balances.get(&from).ok_or("From user does not exist")?;
        let to_balance = self.balances.get(&to).unwrap_or(&0);

        let new_from_balance = from_balance.checked_sub(amount).ok_or("Not enough funds")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}
