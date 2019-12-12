# Change Log

## [Unreleased]

### v0.2.0 - 2019-12-11

* Multi-party Transactions: Track transactions with multiple debtors and creditors (e.g. Alice & Bob paid 50 for a dinner shared between Alice, Bob and Charlie)
* Settle now automatically sizes for the fewest transactions, and settle_upto accepts manual sizes. 
* Monetary amounts are now tracked as 'Money' objects instead of integers, using the rusty_money gem

## [0.1.0] - 2019-11-19

* Initial Release

## [Planned]

### v0.3.0
* Implement a true subset sum solution: https://pure.tue.nl/ws/files/2062204/623903.pdf
* Support subunits in currency calculations.
