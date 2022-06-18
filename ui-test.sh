#!/bin/bash
set -xeuo pipefail
RESULT=$(for file in test/fixtures/*; do
	cargo run --release -q 2>&1 -- <"${file}";
done)

diff -u ./expected-cli.out <(echo "${RESULT}")
