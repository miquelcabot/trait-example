mod step1;
mod step2;
mod step3;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_step1() {
    let mut balances = step1::BalancesModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert_eq!(balances.balance(1), 100);
    assert_eq!(balances.balance(2), 200);
    assert_eq!(balances.balance(3), 0);

    assert!(balances.transfer(1, 2, 50).is_ok());

    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);

    assert!(balances.transfer(1, 2, 100).is_err());

    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);
}

#[test]
fn test_step2() {
    let mut balances = step2::BalancesModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert_eq!(balances.balance(1), 100);
    assert_eq!(balances.balance(2), 200);
    assert_eq!(balances.balance(3), 0);

    assert!(balances.transfer(1, 2, 50).is_ok());

    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);

    assert!(balances.transfer(1, 2, 100).is_err());

    assert_eq!(balances.balance(1), 50);
    assert_eq!(balances.balance(2), 250);
}
