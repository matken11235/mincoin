extern crate serde;

use self::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Transaction {
    sender: String,
    recipient: String,
    value: i32,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, value: i32) -> Transaction {
        Transaction { sender, recipient, value, }
    }
}

impl std::string::ToString for Transaction {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub trait ToVecString {
    fn to_vec_string(&self) -> Vec<String>;
}

impl<T> ToVecString for Vec<T> where
    T: ToString + Serialize {
    fn to_vec_string(&self) -> Vec<String> {
        self.into_iter().map(|x| x.to_string()).collect()
    }
}

#[derive(Clone, Debug)]
pub struct TransactionPool {
    transactions: Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> TransactionPool {
        println!("Initializing TransactionPool ...");
        TransactionPool {
            transactions: Vec::new(),
        }
    }

    pub fn set_new_transaction(&mut self, transaction: Transaction) {
        println!("set_new_transaction is called: {:#?}", transaction);
        self.transactions.push(transaction);
    }

    pub fn clear_my_transactions(&mut self, index: usize) {
        if index <= self.transactions.len() {
            let new_txns = self.transactions[index..].to_vec();
            println!("transaction is now refreshed ... ({:#?})", new_txns);
            self.transactions = new_txns.clone();
        };
    }

    pub fn get_stored_transactions(&self) -> Option<Vec<Transaction>> {
        if self.transactions.len() > 0 {
            Some(self.transactions.clone())
        } else {
            println!("Currently, it seems transaction pool is empty ...");
            None
        }
    }
}