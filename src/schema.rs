// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        balance -> Double,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    category (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        label -> Text,
        category_id -> Integer,
        account_id -> Integer,
        amount -> Double,
        transaction_type -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(transactions -> account (account_id));
diesel::joinable!(transactions -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    category,
    transactions,
);
