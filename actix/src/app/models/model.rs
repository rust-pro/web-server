use chrono::{DateTime, Local, NaiveDate};
use diesel::prelude::*;

use crate::database::schema::main::satellites;

#[derive(Identifiable, Queryable)]
#[diesel(table_name = satellites)]
pub struct SatelliteEntity {
    pub id: i32,
    pub name: String,
    pub life_exists: String,
    pub first_spacecraft_landing_date: Option<NaiveDate>,
    pub planet_id: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
