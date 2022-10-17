// @generated automatically by Diesel CLI.

pub mod main {
    diesel::table! {
        main.posts (id) {
            id -> Int4,
            title -> Varchar,
            body -> Text,
            published -> Bool,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        main.users (id) {
            id -> Int4,
            title -> Varchar,
            body -> Text,
            published -> Bool,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        posts,
        users,
    );
}
