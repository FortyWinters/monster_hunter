#!/bin/bash

# -- 1. build postgres container --
# brew install postgresql
# cargo install diesel_cli --no-default-features --features postgres
# docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=monster_hunter -d -p 5432:5432 postgres

# -- 2. diesel migration --
diesel setup
diesel migration generate monster_hunter
for dir in migrations/*_monster_hunter; do
    if [ -d "$dir" ]; then
        cp init.sql "$dir/up.sql"
    fi
done
diesel migration run

# -- 3. run spider --
# curl -X POST "http://localhost:9876/api/monster/info/update" \
#     -H "Content-Type: application/json" \
#     -d '{"game_type": 0}'

# curl -X POST "http://localhost:9876/api/monster/info/update" \
#     -H "Content-Type: application/json" \
#     -d '{"game_type": 1}'
