use bank::SavingAccount;

#[test]
fn should_have_a_starting_balance_of_0() {
    let account = SavingAccount::new();
    assert_eq!(account.get_balance(), 0);
}
