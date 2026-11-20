#[derive(Debug)]struct Account{balance:i64}impl Account{fn deposit(&mut self,n:i64){self.balance+=n}}fn main(){let mut account=Account{balance:100};account.deposit(25);println!("{:?}",account);}
