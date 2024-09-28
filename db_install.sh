#!/bin/bash

diesel setup
diesel migration generate monster_hunter
for dir in migrations/*_monster_hunter; do
    if [ -d "$dir" ]; then
        cp init.sql "$dir/up.sql"
    fi
done
diesel migration run
