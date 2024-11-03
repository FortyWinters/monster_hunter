// @generated automatically by Diesel CLI.

diesel::table! {
    monster_info (id) {
        id -> Int4,
        monster_id -> Int4,
        monster_name -> Text,
        monster_type -> Int4,
        monster_description -> Nullable<Text>,
        monster_icon_url -> Nullable<Text>,
        game_type -> Int4,
    }
}

diesel::table! {
    monster_weakness (id) {
        id -> Int4,
        monster_id -> Int4,
        part_name -> Text,
        weakness_type -> Int4,
        weakness_value -> Int4,
        game_type -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    monster_info,
    monster_weakness,
);
