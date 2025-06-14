use crate::connection::establish_connection;
use diesel::prelude::*;
use crate::models::category::Category as CategoryData;

#[server]
async fn get_category_by_id(
    category_id: i32,
) -> CategoryData {
    use crate::schema::category::dsl::*;
    category.filter(id.eq(category_id))
        .select(CategoryData::as_select())
        .first::<CategoryData>(DB)
        .expect("Error loading category")
}