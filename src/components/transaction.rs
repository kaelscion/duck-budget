use dioxus::prelude::*;
use uuid::Uuid;

use data::Transaction as TransactionData;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Transaction(
    data: TransactionData,
) -> Element {
    rsx! {
        div {
            id: "transaction",
            img { src: HEADER_SVG, id: "header" }
            div { id: "transaction-details",
                h2 { "Transaction ID: {id}" }
                p { "Name: {name}" }
                p { "Amount: ${amount}" }
                p { "Date: {date}" }
                p { "Category: {category}" }
            }
        }
    }
}