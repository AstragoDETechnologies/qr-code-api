set shell := ["nu", "-c"]
set windows-shell := ["nu", "-c"]

default: run

# ===== Running

# Run the Program.
run PROFILE="dev":
    cargo run --profile {{PROFILE}}

# Run the program with `trace` debugging.
trace PROFILE="dev":
    RUST_LOG=trace just run {{PROFILE}}

# Run the program with `debug` debugging.
debug PROFILE="dev":
    RUST_LOG=debug just run {{PROFILE}}

# Run the program with `info` debugging.
info PROFILE="dev":
    RUST_LOG=info just run {{PROFILE}}

# Run the program with `warn` debugging.
warn PROFILE="dev":
    RUST_LOG=warn just run {{PROFILE}}

# Run the program with `error` debugging.
error PROFILE="dev":
    RUST_LOG=error just run {{PROFILE}}

# == Watch

# Watch for file changes and re-run the program in case they change.
watch RUST_LOG="default" PROFILE="dev":
    RUST_LOG={{RUST_LOG}} cargo watch -c -x "run --profile {{PROFILE}}"

# ===== Building

# Build the program.
build PROFILE="dev":
    cargo build --profile {{PROFILE}}

# ==== Docker

start:
    git pull
    docker compose up --build --force-recreate -d

stop:
    docker compose down

logs:
    docker compose logs
