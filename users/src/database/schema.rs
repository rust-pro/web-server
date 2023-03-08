// @generated automatically by Diesel CLI.

pub mod main {
    diesel::table! {
        main.user_profiles (id) {
            id -> Int4,
            user_id -> Int4,
            first_name -> Nullable<Varchar>,
            last_name -> Nullable<Varchar>,
            image -> Nullable<Varchar>,
            sex -> Nullable<Varchar>,
            birthday -> Nullable<Date>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        main.users (id) {
            id -> Int4,
            username -> Varchar,
            email -> Varchar,
            role -> Varchar,
            password -> Varchar,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::joinable!(user_profiles -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        user_profiles,
        users,
    );
}
