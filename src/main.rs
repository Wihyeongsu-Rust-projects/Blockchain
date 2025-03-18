mod balances;
mod system;
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let charlie = "Charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);

    let _ = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| {
        println!("Error: {:?}", e);
    });

    runtime.system.inc_nonce(&alice);

    let _ = runtime
        .balances
        .transfer(&alice, &charlie, 20)
        .map_err(|e| {
            println!("Error: {:?}", e);
        });

    println!("{:#?}", runtime);
}
