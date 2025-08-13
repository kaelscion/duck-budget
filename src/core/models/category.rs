use diesel::prelude::*;
use crate::core::schema::category;


#[derive(Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCategory {
    pub name: String,
    pub description: String,
}