use diesel::prelude::*;
use crate::schema::transactions;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl TransactionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "credit" => Some(TransactionType::Credit),
            "debit" => Some(TransactionType::Debit),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            TransactionType::Credit => "Credit",
            TransactionType::Debit => "Debit",
        }
    }
}

#[derive(Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub label: String,
    pub category_id: i32,
    pub account_id: i32,
    pub amount: f64,
    pub transaction_type: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Category))]
pub struct NewTransaction {
    pub label: String,
    pub category_id: i32,
    pub account_id: i32,
    pub amount: f64,
    pub transaction_type: String,
}
