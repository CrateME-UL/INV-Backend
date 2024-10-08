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

## setup rust

- install rust on windows from https://www.rust-lang.org/tools/install
- check your rust version: rustc --version
- check for the latest version of rust : rustup update

## setup after installing rust linux

```bash
sudo apt-get update
sudo apt-get install libssl-dev pkg-config clang libclang-dev postgresql-client
```

## to install sqlx-cli to get offline mode and run in the server dir and excel_to_sql

```bash
rustup update stable
cargo install sqlx-cli
export SQLX_OFFLINE=true
export export DATABASE_URL=postgres://some-postgres:mysecretpassword@localhost:5432/some-postgres
cargo sqlx prepare --workspace
# or for non workspcace
cargo sqlx prepare
```

### run with cargo package (preferred)

- check cargo version: cargo --version
- to build: cargo build
- to execute in dev: cargo run
- to use release profile (in prod): cargo run --release

## Docker setup

follow the [docker-compose setup](./README.Docker.md) to launch containers locally or run only the database container

## Rust lint code

```
cargo clippy --all-targets --all-features
```

## Rust format code

```
cargo fmt
```

## REMARKS:

- we make the choice to use tokio for the runtime, axum for the web framework and sqlx-tokio for the database.
- to compensate the lack of flexibility, we will delegate a minimum off resources to these frameworks to ensure the code lives as long as possible and requires less maintenance
- for that, the domain will have most of the logic, the web part will be in the resource and the database part in the repository
- the domain will use traits so the repository and resource having a change of technology does not affect all the codebase
- we will combine the service and the domain as one, because the domain here is only used for types now
- service could be usefull for more complex use cases, but we want to reduce the number of adapters needed
- only the service has access to the domain again to prevent devastating changes to all the codebase, because it's protected by the traits on both sides
- we will do an abstraction for inMemory in the repository for easier testing. we will add a config to choose between the two at runtime. a slight trade off of performance for flexibility
- integration tests are in seperate files, but unit tests are in the same file
