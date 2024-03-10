#!/usr/bin/env bash
set -x
set -eo pipefail

#check for prereqs

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install --version='~0.7' sqlx-cli \
--no-default-features --features rustls,postgres"
    echo >&2 "to install it."
    exit 1
fi


DB_USER="${POSTGRES_USER:=postgres}"

DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

DB_NAME="${POSTGRES_DB:=newsletter}"

DB_PORT="${POSTGRES_PORT:=5432}"

DB_HOST="${POSTGRES_HOST:=localhost}"

CONTAINER_NAME=newsletter

# Allow to skip Docker if a dockerized Postgres data base is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
    if [[ $(docker ps -a --filter="name=${CONTAINER_NAME}" --filter "status=exited" | grep -w "${CONTAINER_NAME}") ]]; then
        echo "docker start ... TODO!"
    elif [[ $(docker ps -a --filter="name=${CONTAINER_NAME}" --filter "status=running" | grep -w "${CONTAINER_NAME}") ]]; then
        echo "docker still running TODO!"
    else
        echo "docker run ..."
        docker run\    
            --name "${CONTAINER_NAME}"\
            -e POSTGRES_USER=${DB_USER}\
            -e POSTGRES_PASSWORD=${DB_PASSWORD}\
            -e POSTGRES_DB=${DB_NAME}\
            -p "${DB_PORT}":5432 \
            -d postgres \
            postgres -N 1000
    fi
fi

#Keep pinging postgres until its ready
export PGPASSWORD="${DB_PASSWORD}"

until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do 
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_POR}! running migrations"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
#export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"

