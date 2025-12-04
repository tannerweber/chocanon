# ChocAn

all:
	cargo fmt && cargo clippy && cargo test -- --test-threads=1

release:
	cargo build --release

docs:
	cargo doc --no-deps --target-dir docs

.PHONY: clean

clean:
	rm -rf test_* emails/
