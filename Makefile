build: test
	cargo build --release


test: src/* cli_test.sh
	cargo test
	bash cli_test.sh