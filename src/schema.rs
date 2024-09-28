// @generated automatically by Diesel CLI.

diesel::table! {
    mh_monster (id) {
        id -> Nullable<Integer>,
        monster_id -> Integer,
        monster_name -> Text,
        monster_type -> Integer,
    }
}
