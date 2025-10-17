#!/bin/sh
apk add rustup build-base libressl-dev
rustup-init -y
source "$HOME/.cargo/env"
cargo build --release
