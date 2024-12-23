#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    // If we need to store the value somewhere, tend to receive ownership of value
    fn add_account(&mut self, account: Account) {
        // Mutable reference to self because we will be modifying `self`
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        // for account in &self.accounts {
        // total_balance += account.balance;
        // }

        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance = 10
}

fn print_bank(bank: &Bank) {
    println!("{:#?}", bank);
}

fn main() {
    // Every value is `owned` by a single variable, struct, vector, argument at a time
    // reassigning the value `moves` the value to another variable
    let mut account_luis = Account::new(1, String::from("Luis"));
    let mut account_goldie = Account::new(2, String::from("Goldie"));
    let mut bank = Bank::new();

    account_luis.deposit(500);
    account_goldie.deposit(1000);
    let account_luis_summary = account_luis.summary();
    println!("{account_luis_summary}");
    bank.add_account(account_luis);
    bank.add_account(account_goldie);

    println!("{}", bank.total_balance());
    println!("{:#?}", bank.summary());
}
