use diesel::insert_into;
use diesel::prelude::*;

use crate::app::entities::user_entity::{RegisterUserEntity, UserEntity};
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
### Register
Intermediate function connecting between service and database
 */
pub fn register(new_user: RegisterUserEntity, conn: &mut PgConnection) -> QueryResult<UserEntity> {
    use crate::database::schema::main::users::dsl::*;
    insert_into(users).values(new_user).get_result(conn)
}

/**
 * Check Existing User
 */
pub fn check_existing_user(username: &str, conn: &mut PgConnection) -> QueryResult<UserEntity> {
    main::users::table
        .filter(main::users::username.eq(username))
        .first(conn)
}
