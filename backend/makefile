gen-openapi:
	cargo run --bin gen-openapi

run-db:
	cargo sqlx database setup

reset-db:
	cd database && cargo sqlx database drop -y
	cd database && cargo sqlx database setup

link-openapi:
	npx @redocly/cli@latest lint ./openapi.json 
