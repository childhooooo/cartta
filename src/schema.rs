table! {
    credentials (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        user_id -> Int4,
    }
}

table! {
    notes (id) {
        id -> Int4,
        title -> Varchar,
        content -> Varchar,
        access -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Nullable<Int4>,
    }
}

table! {
    note_tags (id) {
        id -> Int4,
        note_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(credentials -> users (user_id));
joinable!(note_tags -> notes (note_id));
joinable!(note_tags -> tags (tag_id));
joinable!(notes -> users (user_id));
joinable!(tags -> users (user_id));

allow_tables_to_appear_in_same_query!(
    credentials,
    notes,
    note_tags,
    tags,
    users,
);
