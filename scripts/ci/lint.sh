#!/usr/bin/env bash
#
# This script lints the system
#

cargo --color always fmt --check
cargo clippy --color always --all-features --tests --examples --bins
