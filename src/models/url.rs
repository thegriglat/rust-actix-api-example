use diesel;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::urls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Url {
    pub url: String,
    pub short_url: String,
}
