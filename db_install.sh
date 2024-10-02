#!/bin/bash
# brew install postgresql
# cargo install diesel_cli --no-default-features --features postgres
# docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=monster_hunter -d -p 5432:5432 postgres

diesel setup
diesel migration generate monster_hunter
for dir in migrations/*_monster_hunter; do
    if [ -d "$dir" ]; then
        cp init.sql "$dir/up.sql"
    fi
done
diesel migration run
