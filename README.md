# Backend

## vscode extensions to install in general

- yzhang.markdown-all-in-one
- formulahendry.code-runner
- vsls-contrib.codetour
- wayou.vscode-todo-highlight
- streetsidesoftware.code-spell-checker
- hediet.vscode-drawio
- ms-vscode.live-server
- ms-azuretools.vscode-docker
- TabNine.tabnine-vscode
- jeff-hykin.polacode-2019
- johnpapa.vscode-peacock
- PKief.material-icon-theme
- usernamehw.errorlens
- mhutchie.git-graph
- eamodio.gitlens
- mikestead.dotenv
- aaron-bond.better-comments
- ms-vscode.notepadplusplus-keybindings
- ms-vsliveshare.vsliveshare

## vscode extensions to install for rust

- rust-lang.rust-analyzer
- serayuzgur.crates
- vadimcn.vscode-lldb
- tamasfe.even-better-toml


## setup rust on windows
- install rust on windows from https://www.rust-lang.org/tools/install
- check your rust version: rustc --version
- check for the latest version of rust : rustup update
### run without cargo package: 
   TO compile: rustc main.rs
   To execute:.\main.exe


### run with cargo package
- check cargo version: cargo --version
- to build: cargo build
- to execute: cargo run


## Docker
- to connect to docker: docker login
- download the container if not existing, container named some-postgres (we are specifing the port as it must be the same used in DBreave) :
```
docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=[Password] -d postgres
  ```
- show all the containers that are running with thier status, ports... : docker ps
- To launch your Postgres database and supporting services: docker compose -f docker-compose.yml up or in docker desktop
- to stop the container: docker stop some-postgres

## create Docker image for Rust server
```
docker build -t inv-backend .
```

## run Docker container
```
docker run -p 5000:5000 inv-backend
```

### Postgres and SQLx
- launch the DB on docker
- setup the database connection with the environment variable for example in a .env file in the src directory (replace the values of [...] corresponding in DBeaver connection form into your connection string) to launch backend with Rust and access the DB via the backend API
- **Make Sure to NOT track the .env file on Github!!**
```
# add this in a .env file in the src directory
DATABASE_URL=postgres://[Username]:[Password]@[Host]:[Port]/[Database]
```
### Rust format code
```
cargo fmt
```