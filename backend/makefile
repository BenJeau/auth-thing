gen-openapi:
	cargo run --bin gen-openapi

run-db:
	cd database && cargo sqlx database setup

reset-db:
	cd database && $(if $(DB_FILE),DATABASE_URL=sqlite://./$(DB_FILE)) cargo sqlx database drop -y
	cd database && $(if $(DB_FILE),DATABASE_URL=sqlite://./$(DB_FILE)) cargo sqlx database setup
	cd database && source .env && $(if $(DB_FILE),DATABASE_URL=sqlite://./$(DB_FILE),DATABASE_URL=$$DATABASE_URL) cargo run --bin seeder $${SEED_FILES:-data.sql}

delete-db:
	if [ -n "$(DB_FILE)" ]; then rm -f "./database/$(DB_FILE)" "./database/$(DB_FILE)-wal" "./database/$(DB_FILE)-shm"; fi

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