# Todo List Apps

Todo list cli app build with Rust

## Prerequisite

1. Make sure you have Postgres running.
1. Make sure you have installed `diesel-cli`. This project is using posgresql as the main DB
    ```
    cargo install diesel_cli --no-default-features --features postgres
    ```
    > Notes:<br />
    > Sometimes you need to install `libpq` for supporting postgres backend
1. Creating `.env` file. _reference: `.env.example`_
1. Set `DATABASE_URL` in the `.env` file.
1. Initialize diesel using: `diesel setup`.
1. Run the migration using: `diesel migration run`.

## How to setup

1. Install dependency and build app using cargo
    ```sh
   cargo build --release
    ```
1. Create a copy or link to `target/release/rustodo`. I prefer using symlink.
    ```
    ln -s target/release/rustodo rustodo
    ```
1. Run the app
    ```bash
    # Run interactively using:
    ./rustodo

    # Or 
    ./rustodo add "Create new todo list api" "Testing todo list app"
    ./rustodo list
    ```
