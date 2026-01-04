.PHONY: run build clean release

run:
	cargo run --release

build:
	cargo build --release

clean:
	cargo clean

release: build
	@echo "Binary at: target/release/dinorustrun"
