# ChocAn

all:
	cargo fmt && cargo clippy && cargo test -- --test-threads=1

release:
	cargo build --release

.PHONY: clean

clean:
	rm -rf test_* emails/
