#!/bin/sh
set -euo pipefail

for file in test/fixtures/*; do
	cat $file | cargo run;
done
