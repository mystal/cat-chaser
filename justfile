set shell := ["nu", "-c"]

default:
  just --list

check:
  cargo check

run:
  cargo run

dist:
  cargo build --profile dist --no-default-features

dist-itch:
  RUSTFLAGS=--cfg=web_sys_unstable_apis \
    trunk build --public-url ./ --release --no-default-features -d builds/html5 index.html
