set shell := ["nu", "-c"]

default:
  just --list

check:
  cargo check

run:
  cargo run

web:
  RUSTFLAGS=--cfg=web_sys_unstable_apis \
    trunk serve --no-default-features -d target/web-dev src_assets/index-dev.html

dist:
  cargo build --profile dist --no-default-features

dist-itch:
  RUSTFLAGS=--cfg=web_sys_unstable_apis \
    trunk build --public-url ./ --release --no-default-features -d builds/web-itch src_assets/index-itch.html
