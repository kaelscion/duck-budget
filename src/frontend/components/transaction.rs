use dioxus::prelude::*;
use crate::core::data::transaction::get_transactions_by_account_id;
use crate::core::data::category::get_category_by_id;
use crate::core::models::transactions::Transaction as TransactionData;

#[component]
fn Transaction(
    data: TransactionData,
) -> Element {
    let category = get_category_by_id(data.category_id);
    rsx! {
        div {
            id: "transaction",
            div { id: "transaction-details",
                p { "Name: {data.label}" }
                p { "Amount: ${data.amount}" }
                p { "Date: {category.created_at}" }
                p { "Category: {category.name}" }
            }
        }
    }
}

struct TransactionListProps {
    account_id: i32,
}
#[component]
pub fn TransactionList(
    props: TransactionListProps,
) -> Element {
    let transactions  = get_transactions_by_account_id(props.account_id); 
    rsx! {
        div {
            id: "transaction-list",
            h2 { "Transactions" }
            for transaction in transactions {
                Transaction { data: transaction }
            }
        }
    }
}