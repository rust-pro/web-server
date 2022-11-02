use diesel::prelude::*;

use crate::app::models::model::SatelliteEntity;
use crate::database::schema::main::satellites;

pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<SatelliteEntity>> {
    use crate::database::schema::main::satellites::dsl::*;
    satellites.load(conn)
}

pub fn get(id: i32, conn: &mut PgConnection) -> QueryResult<SatelliteEntity> {
    satellites::table.find(id).get_result(conn)
}

pub fn get_by_planet_id(planet_id: i32, conn: &mut PgConnection) -> QueryResult<Vec<SatelliteEntity>> {
    satellites::table
        .filter(satellites::planet_id.eq(planet_id))
        .load(conn)
}
