#!/bin/bash

# -- 1. postgres cmd --
# brew install postgresql
# cargo install diesel_cli --no-default-features --features postgres
# docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=monster_hunter -d -p 5432:5432 postgres

# docker stop postgres
# docker rm postgres

# -- 2. diesel cmd --
diesel setup
diesel migration generate monster_hunter
for dir in migrations/*_monster_hunter; do
    if [ -d "$dir" ]; then
        cp init.sql "$dir/up.sql"
    fi
done
diesel migration run

# -- 3. spider cmd --
# 更新世界怪物信息
# curl -X POST "http://localhost:9876/api/monster/info/update" \
#     -H "Content-Type: application/json" \
#     -d '{"game_type": 0}'

# 更新崛起怪物信息
# curl -X POST "http://localhost:9876/api/monster/info/update" \
#     -H "Content-Type: application/json" \
#     -d '{"game_type": 1}'
