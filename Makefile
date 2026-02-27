.PHONY: help run build test clean migrate showMigrations importData resetDb

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

run: ## Run the application in development mode
	cargo run

build: ## Build the application in release mode
	cargo build --release

test: ## Run tests
	cargo test

clean: ## Clean build artifacts and database
	cargo clean
	rm -f sonido_sigiloso.db
	rm -f sonido_sigiloso.db-shm
	rm -f sonido_sigiloso.db-wal

migrate: ## Run migrations (happens automatically on 'make run')
	@echo "🔄 Migrations run automatically when starting the app."
	@echo "   Use 'make run' to start the app and apply migrations."
	@echo ""
	@echo "Or manually run:"
	@echo "   cargo run --bin revista-sigilosa"

showMigrations: ## Show all available migrations
	@bash scripts/show_migrations.sh

importData: ## Import additional data from JSON file (requires app running)
	@echo "📥 Importing data from seed_data.json..."
	@bash scripts/import_seed.sh

resetDb: ## Reset database (will be re-initialized on next run)
	@echo "🗑️  Removing database..."
	@rm -f sonido_sigiloso.db sonido_sigiloso.db-shm sonido_sigiloso.db-wal
	@echo "✅ Database removed."
	@echo "   Run 'make run' to recreate with migrations and init data."
