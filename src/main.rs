mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
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
