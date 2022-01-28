table! {
    artifacts (id) {
        id -> Integer,
        file_name -> Text,
        buildinfo_id -> Integer,
    }
}

table! {
    buildinfos (id) {
        id -> Integer,
        url -> Text,
        content -> Text,
    }
}

joinable!(artifacts -> buildinfos (buildinfo_id));

allow_tables_to_appear_in_same_query!(
    artifacts,
    buildinfos,
);
