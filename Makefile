# ChocAn

all:
	cargo fmt && cargo clippy && cargo test

release:
	cargo build --release

clean:
	rm test_*
