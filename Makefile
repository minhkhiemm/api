
.PHONY: up down ps logs restart clean build run create-user get-users psql start-api db-info

# Start PostgreSQL container in detached mode
up:
	docker-compose up -d

# Stop containers
down:
	docker-compose down

# Show running containers
ps:
	docker-compose ps

# Show PostgreSQL container logs
logs:
	docker-compose logs postgres

# Follow PostgreSQL container logs
logs-follow:
	docker-compose logs -f postgres

# Restart PostgreSQL container
restart:
	docker-compose restart postgres

# Clean up containers and volumes
clean:
	docker-compose down -v

# Build Rust project
build:
	cargo build

# Run Rust project
run:
	cargo run

# Start everything (database and application)
start: up build run

# Stop everything
stop: down

# Create a new user
create-user:
	curl -X POST http://localhost:8080/users \
		-H "Content-Type: application/json" \
		-d '{"id": 0, "name": "John Doe", "email": "john@example.com"}'

# Get all users
get-users:
	curl http://localhost:8080/users

# Connect to PostgreSQL database
psql:
	docker exec -it api_postgres psql -U postgres -d api_db

# Start API (database and application)
start-api:
	docker-compose up -d && cargo run

# Show database connection info
db-info:
	@echo "Database connection information:"
	@echo "Host: localhost"
	@echo "Port: 5432"
	@echo "Database: api_db"
	@echo "Username: postgres"
	@echo "Password: postgres"