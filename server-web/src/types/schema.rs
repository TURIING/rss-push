table! {
    crates (crate_id) {
        crate_id -> Text,
        crate_type -> Integer,
        info -> Text,
    }
}

table! {
    message (id) {
        id -> Nullable<Integer>,
        message_id -> Text,
        crate_id -> Text,
        receiver -> Text,
        check_status -> Bool,
        notify_status -> Bool,
    }
}

table! {
    message_content (id) {
        id -> Nullable<Integer>,
        message_id -> Text,
        send_time -> Text,
        content -> Text,
        title -> Text,
        description -> Text,
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
        task_type -> Integer,
        username -> Text,
        params -> Nullable<Text>,
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
    message,
    message_content,
    subscribe,
    task,
    user,
);
