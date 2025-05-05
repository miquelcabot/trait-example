use std::collections::HashMap;

pub struct BalanceModule {
    balances: HashMap<u32, u32>,
}

impl BalanceModule {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }
}
