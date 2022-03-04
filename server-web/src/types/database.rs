table! {
    User (id) {
        id -> Nullable<Integer>,
        username -> Text,
        passwd -> Text,
    }
}

table! {
    login_state (id) {
        id -> Nullable<Integer>,
        username -> Text,
        session_data -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    User,
    login_state,
);
