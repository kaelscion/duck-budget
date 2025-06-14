use crate::connection::establish_connection;
use crate::models::account::Account as AccountData;
use diesel::prelude::*;

#[server]
async fn get_account_by_id(account_id: i32, ) -> Option<AccountData> {
    use crate::schema::account::dsl::*;
    account.filter(id.eq(account_id))
        .first(DB)
        .ok()
}