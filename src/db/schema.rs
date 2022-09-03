// @generated automatically by Diesel CLI.

diesel::table! {
    rel_todos_tags (id) {
        id -> Bpchar,
        todo_id -> Bpchar,
        tag_id -> Bpchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Bpchar,
        name -> Varchar,
        color -> Bpchar,
    }
}

diesel::table! {
    todos (id) {
        id -> Bpchar,
        name -> Varchar,
        memo -> Nullable<Text>,
    }
}

diesel::joinable!(rel_todos_tags -> tags (tag_id));
diesel::joinable!(rel_todos_tags -> todos (todo_id));

diesel::allow_tables_to_appear_in_same_query!(
    rel_todos_tags,
    tags,
    todos,
);
