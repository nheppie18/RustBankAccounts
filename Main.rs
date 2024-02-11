use std::io;

struct BankAccount {
    identification: u64,
    balance: u64,
}

impl BankAccount {
    fn set_id(&mut self, id: u64) {
        self.identification = id;
    }

    fn get_id(&self) -> u64 {
        return self.identification;
    }

    fn set_balance(&mut self, amount: u64) {
        self.balance = amount;
    }

    fn get_balance(&self) -> u64 {
        return self.balance;
    }

    fn deposit(&mut self, amount: u64) {
        self.balance = amount + self.balance;
    }
    
    fn withdraw(&mut self, amount: u64) {
        if amount <= self.balance {
            self.balance = self.balance - amount;
            println!("you have withdrawn ${}", amount);
        }
        else {
            println!("you dont have enough money!");
        }
    }
}   


fn main() {
 let mut finished: bool = false;
 let mut account_created = false;
 let mut input = String::new();

 let mut acct1 = BankAccount {
    identification: 0,
    balance: 0,
 };
 
 while finished != true {
    println!("Select an option: ");
    println!("Option 1: create your account.");
    println!("Option 2: deposit to your account.");
    println!("Option 3: withdraw from your account.");
    println!("Option 4: view account details.");
    println!("Option 5: quit the program.");
    io::stdin().read_line(&mut input).unwrap();
    let option: u64 = input.trim().parse().expect("Input is not an integer.");
    input.clear();
    println!();
    
    //create an account option
    if option == 1 {
        if account_created == false {
            //get an id number from the user
            println!("Please provide an id number.");
            io::stdin().read_line(&mut input).unwrap();
            let id: u64 = input.trim().parse().expect("Input is not an Integer");
            input.clear();

            //get an initial balance from the user
            println!("Please provide an intitial balance for the account.");

            io::stdin().read_line(&mut input).unwrap();
            let amount: u64 = input.trim().parse().expect("Input is not an integer.");
            input.clear();

            //create an instance of bank account
            acct1.set_id(id);
            acct1.set_balance(amount);
            println!("id is: {}", acct1.get_id());
            println!("balance is: ${}", acct1.get_balance());

            println!("Your account has been successfully created, you can now deposit and withdraw.");
            println!();
            account_created = true;
        }
        else if account_created == true {
            println!("Your account has already been created.");
        }
    }
    
    //deposit option
    else if option == 2 {
        if account_created == true {
            println!("How much do you want to deposit?: ");
            io::stdin().read_line(&mut input).unwrap();
            let deposit_amount: u64 = input.trim().parse().expect("Input is not an integer.");
            input.clear();
            acct1.deposit(deposit_amount);
            println!("you have deposited ${}", deposit_amount);
            println!("Your balace is now now: ${}", acct1.get_balance());
            println!();
        }
        else {
            println!("you must create an account first!");
            println!();
        }
    }
    
    //withdraw option
    else if option == 3 {
        if account_created == true {
            println!("How much do you want to withdraw?: ");
            io::stdin().read_line(&mut input).unwrap();
            let withdraw_amount: u64 = input.trim().parse().expect("Input is not an integer.");
            input.clear();
            acct1.withdraw(withdraw_amount);
            println!("Your balance is now: ${}", acct1.get_balance());
            println!();
        }
        else {
            println!("you must create an account first!");
            println!();
        }
    }
    
    //view account option
    else if option == 4 {
        if account_created == true {
            println!("Account ID is: {}", acct1.get_id());
            println!("Account balance is: ${}", acct1.get_balance());
            println!();
        }
        else {
            println!("you must create an account first!");
            println!();
        }
    }

    //quit option
    else if option == 5 {
        println!("Good bye!");
        finished = true;
    }
    
    //error option
    else {
        println!("That is not a valid option!");
    }
 }
}
