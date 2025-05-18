use uuid::Uuid;
use chrono::DateTime;


enum TransactionType {
    Credit,
    Debit,
}

pub struct Transaction {
    pub id: Uuid,
    pub label: String,
    pub category: Category,
    pub account_id: Uuid,
    pub amount: f64,
    pub transaction_type: TransactionType, // e.g., "credit" or "debit"
    pub date: DateTime, // e.g., "2023-10-01"
}

impl Transaction {
    pub fn new(label: String, account_id: String, amount: f64, transaction_type: TransactionType, date: Option<String>) -> Self {
        date = match date{
            Some(date) => date,
            None => chrono::Local::now(),
        };
        Self {
            id: Uuid::new_v4(),
            account_id,
            amount,
            transaction_type,
            date,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Transaction ID: {}, Account ID: {}, Amount: {}, Type: {:?}, Date: {}",
            self.id, self.account_id, self.amount, self.transaction_type, self.date
        )
    }
}