table! {
    user (id) {
        id -> Nullable<Integer>,
        username -> Text,
        passwd -> Text,
    }
}

table! {
    crates (crates_id) {
        crates_id -> Text,
        crates_type -> Text,
        info -> Text,
    }
}

table! {
    login_state (id) {
        id -> Nullable<Integer>,
        username -> Text,
        session_data -> Text,
    }
}

table! {
    task (crates_id) {
        crates_id -> Text,
        task_type -> Text,
        username -> Text,
        params -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    user,
    crates,
    login_state,
    task,
);
