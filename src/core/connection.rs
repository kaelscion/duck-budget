use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


fn seed_default_categories(conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error>{
    use crate::models::category::NewCategory;
    let categories = vec![
        NewCategory {
            name: "Food".to_string(),
            description: "Food and drinks".to_string(),
        },
        NewCategory {
            name: "Entertainment".to_string(),
            description: "Movies, games, etc.".to_string(),
        },
        NewCategory {
            name: "Transportation".to_string(),
            description: "Gas, public transport, etc.".to_string(),
        },
    ];
    insert_into(category::category)
        .values(&categories)
        .execute(conn)
}

fn seed_default_accounts(conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
    let account = NewAccount {
        name: "Personal Savings".to_string(),
        balance: 100.0.into(),
    };
    insert_into(account::account)
        .values(&account)
        .execute(conn)
}

fn seed_defaults(conn: &mut SqliteConnection) {
    seed_default_categories(conn);
    seed_default_accounts(conn);
}


#[cfg(feature = "server")]
thread_local! {
    pub static DB : rusqlite::Connection = {
        dotenv().ok(); 
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = rusqlite::Connection::open(database_url).expect("Failed to connect to database");
        seed_defaults(&mut conn);
        conn
    };
}