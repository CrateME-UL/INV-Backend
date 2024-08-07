# Docker

## 1. Postgres and SQLx

### if you only want the db -> in the .env file dir -> make sure to change `inv-db` to `localhost` in the .env file

```bash
docker run -d --name inv-db-standalone --env-file .env -p 5432:5432 postgres:latest
```

### to launch only, not create

```bash
docker start inv-db-standalone
```

### general setup (for compose)

- setup the database connection with the environment variable for example in a .env file in the src directory (replace the values of [...] corresponding in DBeaver connection form into your connection string) to launch backend with Rust and access the DB via the backend API

```bash
# database
POSTGRES_USER=<...>
POSTGRES_DB=<...>
POSTGRES_PASSWORD=<...>

# server
DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@inv-db:5432/$POSTGRES_DB

# log
RUST_LOG=debug

# ui -> change localhost to DNS name
VITE_API_ENDPOINT=http://localhost/api/v0
```

## 2. run docker-compose (for all services in a network) Make sure to create a token on github. Make sure to pull the good versions -> change the docker-compose if needed ex: `docker pull ghcr.io/crateme-ul/inv-<repository>:<tag>`. check this link to connect with the github container registery

https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-with-a-personal-access-token-classic

```bash
docker compose up --build -d
```

## 3. access the database in the docker compose with the .env variables

```bash
docker-compose exec inv-db sh -c 'psql -U $POSTGRES_USER $POSTGRES_DB'
```

### useful commands for the psql terminal (you can also copy paste normal sql commands)

```bash
\dt
\q
```

### to upload excel to sql, create a new database, create the tables, run the script, downlaod the script with sql commands, then use psql to insert the entries in the database. make sure that the database has empty tables before doing that. because the database is small, we can afford that

## to deploy: use the commands one at a time for deploy_compose.sh in the cloud shell aws
