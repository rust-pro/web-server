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

/**
 * Get user by ID
 */
pub fn get_user_by_id(id: i32, conn: &mut PgConnection) -> QueryResult<UserEntity> {
    main::users::table.find(id).get_result(conn)
}

/**
 * Check Existing User
 */
pub fn check_existing_user(username: &str, conn: &mut PgConnection) -> QueryResult<UserEntity> {
    main::users::table
        .filter(main::users::username.eq(username))
        .first(conn)
}
