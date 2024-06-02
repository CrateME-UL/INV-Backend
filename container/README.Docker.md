# Docker
## 1. create Docker image for Rust server (you will need to run this to apply changes in the server folder)
```
docker build -t inv-server .
```
## 2. Postgres and SQLx
### if you only want the db -> in the .env file dir -> make sure to change `inv-db` to `localhost` in the .env file
```console
docker run -d --name inv-db-standalone --env-file .env -p 5432:5432 postgres:latest
```
### to launch only, not create
```console
docker start inv-db-standalone
```

### general setup
- setup the database connection with the environment variable for example in a .env file in the src directory (replace the values of [...] corresponding in DBeaver connection form into your connection string) to launch backend with Rust and access the DB via the backend API
- **Make Sure to NOT track the .env file on Github!!**
```
# database
POSTGRES_USER=<...>
POSTGRES_DB=<...>
POSTGRES_PASSWORD=<...>

# server
DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@inv-db:5432/$POSTGRES_DB

# log
RUST_LOG=debug
```
## 3. run docker-compose (for all services in a network) N.B: make sure to build inv-ui before!
```
docker compose up --build
```
## 4. click manually in docker desktop to start created containers (to look the content of the db for example)

### useful commands
```
docker login
docker stop some-postgres
docker ps
docker run -p 3000:3000 inv-server
```
