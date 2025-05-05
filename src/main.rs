mod step1;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_step1() {
    let balances = step1::BalanceModule::new();
}
