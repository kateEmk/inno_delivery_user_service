// @generated automatically by Diesel CLI.

diesel::table! {
    courier (uuid) {
        is_free -> Bool,
        rating -> Float8,
        uuid -> Uuid,
    }
}

diesel::table! {
    users (uuid) {
        first_name -> Text,
        address -> Text,
        phone_number -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
        is_blocked -> Bool,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        uuid -> Uuid,
    }
}

diesel::joinable!(courier -> users (uuid));

diesel::allow_tables_to_appear_in_same_query!(
    courier,
    users,
);

