# Start the PostgreSQL container
make up

# Check running containers
make ps

# View PostgreSQL logs
make logs

# Follow logs in real-time
make logs-follow

# Restart PostgreSQL
make restart

# Stop containers
make down

# Remove containers and volumes
make clean

# Build and run the Rust application
make build
make run

# Start everything (database and application)
make start

# Stop everything
make stop

# Show database connection information
make db-info

# Create a new user
make create-user

# Get all users
make get-users

# Connect to PostgreSQL database
make psql

# Start both database and API in one command
make start-apit