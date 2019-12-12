# Debtsolver

Track and settle debts between parties with the fewest transactions. 

## Example

```rust
use debtsolver::Ledger;
use debtsolver::Transaction;
use debtsolver::transaction;

fn main() {
    let mut ledger = Ledger::new();

    // Let's say that:
    // Alice paid 20 for Bob's lunch
    // Bob paid 20 for Charlie's dinner the next day.
    ledger.add_transaction(transaction!("Alice", "Bob", (20, "USD")));
    ledger.add_transaction(transaction!("Bob", "Charlie", (20, "USD")));

    for payment in ledger.settle() {
        println!("{}", payment)
    } 
    // Debtsolver will resolve this with one payment:
    // Alice owes Charlie 20.00 USD


    // Now lets say that:
    //   Bob paid for Alice's breakfast (20).
    //   Charlie paid for Bob's lunch (50).
    //   Alice paid for Charlie's dinner (35).
    ledger.add_transaction(transaction!("Alice", "Bob", (20, "USD")));
    ledger.add_transaction(transaction!("Bob", "Charlie", (50, "USD")));
    ledger.add_transaction(transaction!("Charlie", "Alice", (35, "USD")));
    

    for payment in ledger.settle() {
        println!("{}", payment)
    } 
    // Debtsolver will resolve this with just two payments:
    // Bob owes Alice 15.00 USD
    // Bob owes Charlie 15.00 USD
}
