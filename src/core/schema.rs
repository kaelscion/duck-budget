diesel::table! {
    account (id) {
        id -> Integer,
        name -> Text,
        balance -> Float,
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
        account_id -> Integer,
        category_id -> Integer,
        amount -> Float,
        transaction_type -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}