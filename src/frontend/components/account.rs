use dioxus::prelude::*;
use crate::core::models::account::Account as AccountData;
// use crate::components::transaction::TransactionList;


#[component]
pub fn Account(data: AccountData) -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "account",
            h2 { "Account: {data.name}"}
            p { "Balance: ${data.balance}" }
            // TransactionList { 
            //     account_id: props.data.id,
            //     conn: props.conn.clone(),
            // }
        }
    }
}
