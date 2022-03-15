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
        token -> Text,
    }
}

table! {
    task (id) {
        id -> Nullable<Integer>,
        crates_id -> Text,
        task_type -> Text,
        username -> Text,
        params -> Text,
    }
}

table! {
    user (id) {
        id -> Nullable<Integer>,
        username -> Text,
        passwd -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    crates,
    login_state,
    task,
    user,
);
