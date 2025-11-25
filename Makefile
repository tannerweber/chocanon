# ChocAn

all:
	cargo fmt && cargo clippy && cargo test -- --test-threads=1

release:
	cargo build --release

clean:
	rm test_*
	rm -r emails/
