

use diesel::{Insertable, Selectable, Queryable};
use serde::Deserialize;

use uuid::Uuid;

use crate::database::schema;

// --- orm only

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::portfolio )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertPortfolio<'a> {
    pub app_user_id: Uuid,
    pub label: &'a str
}

#[derive(Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::portfolio )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SelectPortfolio {
    pub id: Uuid,
    pub label: String,
    pub app_user_id: Uuid
}
