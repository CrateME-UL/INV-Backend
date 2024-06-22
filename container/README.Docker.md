# Docker

## 1. create Docker image for Rust server (you will need to run this to apply changes in the server folder to test it locally if you change the image)

```bash
docker build -t inv-server .
```

## 2. Postgres and SQLx

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
- **Make Sure to NOT track the .env file on Github!!**

```bash
# database
POSTGRES_USER=<...>
POSTGRES_DB=<...>
POSTGRES_PASSWORD=<...>

# server
DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@inv-db:5432/$POSTGRES_DB

# log
RUST_LOG=debug
```

## 3. run docker-compose (for all services in a network) N.B: make sure to build inv-ui before! Make sure to create a token on github

https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-with-a-personal-access-token-classic

```bash
docker compose up --build -d
```

## 4. access the database in the docker compose with the .env variables

```bash
docker-compose exec inv-db sh -c 'psql -U $POSTGRES_USER $POSTGRES_DB'
```

### useful commands for the psql terminal (you can also copy paste normal sql commands)

```bash
\dt
\q
```
