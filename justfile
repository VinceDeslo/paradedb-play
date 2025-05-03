__default:
    just --list

up:
    docker compose up

down:
    docker compose down -v

connect:
    psql $DATABASE_URL

fts:
    cargo run --quiet
