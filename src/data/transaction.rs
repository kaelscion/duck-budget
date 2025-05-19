use uuid::Uuid;
use chrono::{DateTime, Local};
use crate::data::category::Category;

#[derive(Debug, Clone, PartialEq)]
enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub label: String,
    pub category: Category,
    pub account_id: Uuid,
    pub amount: f64,
    pub transaction_type: TransactionType, // e.g., "credit" or "debit"
    pub date: DateTime<Local>, // e.g., "2023-10-01"
}

impl Transaction {
    pub fn new(account_id: Uuid, label: String, category: Category, amount: f64, transaction_type: TransactionType, date: Option<DateTime<Local>>) -> Self {
        let date = match date{
            Some(date) => date,
            None => chrono::Local::now(),
        };
        Self {
            id: Uuid::new_v4(),
            label: label,
            account_id: account_id,
            category: category,
            amount: amount,
            transaction_type: transaction_type,
            date: date,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Transaction ID: {}, Account ID: {}, Amount: {}, Type: {:?}, Date: {}",
            self.id, self.account_id, self.amount, self.transaction_type, self.date
        )
    }
}