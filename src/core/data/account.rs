use crate::core::connection::get_pool;
use crate::core::models::account::Account as AccountData;
use diesel::prelude::*;


pub async fn get_account_by_id(account_id: i32) -> Result<Option<AccountData>, diesel::result::Error> {
    use crate::core::schema::account::dsl::*;
    let mut db = get_pool().await.get().expect("Failed to get connection from pool");
    Ok(account.filter(id.eq(&account_id))
        .first(&mut db)
        .ok())
}

pub async fn get_all_accounts() -> Result<Vec<AccountData>, diesel::result::Error> {
    use crate::core::schema::account::dsl::*;
    let mut db: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = get_pool().await.get().expect("Failed to get connection from pool");
    Ok(account.load(&mut db).unwrap_or_default())
}