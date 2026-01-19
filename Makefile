.PHONY: build run test clean integration-test

build:
	cargo build

run:
	cargo run

test:
	cargo test

clean:
	cargo clean
	rm -rf example-project

integration-test: build
	rm -rf example-project
	mkdir -p example-project
	cd example-project && printf "\n\n\n\nsecret\n\n\nsecret\n\n" > input.txt && ../target/debug/rice-cli < input.txt
	@echo "Checking generated files..."
	@test -f example-project/rice.config.js
	@test -f example-project/.env
	@echo "Generated .env content:"
	@cat example-project/.env
	@echo "Integration test passed!"
