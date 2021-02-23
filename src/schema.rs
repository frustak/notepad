table! {
    account (account_id) {
        account_id -> Int4,
        username -> Text,
        password -> Text,
    }
}

table! {
    notepad (notepad_id) {
        notepad_id -> Int4,
        note -> Text,
        account_id -> Int4,
    }
}

joinable!(notepad -> account (account_id));

allow_tables_to_appear_in_same_query!(
    account,
    notepad,
);
