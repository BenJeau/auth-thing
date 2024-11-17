gen-openapi:
	cargo run --bin gen-openapi

run-db:
	cargo sqlx database setup

reset-db:
	cd database && cargo sqlx database drop -y
	cd database && cargo sqlx database setup

link-openapi:
	npx @redocly/cli@latest lint ./openapi.json 

test:
	cargo nextest run

lint:
	cargo clippy -- -D warnings

format:
	cargo fmt

mutation-test:
	cargo mutants -j5

coverage:
	cargo llvm-cov nextest
	cargo llvm-cov report --html

db-prepare:
	cd database && cargo sqlx prepare