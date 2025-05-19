use uuid::Uuid;
use crate::data::transaction::Transaction;

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(name: String, balance: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            balance,
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn remove_transaction(&mut self, transaction_id: Uuid) {
        self.transactions.retain(|t| t.id != transaction_id);
    }

    pub fn display(&self) -> String {
        format!(
            "Account ID: {}, Name: {}, Balance: {}",
            self.id, self.name, self.balance
        )
    }
}
