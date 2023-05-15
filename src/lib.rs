use std::format;

pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }
    pub fn get_balance(&self) -> i32 {
        self.balance
    }
    pub fn deposite(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot deposit negative amount {}", amount);
        }
        self.balance += amount;
    }
    pub fn transfer(&mut self, acc_number: u32, amount: i32) -> Result<String, String> {
        if self.balance < amount {
            return Err(format!("Insufficient funds to transfer {}", amount));
        }
        self.balance -= amount;
        Ok(format!("Transfered {} to {}", amount, acc_number))
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_zero() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_abble_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposite(100);
        assert_eq!(account.get_balance(), 100);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_depositing_negative_values() {
        let mut account = SavingsAccount::new();
        account.deposite(-100);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposite(100);
        account.transfer(123, 50)?;
        Ok(())
    }
}
