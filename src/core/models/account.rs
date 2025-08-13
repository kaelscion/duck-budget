use diesel::prelude::*;
use crate::core::schema::account;


#[derive(Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: f32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccount {
    pub name: String,
    pub balance: f32,
}
