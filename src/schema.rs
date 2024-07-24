// @generated automatically by Diesel CLI.

diesel::table! {
    attributes (id) {
        id -> Integer,
        name -> Text,
        category -> Nullable<Text>,
        description -> Nullable<Text>,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    game_attributes (game_id, attribute_id) {
        game_id -> Nullable<Integer>,
        attribute_id -> Nullable<Integer>,
    }
}

diesel::table! {
    games (id) {
        id -> Integer,
        title -> Text,
        note -> Nullable<Text>,
    }
}

diesel::joinable!(game_attributes -> attributes (attribute_id));
diesel::joinable!(game_attributes -> games (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    attributes,
    game_attributes,
    games,
);
