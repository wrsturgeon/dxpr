#!/bin/sh

set -eu

rustup toolchain install nightly
rustup component add clippy --toolchain nightly
cargo install rustfilt

export RUSTFLAGS='-Copt-level=0 -g'

export CARGOFLAGS='--features=std'
sh test-coverage.sh

export CARGOFLAGS='--features=std --release'
sh test-coverage.sh

export CARGOFLAGS='--no-default-features'
sh test-coverage.sh

export CARGOFLAGS='--no-default-features --release'
sh test-coverage.sh
