all: build_sbhandler build_sbrunner

build_sbhandler:
	cargo build --bin sbhandler

build_sbrunner:
	cargo build --bin sbrunner

run_handler:
	RUST_LOG=trace cargo run --bin sbhandler

run_runner:
	RUST_LOG=trace cargo run --bin sbrunner

clean:
	rm -rf target	
