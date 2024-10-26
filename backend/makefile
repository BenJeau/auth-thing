gen-openapi:
	cargo run --bin gen-openapi

run-db:
	cargo sqlx database setup

reset-db:
	cargo sqlx database drop
	cargo sqlx database setup

link-openapi:
	npx @redocly/cli@latest lint ./openapi.json 
