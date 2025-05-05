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

#[test]
fn test_step_3() {
    let mut voting = step3::VotingModule::new();
    voting.vote(1, 0, true);
    voting.vote(2, 0, false);

    assert!(voting.get_vote(1, 0) == true);
    assert!(voting.get_vote(2, 0) == false);

    assert!(voting.get_vote(1, 1) == false);
    assert!(voting.get_vote(2, 1) == false);
}

// // Account ID in step 2 and step 3 are defined differently!
// fn wont_work() {
// 	let user_1 = 1;
// 	let user_2 = 2;
// 	let mut balances = step2::BalancesModule::new();
// 	let mut voting = step3::VotingModule::new();

// 	balances.set_balance(user_1, 100);
// 	balances.set_balance(user_2, 200);

// 	voting.vote(user_1, 0, true);
// 	voting.vote(user_2, 0, false);
// }
