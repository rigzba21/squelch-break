all: build_cli start_server

build_cli:
	cargo build --bin sb --release

start_server:
	cargo run --bin sb-server

clean:
	rm -rf target	