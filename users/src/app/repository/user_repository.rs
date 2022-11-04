use diesel::prelude::*;

use crate::app::entities::user_entity::UserEntity;
use crate::database::schema::main;
/**
* Get All users
*/
pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<UserEntity>> {
    use crate::database::schema::main::users::dsl::*;
    users.load(conn)
}

pub fn get_user(id: i32, conn: &mut PgConnection) -> QueryResult<UserEntity> {
    main::users::table.find(id).get_result(conn)
}
