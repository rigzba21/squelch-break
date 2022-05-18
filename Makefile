all: build_sbhandler 

build_sbhandler:
	cargo build --bin sbhandler

run_handler:
	RUST_LOG=trace cargo run --bin sbhandler

clean:
	rm -rf target	
