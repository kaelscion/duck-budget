use diesel::prelude::*;
use crate::schema::account;


#[derive(Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAccount {
    pub name: String,
    pub balance: f64,
}
