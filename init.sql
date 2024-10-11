DROP TABLE IF EXISTS monster_info;
DROP TABLE IF EXISTS monster_part;
DROP TABLE IF EXISTS monster_weakness;

CREATE TABLE monster_info (
    id SERIAL PRIMARY KEY,
    monster_id INTEGER NOT NULL,
    monster_name TEXT NOT NULL,
    monster_type INTEGER NOT NULL,
    monster_description TEXT,
    monster_icon_url TEXT,
    game_type INTEGER NOT NULL -- 0: World, 1: Rise, 2: Wild
);

CREATE TABLE monster_part (
    id SERIAL PRIMARY KEY,
    monster_id INTEGER NOT NULL,
    part_id INTEGER NOT NULL,
    part_name TEXT NOT NULL
);

CREATE TABLE monster_weakness (
    id SERIAL PRIMARY KEY,
    part_id INTEGER NOT NULL,
    weakness_name TEXT NOT NULL,
    weakness_level INTEGER NOT NULL
);