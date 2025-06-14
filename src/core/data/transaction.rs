use diesel::prelude::*;
use crate::models::transactions::Transaction as TransactionData;

#[server]
async fn get_transactions_by_account_id(
    acct_id: i32,
) -> Vec<TransactionData> {
    use crate::schema::transactions::dsl::*;
    transactions
        .filter(account_id.eq(acct_id))
        .select(TransactionData::as_select())
        .load::<TransactionData>(DB)
        .expect("Error loading transactions")
}