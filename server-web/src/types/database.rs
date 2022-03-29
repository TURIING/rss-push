table! {
    crates (crate_id) {
        crate_id -> Text,
        crate_type -> Text,
        info -> Text,
    }
}

table! {
    subscribe (id) {
        id -> Nullable<Integer>,
        username -> Text,
        crate_id -> Text,
    }
}

table! {
    task (id) {
        id -> Nullable<Integer>,
        crate_id -> Text,
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

joinable!(subscribe -> crates (crate_id));

allow_tables_to_appear_in_same_query!(
    crates,
    subscribe,
    task,
    user,
);
