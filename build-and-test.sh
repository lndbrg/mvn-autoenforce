#!/bin/bash
set -xeuo pipefail
cargo fmt -- --check;
cargo clippy -- -Dwarnings;
cargo test;