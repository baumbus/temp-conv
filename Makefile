:PHONY: build

build:
    cargo build

run:
    cargo run

build-release:
    cargo build --release

run-release:
    cargo run --release

test:
    cargo test --no-fail-fast

docs:
    @cargo doc --open

clean:
    cargo clean
