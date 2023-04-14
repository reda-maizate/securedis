#!/bin/sh

# Get environment variables from .env file
set -a
. ./.env
set +a

export RUST_LOG=$RUST_LOG
exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/codecrafters-redis-target \
    --manifest-path $(dirname $0)/Cargo.toml "$@"
