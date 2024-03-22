set shell := ["nu", "-c"]

default:
  just --list

check:
  cargo check

run:
  cargo run

dist:
  cargo build --profile dist

itch-build:
  trunk build --public-url ./ --release --no-default-features
