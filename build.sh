#!/usr/bin/env bash
set -euo pipefail
cargo build
cp target/debug/linear-cli ~/scripts/_lr
