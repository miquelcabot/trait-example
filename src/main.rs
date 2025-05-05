mod step1;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_step1() {
    let mut balances = step1::BalanceModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert_eq!(balances.balance(1), 100);
    assert_eq!(balances.balance(2), 200);
    assert_eq!(balances.balance(3), 0);
}
