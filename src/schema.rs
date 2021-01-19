table! {
    apis (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        respository -> Nullable<Text>,
        documentation -> Nullable<Text>,
        homepage -> Nullable<Text>,
        last_updated -> Timestamptz,
        created -> Timestamptz,
        creator_id -> Text,
    }
}

table! {
    tags (api_id, tag) {
        api_id -> Uuid,
        tag -> Text,
    }
}

joinable!(tags -> apis (api_id));

allow_tables_to_appear_in_same_query!(
    apis,
    tags,
);
