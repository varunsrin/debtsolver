# Debtsolver

Track and settle debts between parties with the fewest transactions. 

## Example

```rust
use debtsolver::Ledger;
use debtsolver::Transaction;

fn main() {
    let mut ledger = Ledger::new();

    // Let's say that:
    // Alice paid 20 for Bob's lunch
    // Bob paid 20 for Charlie's dinner the next day.
    ledger.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 20).unwrap());
    ledger.add_transaction(Transaction::new("Bob".to_string(), "Charlie".to_string(), 20).unwrap());

    for payment in ledger.settle(3) {
        println!("{}", payment)
    } 
    // Debtsolver will resolve this with one payment:
    // Alice owes Charlie 2


    // Now lets say that:
    //   Bob paid for Alice's breakfast (20).
    //   Charlie paid for Bob's lunch (50).
    //   Alice paid for Charlie's dinner (35).
    ledger.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 20).unwrap());
    ledger.add_transaction(Transaction::new("Bob".to_string(), "Charlie".to_string(), 50).unwrap());
    ledger.add_transaction(Transaction::new("Charlie".to_string(), "Alice".to_string(), 35).unwrap());
    

    for payment in ledger.settle(3) {
        println!("{}", payment)
    } 
    // Debtsolver will resolve this with just two payments:
    // Bob owes Alice 15
    // Bob owes Charlie 15
}
