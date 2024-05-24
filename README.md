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
### run without cargo package: 
   To compile: rustc main.rs
   To execute:.\main.exe
### run with cargo package (preferred)
- check cargo version: cargo --version
- to build: cargo build
- to execute: cargo run

## Docker setup
follow the [docker-compose setup](./rust/README.Docker.md) to launch containers locally or run only the database container


## Rust format code
```
cargo fmt
```