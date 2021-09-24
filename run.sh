#!/usr/bin/env bash
set -euxo pipefail
die() { echo "$*" 1>&2 ; exit 1; }

[ -z "${1:""}" ] && die "No hub country specified - usage: cargo run -- -c USA"

xsv sort data.csv | tail -n +2 | cargo run -- -c $1
