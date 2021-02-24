# Notepad
Yes, a simple notepad made with a Rusty backend, Rocket & Diesel.

## Usage
```bash
# install rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly

# start postgresql and seed the database
psql -f init.sql
cargo install diesel_cli --no-default-features --features "postgres"
diesel migration run

cargo run
```

inspired by https://github.com/TatriX/realworld-rust-rocket