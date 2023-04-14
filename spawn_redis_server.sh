#!/bin/sh
#
# DON'T EDIT THIS!
#
# CodeCrafters uses this file to test your code. Don't make any changes here!
#
# DON'T EDIT THIS!
export RUST_LOG="debug"
exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/codecrafters-redis-target \
    --manifest-path $(dirname $0)/Cargo.toml "$@"
