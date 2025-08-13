use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use diesel::insert_into;
use std::sync::OnceLock;
use diesel::r2d2::PooledConnection;

fn seed_default_categories(conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>) -> Result<usize, diesel::result::Error>{
    use crate::core::models::category::NewCategory;
    use crate::core::schema::category::dsl::*;
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
    insert_into(category)
        .values(&categories)
        .execute(conn)
}

fn seed_default_accounts(conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>) -> Result<usize, diesel::result::Error> {
    use crate::core::models::account::NewAccount;
    use crate::core::schema::account::dsl::*;
    let new_account = NewAccount {
        name: "Personal Savings".to_string(),
        balance: 100.0.into(),
    };
    insert_into(account)
        .values(&new_account)
        .execute(conn)
}

fn seed_defaults(conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>) {
    seed_default_categories(conn);
    seed_default_accounts(conn);
}

static POOL: OnceLock<Pool<ConnectionManager<PgConnection>>> = OnceLock::new();

async fn init_pool() -> Pool<ConnectionManager<PgConnection>> {
    let secret = "postgresql://test_user:test_pass@localhost:5432/duck-budget";
    let manager = ConnectionManager::<PgConnection>::new(secret);
    tracing::info!("Connecting to database at {}", secret);
    let pool = Pool::builder()
        .max_size(25)
        .build(manager)
        .expect("Failed to create pool");
    pool
}


pub async fn get_pool() -> &'static Pool<ConnectionManager<PgConnection>> {
    if let Some(pool) = POOL.get() {
        pool
    } else {
        // Initialize the pool asynchronously
        let pool = init_pool().await;
        POOL.set(pool).expect("Failed to set global pool");
        POOL.get().unwrap()
    }
}
