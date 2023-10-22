// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Varchar,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
