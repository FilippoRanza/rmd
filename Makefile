build: test
	cargo build --release
	cp target/release/rmd .

test: src/* cli_test.sh
	cargo test
	bash cli_test.sh