// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Int4,
        user_id -> Int4,
        address -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        phone_number -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
        is_blocked -> Nullable<Bool>,
    }
}

diesel::joinable!(addresses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    users,
);
