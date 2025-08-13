use diesel::prelude::*;
use crate::core::connection::get_pool;
use crate::core::models::transactions::Transaction as TransactionData;

pub async fn get_transactions_by_account_id(
    acct_id: i32,
) -> Vec<TransactionData> {
    use crate::core::schema::transactions::dsl::*;
    let mut db = get_pool().await.get().expect("Failed to get connection from pool");
    transactions
        .filter(account_id.eq(acct_id))
        .select(TransactionData::as_select())
        .load::<TransactionData>(&mut db)
        .expect("Error loading transactions")
}