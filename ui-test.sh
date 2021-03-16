#!/bin/bash
set -xeuo pipefail
RESULT=$(for file in test/fixtures/*; do
	cat $file | cargo run --release -q;
done)

diff -u ./expected-cli.out <(echo "${RESULT}")
