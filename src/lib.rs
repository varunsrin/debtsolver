use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/*
* Add RustDoc examples: https://rust-lang.github.io/api-guidelines/documentation.html (4)
* Add documentation for public things
*/

// Represents a transaction where one party (debtor) pays another (creditor) the amount specified.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Transaction {
    debtor: String,
    creditor: String,
    amount: i32,
}

impl Transaction {
    pub fn new(debtor: String, creditor: String, amount: i32) -> Result<Self, ParseAmountError> {
        if amount <= 0 {
            return Err(ParseAmountError { amount: amount });
        };
        return Ok(Transaction {
            debtor,
            creditor,
            amount,
        });
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} owes {} {}", self.debtor, self.creditor, self.amount)
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

// Represents a zero-sum ledger which tracks the current state of who owes money, and who is owed money.
// The sum of all balances must always add up to zero, since each debtor has an equivalent creditor.
#[derive(Debug)]
pub struct Ledger {
    map: HashMap<String, i32>,
}

impl Ledger {
    pub fn new() -> Ledger {
        return Ledger {
            map: HashMap::new(),
        };
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        *self.map.entry(transaction.debtor).or_insert(0) -= transaction.amount;
        *self.map.entry(transaction.creditor).or_insert(0) += transaction.amount;
    }

    // Settles the ledger by creating a set of payments will payback creditors in the fewest
    // transactions. Also clears all current balances in the ledger. The best possible set
    // of payments is n/2 in size and the worst possible case is n-1, where n is the number of
    // people in the ledger.
    pub fn settle(&mut self, group_size: usize) -> Vec<Transaction> {
        let mut payments: Vec<Transaction> = Vec::new();
        if group_size > 0 {
            for x in 1..group_size + 1 {
                payments.append(&mut self.settle_combinations(x));
            }
        }
        payments.append(&mut self.clear_all_entries());
        return payments;
    }

    // Converts the ledger from a hashmap into a set of vector-tuples containing the
    // debtor/creditor and the amount. Debts are negative, and credits are positive.
    pub fn to_vector(&self) -> Vec<(String, i32)> {
        let mut ledger_entries: Vec<(String, i32)> = Vec::new();

        for (key, val) in self.map.iter() {
            ledger_entries.push((key.clone(), *val));
        }
        return ledger_entries;
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
        return payments;
    }

    // Settles all entries left in the ledger with a balance, in random order.
    fn clear_all_entries(&mut self) -> Vec<Transaction> {
        let (debtor_keys, creditor_keys) = self.debtor_and_creditor_keys();
        return self.clear_given_keys(debtor_keys, creditor_keys);
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
        return payments;
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
        return zero_sum_combinations;
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
        return (debtors, creditors);
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

}
