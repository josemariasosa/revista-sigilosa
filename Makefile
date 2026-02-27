.PHONY: help up down run build test clean migrate showMigrations importData resetDb logs

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

up: ## Start PostgreSQL database with Docker
	@echo "🚀 Starting PostgreSQL with Docker..."
	@if ! docker info > /dev/null 2>&1; then \
		echo "❌ Docker is not running. Please start Docker Desktop first."; \
		echo "   On macOS: Open Docker Desktop application"; \
		echo "   Or run: open -a Docker"; \
		exit 1; \
	fi
	docker-compose up -d
	@echo "✅ PostgreSQL is running at localhost:5432"
	@echo "   Database: sonido_sigiloso"
	@echo "   User: postgres"
	@echo "   Password: postgres"
	@echo ""
	@echo "💡 Run 'make run' to start the application"

down: ## Stop PostgreSQL database
	@echo "🛑 Stopping PostgreSQL..."
	docker-compose down
	@echo "✅ PostgreSQL stopped"

logs: ## Show PostgreSQL logs
	docker-compose logs -f postgres

run: ## Run the application in development mode
	cargo run

build: ## Build the application in release mode
	cargo build --release

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

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

resetDb: ## Reset PostgreSQL database (drops and recreates)
	@echo "🗑️  Resetting PostgreSQL database..."
	@docker-compose exec postgres psql -U postgres -c "DROP DATABASE IF EXISTS sonido_sigiloso;"
	@docker-compose exec postgres psql -U postgres -c "CREATE DATABASE sonido_sigiloso;"
	@echo "✅ Database reset."
	@echo "   Run 'make run' to recreate with migrations and init data."
