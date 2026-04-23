.PHONY: build run test lint docker-up clean

build:
	cargo build --release

run:
	cargo run

test:
	cargo test

lint:
	cargo clippy

docker-up:
	docker compose up --build -d

clean:
	cargo clean
