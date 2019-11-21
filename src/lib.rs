//! Track and settle debts between parties with the fewest transactions.
//!
//! Debtsolves gives you two structs - Transactions, which track payments that
//! have been made or need to be made, and a Ledger which can store and balance
//! the current states of credits and debits between everyone.
//!
//! # Use
//!
//! Transactions must be initialized with a debtor, creditor and positive amount.
//! For example, if Bob borrows 10 from Alice, you would track that as:
//!
//! ```edition2018
//! transaction = Transaction::new(debtor: "Alice".to_string, creditor: "Bob".to_string, amount: 10)
//! ```
//!
//! Legders are created empty, and you can add transactions to them to track the current state of
//! debtors and creditors.   
//!
//! ```edition2018
//! ledger = Ledger::new()
//! ledger.add_transaction(transaction);
//! ```
//!
//! You can inspect the state of the ledger at any point by calling to_vector on it to get the
//! list of debtors and creditors as a vector of tuples
//!
//! ```edition2018
//! for transaction in ledger.to_vector(){
//!     println!("{}", transaction)
//! };
//! // (Alice, Bob, 10)
//! ```
//!
//! Once all the debts are tracked, and you want to figure out the fastest way for debtors to pay
//! back creditors, you can simply call settle:
//!
//! ```edition2018
//! let payments = ledger.settle(3);
//! ```
//!   
//!
//! ### Examples
//! ```edition2018
//!
//! use debtsolver::Ledger;
//! use debtsolver::Transaction;
//!
//! fn main() {
//!     let mut ledger = Ledger::new();
//!
//!     // Let's say that:
//!     // Alice paid 20 for Bob's lunch
//!     // Bob paid 20 for Charlie's dinner the next day.
//!     ledger.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 20).unwrap());
//!     ledger.add_transaction(Transaction::new("Bob".to_string(), "Charlie".to_string(), 20).unwrap());
//!
//!     for payment in ledger.settle(3) {
//!         println!("{}", payment)
//!     }
//!     // Debtsolver will resolve this with one payment:
//!     // Alice owes Charlie 2
//!
//!
//!     // Now lets say that:
//!     //   Bob paid for Alice's breakfast (20).
//!     //   Charlie paid for Bob's lunch (50).
//!     //   Alice paid for Charlie's dinner (35).
//!     ledger.add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 20).unwrap());
//!     ledger.add_transaction(Transaction::new("Bob".to_string(), "Charlie".to_string(), 50).unwrap());
//!     ledger.add_transaction(Transaction::new("Charlie".to_string(), "Alice".to_string(), 35).unwrap());
//!    
//!
//!     for payment in ledger.settle(3) {
//!         println!("{}", payment)
//!     }
//!     //Debtsolver will resolve this with just two payments:
//!     // Bob owes Alice 15
//!     // Bob owes Charlie 15
//! ```

use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// pub fn =  (amount & currency)
// pub fn +  (test if money, currency)
// pub fn - (test if money, currency)
// pub fn * (money or sdcalar)
// pub fn / (money or scalar)
// pub fn compare

#[derive(Debug, PartialEq)]
pub struct Money {
    amount: i32,
    currency: String,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

impl Money {
    pub fn new(amount: i32, currency: String) -> Money {
        // test that currency is valid.
        // accept amounts as string, parse to decimals, panic if wrong.
        Money { amount, currency }
    }

    pub fn allocate_to(self, number: i32) -> Vec<Money> {
        let ratios: Vec<i32> = (0..number).map(|_| 1).collect();
        self.allocate(ratios)
    }

    pub fn allocate(self, ratios: Vec<i32>) -> Vec<Money> {
        if ratios.is_empty() {
            panic!();
        }

        let mut remainder = self.amount;
        let ratio_total: i32 = ratios.iter().sum();
        let mut allocations: Vec<Money> = Vec::new();

        for ratio in ratios {
            if ratio <= 0 {
                panic!();
            }
            let share = self.amount * ratio / ratio_total;
            allocations.push(Money::new(share, self.currency.clone()));
            remainder -= share;
        }

        let mut i = 0;
        while remainder > 0 {
            allocations[i as usize].amount += 1;
            remainder -= 1;
            i += 1;
        }
        allocations
    }
}

/// Represents a transaction where one party (debtor) pays another (creditor) the amount specified.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Transaction {
    debtor: String,
    creditor: String,
    amount: i32,
}

impl Transaction {
    pub fn new(debtor: String, creditor: String, amount: i32) -> Result<Self, ParseAmountError> {
        if amount <= 0 {
            return Err(ParseAmountError { amount });
        };
        Ok(Transaction {
            debtor,
            creditor,
            amount,
        })
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} owes {} {}", self.debtor, self.creditor, self.amount)
    }
}

/// Represents a multi-party transaction where one or more parties (debtors) owes one or more
/// parties (creditors) the amount specified.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MultiPartyTransaction {
    debtors: Vec<String>,
    creditors: Vec<String>,
    amount: i32,
}

impl MultiPartyTransaction {
    pub fn new(
        debtors: Vec<String>,
        creditors: Vec<String>,
        amount: i32,
    ) -> Result<Self, ParseAmountError> {
        if amount <= 0 {
            return Err(ParseAmountError { amount });
        };
        Ok(MultiPartyTransaction {
            debtors,
            creditors,
            amount,
        })
    }
}

impl fmt::Display for MultiPartyTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} owes {} to {}",
            self.debtors.join(","),
            self.amount,
            self.creditors.join(","),
        )
    }
}

#[derive(Debug)]
pub struct ParseAmountError {
    amount: i32,
}

impl Error for ParseAmountError {}

impl fmt::Display for ParseAmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction amount {} is less than or equal to 0",
            self.amount
        )
    }
}

/// Represents a zero-sum ledger which tracks the current state of who owes money, and who is owed money.
/// The sum of all balances must always add up to zero, since each debtor has an equivalent creditor.
#[derive(Debug)]
pub struct Ledger {
    map: HashMap<String, i32>,
}

impl Ledger {
    /// Creates a new Ledger
    pub fn new() -> Ledger {
        Ledger {
            map: HashMap::new(),
        }
    }

    /// Accepts a transaction and updates debtor and creditor balances in the ledger.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        *self.map.entry(transaction.debtor).or_insert(0) -= transaction.amount;
        *self.map.entry(transaction.creditor).or_insert(0) += transaction.amount;
    }

    pub fn add_multi_party_transaction(&mut self, transaction: MultiPartyTransaction) {
        let num_debtors = transaction.debtors.len() as i32;
        let debt_amount = transaction.amount / num_debtors;
        let debt_total = num_debtors * debt_amount;

        let num_creditors = transaction.creditors.len() as i32;
        let credit_amount = transaction.amount / num_creditors;
        let credit_total = num_creditors * credit_amount;

        if debt_total > credit_total {
            *self
                .map
                .entry(transaction.creditors[0].clone())
                .or_insert(0) += debt_total - credit_total;
        } else if credit_total > debt_total {
            *self.map.entry(transaction.debtors[0].clone()).or_insert(0) -=
                credit_total - debt_total;
        }

        for debtor in transaction.debtors {
            *self.map.entry(debtor).or_insert(0) -= debt_amount;
        }

        for creditor in transaction.creditors {
            *self.map.entry(creditor).or_insert(0) += credit_amount;
        }
    }

    /// Returns the smallest possible set of  transactions that will resolve all debts.
    /// This ranges between n/2 (best case) and n-1 (worst case), where n is the number of
    /// debtors and creditors.
    pub fn settle(&mut self, group_size: usize) -> Vec<Transaction> {
        let mut payments: Vec<Transaction> = Vec::new();
        if group_size > 0 {
            for x in 1..=group_size {
                payments.append(&mut self.settle_combinations(x));
            }
        }
        payments.append(&mut self.clear_all_entries());
        payments
    }

    // Converts the ledger from a hashmap into a set of vector-tuples containing the
    // debtor/creditor and the amount. Debts are negative, and credits are positive.
    pub fn to_vector(&self) -> Vec<(String, i32)> {
        let mut ledger_entries: Vec<(String, i32)> = Vec::new();

        for (key, val) in self.map.iter() {
            ledger_entries.push((key.clone(), *val));
        }
        ledger_entries
    }

    fn panic_unless_empty(&self) {
        for (_, val) in self.map.iter() {
            if *val > 0 {
                panic!();
            }
        }
    }

    // Settles combinations of a specified size. A combination is a set of ledger balances that
    // are zero sum (add up to zero).
    // e.g.  A = 3, B = -2 and C= -1 is a group entry of 3, since the three of them settle to 0.
    fn settle_combinations(&mut self, combo_size: usize) -> Vec<Transaction> {
        let mut payments: Vec<Transaction> = Vec::new();
        let settling_combinations = self.find_zero_sum_combinations(combo_size);

        for combo in settling_combinations {
            let mut debtor_keys: Vec<String> = Vec::new();
            let mut creditor_keys: Vec<String> = Vec::new();
            for item in combo {
                if item.1 > 0 {
                    creditor_keys.push(item.0)
                } else if item.1 < 0 {
                    debtor_keys.push(item.0)
                } else {
                }
            }
            payments.append(&mut self.clear_given_keys(debtor_keys, creditor_keys));
        }
        payments
    }

    // Settles all entries left in the ledger with a balance, in random order.
    fn clear_all_entries(&mut self) -> Vec<Transaction> {
        let (debtor_keys, creditor_keys) = self.debtor_and_creditor_keys();
        let transactions = self.clear_given_keys(debtor_keys, creditor_keys);
        self.panic_unless_empty();
        transactions
    }

    // Settles a specified list of debtors and creditors against in other, in random order.
    fn clear_given_keys(
        &mut self,
        debtors: Vec<String>,
        creditors: Vec<String>,
    ) -> Vec<Transaction> {
        let mut payments: Vec<Transaction> = Vec::new();

        for debtor in &debtors {
            let mut debtor_amount = *self.map.get(debtor).unwrap();

            for creditor in &creditors {
                let mut creditor_amount = *self.map.get(creditor).unwrap();

                // If there's still debt and credit, create a payment.
                // If either one is missing, try grabbing another creditor
                // If you run out of creditors, grab another debtor and start again.
                while (creditor_amount > 0) && (debtor_amount < 0) {
                    let credit_abs = creditor_amount.abs();
                    let debt_abs = debtor_amount.abs();
                    let payment_amount = cmp::min(credit_abs, debt_abs);

                    debtor_amount += payment_amount;
                    self.map.insert(debtor.clone(), debtor_amount);

                    creditor_amount -= payment_amount;
                    self.map.insert(creditor.clone(), creditor_amount);

                    payments.push(
                        Transaction::new(debtor.clone(), creditor.clone(), payment_amount).unwrap(),
                    )
                }
            }
        }
        payments
    }

    // Finds zero sum combinations of a given size of ledger entries.
    fn find_zero_sum_combinations(&self, combo_size: usize) -> Vec<Vec<(String, i32)>> {
        let mut zero_sum_combinations: Vec<Vec<(String, i32)>> = Vec::new();
        let combinations = self.to_vector().into_iter().combinations(combo_size);
        for item in combinations {
            if item.iter().fold(0, |acc, x| acc + x.1) == 0 {
                zero_sum_combinations.push(item);
            }
        }
        zero_sum_combinations
    }

    // Returns vectors of keys of debtors and creditors with an active balance.s
    fn debtor_and_creditor_keys(&self) -> (Vec<String>, Vec<String>) {
        let mut creditors: Vec<String> = Vec::new();
        let mut debtors: Vec<String> = Vec::new();

        for (person, value) in &self.map {
            if *value > 0 {
                creditors.push(person.clone());
            } else if *value < 0 {
                debtors.push(person.clone());
            } else {
            }
        }
        (debtors, creditors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The settlement should always choose credits and debits that are equal over any other type.
    // This allows two entries in the ledger to be removed in exchange for a single payment.
    // For example, if A = -10 and B = +10, they should always first over any other possibility
    #[test]
    fn matches_equal_debts_and_credits_when_groupsize_is_2() {
        let mut ledger = Ledger::new();

        let expected_results = vec![
            Transaction::new("A".to_string(), "B".to_string(), 2).unwrap(),
            Transaction::new("C".to_string(), "F".to_string(), 3).unwrap(),
            Transaction::new("D".to_string(), "F".to_string(), 5).unwrap(),
            Transaction::new("E".to_string(), "F".to_string(), 7).unwrap(),
        ];

        // The worst case match (i.e. random) can accidentially find the optimal solution for small
        // sets, so we repeat to make this very unlikely
        for _ in 0..5 {
            ledger.add_transaction(Transaction::new("A".to_string(), "B".to_string(), 2).unwrap());
            ledger.add_transaction(Transaction::new("C".to_string(), "F".to_string(), 3).unwrap());
            ledger.add_transaction(Transaction::new("D".to_string(), "F".to_string(), 5).unwrap());
            ledger.add_transaction(Transaction::new("E".to_string(), "F".to_string(), 7).unwrap());

            let mut payments = ledger.settle(2);
            payments.sort();
            assert_eq!(payments, expected_results);
        }
    }

    // Next, the settlement should always choose 3 credits and debits that are zero sum over any other.
    // This allows three entries in the ledger to be removed in exchange for two payments.
    // For example, if A = -10,  B = +5, C= +5.
    #[test]
    fn finds_groups_of_three_credits_and_debits_when_groupsize_is_3() {
        // Test that group matched  payments are always settled first.
        let mut ledger = Ledger::new();

        let expected_results = vec![
            Transaction::new("A".to_string(), "D".to_string(), 3).unwrap(),
            Transaction::new("C".to_string(), "D".to_string(), 4).unwrap(),
            Transaction::new("E".to_string(), "B".to_string(), 10).unwrap(),
            Transaction::new("F".to_string(), "B".to_string(), 17).unwrap(),
            Transaction::new("J".to_string(), "K".to_string(), 20).unwrap(),
            Transaction::new("U".to_string(), "K".to_string(), 21).unwrap(),
        ];

        // The worst case match (i.e. random) can accidentially find the optimal solution for small
        // sets, so we repeat to make this very unlikely
        for _ in 0..5 {
            ledger.add_transaction(Transaction::new("A".to_string(), "D".to_string(), 3).unwrap());
            ledger.add_transaction(Transaction::new("C".to_string(), "D".to_string(), 4).unwrap());
            ledger.add_transaction(Transaction::new("E".to_string(), "B".to_string(), 10).unwrap());
            ledger.add_transaction(Transaction::new("F".to_string(), "B".to_string(), 17).unwrap());
            ledger.add_transaction(Transaction::new("J".to_string(), "K".to_string(), 20).unwrap());
            ledger.add_transaction(Transaction::new("U".to_string(), "K".to_string(), 21).unwrap());

            let mut payments = ledger.settle(3);
            payments.sort();
            assert_eq!(payments, expected_results);
        }
    }

    #[test]
    #[should_panic]
    fn panics_when_settling_unbalanced_ledger() {
        let mut ledger = Ledger::new();
        ledger.map.entry("A".to_string()).or_insert(10);
        ledger.settle(2);
    }

    // Multi Party Transaction Tests
    #[test]
    fn can_handle_debtor_rounding() {
        let transaction = MultiPartyTransaction::new(
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["D".to_string()],
            10,
        )
        .unwrap();
        let mut ledger = Ledger::new();
        ledger.add_multi_party_transaction(transaction);
        let ledger_balance = ledger.to_vector().into_iter().fold(0, |acc, x| acc + x.1);
        assert_eq!(ledger_balance, 0);
    }

    #[test]
    fn can_handle_creditor_rounding() {
        let transaction = MultiPartyTransaction::new(
            vec!["A".to_string()],
            vec!["B".to_string(), "C".to_string(), "D".to_string()],
            10,
        )
        .unwrap();
        let mut ledger = Ledger::new();
        ledger.add_multi_party_transaction(transaction);
        let ledger_balance = ledger.to_vector().into_iter().fold(0, |acc, x| acc + x.1);
        assert_eq!(ledger_balance, 0);
    }

    // Transaction Tests
    #[test]
    fn can_create_positive_transaction() {
        match Transaction::new("A".to_string(), "B".to_string(), 1) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn cannot_create_negative_transaction() {
        match Transaction::new("A".to_string(), "B".to_string(), -1) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn cannot_create_zero_transaction() {
        match Transaction::new("A".to_string(), "B".to_string(), -1) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    // Money Tests
    #[test]
    fn allocate() {
        let money = Money::new(11, "USD".to_string());
        let allocs = money.allocate(vec![1, 1, 1]);
        let expected_results = vec![
            Money::new(4, "USD".to_string()),
            Money::new(4, "USD".to_string()),
            Money::new(3, "USD".to_string()),
        ];
        assert_eq!(expected_results, allocs);
    }

    #[test]
    #[should_panic]
    fn allocate_panics_if_empty() {
        Money::new(1, "USD".to_string()).allocate(Vec::new());
    }

    #[test]
    #[should_panic]
    fn allocate_panics_any_ratio_is_zero() {
        Money::new(1, "USD".to_string()).allocate(vec![1, 0]);
    }

    #[test]
    fn allocate_to() {
        let money = Money::new(11, "USD".to_string());
        let allocs = money.allocate_to(3);
        let expected_results = vec![
            Money::new(4, "USD".to_string()),
            Money::new(4, "USD".to_string()),
            Money::new(3, "USD".to_string()),
        ];
        assert_eq!(expected_results, allocs);
    }

    #[test]
    #[should_panic]
    fn allocate_to_panics() {
        Money::new(1, "USD".to_string()).allocate_to(0);
    }
}
