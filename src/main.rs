use balances::Pallet;

mod balances;

fn main() {
    let mut pallet = Pallet::new();
    pallet.set_balance("daniel".to_string(), 2);

    let balance = pallet.balance("daniel".to_string());

    println!("Balance: {balance}");
    
    println!("Hello, world!");
}
