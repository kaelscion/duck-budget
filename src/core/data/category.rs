use crate::core::connection::get_pool;
use crate::core::models::category::Category as CategoryData;
use diesel::prelude::*;

pub async fn get_category_by_id(category_id: i32) -> Result<Option<CategoryData>, diesel::result::Error> {
    use crate::core::schema::category::dsl::*;
    let mut db = get_pool().await.get().expect("Failed to get connection from pool");
    Ok(category.filter(id.eq(&category_id))
        .first(&mut db)
        .ok())
}