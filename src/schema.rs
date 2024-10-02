// @generated automatically by Diesel CLI.

diesel::table! {
    mh_monster (id) {
        id -> Int4,
        monster_id -> Int4,
        monster_name -> Text,
        monster_type -> Int4,
    }
}
