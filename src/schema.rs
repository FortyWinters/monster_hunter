// @generated automatically by Diesel CLI.

diesel::table! {
    monster_rise (id) {
        id -> Int4,
        monster_id -> Int4,
        monster_name -> Text,
        monster_type -> Int4,
    }
}

diesel::table! {
    monster_wild (id) {
        id -> Int4,
        monster_id -> Int4,
        monster_name -> Text,
        monster_type -> Int4,
    }
}

diesel::table! {
    monster_world (id) {
        id -> Int4,
        monster_id -> Int4,
        monster_name -> Text,
        monster_type -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    monster_rise,
    monster_wild,
    monster_world,
);
