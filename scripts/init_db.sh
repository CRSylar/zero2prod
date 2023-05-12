#!/usr/bin/env bash

# set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
	echo >&2 "Error: psql is not installed."
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed."
	echo >&2 "Use: "
	echo >&2 "	cargo install sqlx-cli --no-default-features --features native-tls,postgres"
	echo >&2 "to install it"
	exit 1
fi

#Check if there's an user, or Default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
#Check if there's a password, or default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#Check if a db name is specified, or default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
#Check for a custom port, default 5432
DB_PORT="${POSTGRES_PORT:=5432}"

#Check for the uri host, default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Launch docker container with Postgres if is not already done
if [[ -z "${SKIP_DOCKER}" ]]
then
	docker run \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_DB=${DB_NAME} \
		-p "${DB_PORT}":5432 \
		-d postgres \
		postgres -N 1000
		# ^ increase max connection for testing purpose
	echo "Postgres image starting..."
fi
# Ping Postgres instance 'till is ready to accept connections
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}"  -c "\q"; do
	>&2 echo "Posgres is still unaivailable - sleeping"
	sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations"


DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Migrations done, ready to go!"

