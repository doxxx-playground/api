# Rust API with PostgreSQL Database

This project implements a Rust-based API service with a PostgreSQL database backend, containerized using Docker for easy
deployment and development.

The application provides a RESTful API interface, leveraging Rust's performance and safety features. It uses a
PostgreSQL database for data persistence, with the database schema managed through SQL migrations. The entire stack is
containerized, allowing for consistent development and deployment across different environments.

## Repository Structure

- `docker-compose.yaml`: Defines the multi-container Docker environment
- `Dockerfile`: Contains instructions for building the Rust API Docker image
- `init.sql`: SQL script for initializing the database
- `migrations/`: Directory containing database migration scripts
    - `2024-12-20-124720_create_items/`: Migration for creating the 'items' table
        - `up.sql`: SQL for applying the migration
        - `down.sql`: SQL for reverting the migration
- `src/`: Contains the Rust source code for the API
    - `main.rs`: Entry point of the application
    - `lib.rs`: Library module definitions
    - `db.rs`: Database connection and query functions
    - `handlers.rs`: Request handler functions
    - `models.rs`: Data models and structures
    - `schema.rs`: Database schema definitions

## Usage Instructions

### Prerequisites

- Docker (version 20.10 or later)
- Docker Compose (version 1.29 or later)

### Installation

1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Build and start the containers:
   ```sh
   docker-compose up --build -d
   ```

This command will build the Rust API image and start both the API and PostgreSQL containers.

### Accessing the API

Once the containers are running, the API will be available at `http://localhost:8080` (assuming the default port
mapping).

### Database Management

The PostgreSQL database is accessible on the host machine at `localhost:5432`. You can connect to it using the following
credentials:

- Username: myuser
- Password: mypass
- Database: mydb

### Running Migrations

Database migrations are managed using SQL scripts in the `migrations/` directory. To apply migrations:

1. Connect to the running API container:
   ```sh
   docker exec -it <api-container-name> /bin/bash
   ```

2. Run the migration command (assuming you're using a migration tool like `diesel`):
   ```sh
   diesel migration run
   ```

### Troubleshooting

#### API Container Fails to Start

If the API container fails to start, it might be due to the database not being ready. Check the logs:

```sh
docker-compose logs api
```

If you see connection errors, wait a few seconds and try restarting the API container:

```sh
docker-compose restart api
```

#### Database Connection Issues

If the API is unable to connect to the database, ensure that the database container is running and that the connection
details in the API configuration match those in `docker-compose.yaml`.

To check if the database is running:

```sh
docker-compose ps
```

If the database is not running, start it with:

```sh
docker-compose up -d db
```

#### Database Migration Issues

If you see errors like "relation already exists" when running migrations, it might be because the database volume
contains old data. You have two options:

1. Complete reset (Development):
   ```sh
   docker compose down -v   # Remove all containers and volumes
   docker compose up -d     # Start fresh
   ```

2. Clean migration (Production):
   ```sh
   docker exec -it rust-api-container diesel migration revert  # Revert last migration
   docker exec -it rust-api-container diesel migration run     # Apply migration again
   ```

#### Viewing Logs

To view logs for debugging:

- API logs: `docker-compose logs api`
- Database logs: `docker-compose logs db`

## API Specification

The API provides a RESTful interface for interacting with the application. The specific endpoints and their functionalities are implemented in the `handlers.rs` file. Below is a general overview of the API structure:

### GET Endpoints

GET endpoints are typically used to retrieve resources.

Example:

```
GET /api/resource
GET /api/resource/{id}
```

### POST Endpoints

POST endpoints are used to create new resources.

Example:

```
POST /api/resource
```

### PUT Endpoints

PUT endpoints are used to update existing resources.

Example:

```
PUT /api/resource/{id}
```

### DELETE Endpoints

DELETE endpoints are used to remove resources.

Example:

```
DELETE /api/resource/{id}
```

For detailed information about specific endpoints, request/response formats, and authentication requirements, please refer to the `handlers.rs` file in the source code.

## Data Flow

The data flow in this application follows these steps:

1. Client sends a request to the API endpoint.
2. The request is received by the Rust web server (likely using a framework like Actix or Rocket).
3. The appropriate handler function in `handlers.rs` processes the request.
4. If database access is required, the handler calls functions from `db.rs`.
5. `db.rs` uses the connection pool to query the PostgreSQL database.
6. The query results are mapped to Rust structures defined in `models.rs`.
7. The handler processes the data and prepares the response.
8. The response is sent back to the client.

```
[Client] <-> [Rust Web Server] <-> [Handlers] <-> [Database Functions] <-> [PostgreSQL]
                                      ^                 ^
                                      |                 |
                                      v                 v
                                 [Models]           [Schema]
```

Note: Ensure proper error handling and logging throughout this flow for robust operation.

## Infrastructure

The project uses Docker Compose to define and manage the infrastructure. The main components are:

### PostgreSQL Database

- **Type**: Docker container
- **Image**: postgres:latest
- **Container Name**: postgres-container
- **Environment Variables**:
    - POSTGRES_USER: myuser
    - POSTGRES_PASSWORD: mypass
    - POSTGRES_DB: mydb
- **Port Mapping**: 5432:5432
- **Volumes**:
    - postgres-data: Persistent volume for database data
    - ./init.sql:/docker-entrypoint-initdb.d/init.sql: Initialization script

### Rust API

- **Type**: Docker container
- **Build**: Custom Dockerfile in the project root
- **Base Image**: rust:1.82 (for building), debian:buster-slim (for runtime)
- **Dependencies**: libssl-dev (for PostgreSQL connectivity)
- **Entry Point**: ./api (the compiled Rust binary)

Note: The API container configuration is not explicitly defined in the provided docker-compose.yaml, but it can be inferred from the Dockerfile and project structure.
