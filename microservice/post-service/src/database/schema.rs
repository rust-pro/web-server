// @generated automatically by Diesel CLI.

pub mod main {
    diesel::table! {
        main.satellites (id) {
            id -> Int4,
            name -> Varchar,
            life_exists -> Varchar,
            first_spacecraft_landing_date -> Nullable<Date>,
            planet_id -> Int4,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }
}
