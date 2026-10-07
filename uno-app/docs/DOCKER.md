# Docker Build and Run Guide

This guide covers building and running uno-app with Docker, including the PostgreSQL database.

## Prerequisites

- Docker installed and running (Docker Desktop, Colima, or Podman)
- For Colima users: ensure adequate resources and network access

### Colima Setup (Recommended for macOS)

```bash
# Start Colima with recommended resources
colima start --cpu 6 --memory 8 --disk 50 --network-address

# Verify Docker is working
docker info
```

## Building the Docker Image

The Dockerfile uses a multi-stage build with Rust 1.92.0 and produces a slim Debian-based runtime image.

### Build Command

Run from the repository root (`u-network/`), not from `uno-app/`:

```bash
cd /path/to/u-network

# Standard build
docker build -f uno-app/Dockerfile -t katson360/uno-app:latest .

# Build with progress output (useful for debugging)
docker build --progress=plain -f uno-app/Dockerfile -t katson360/uno-app:latest .
```

### Build Notes

- **Build time**: 15-25 minutes (first build), faster with cached layers
- **Image size**: ~206MB
- **Architecture**: Builds for the host architecture (arm64 on Apple Silicon)

## Running PostgreSQL Database

### Start PostgreSQL Container

```bash
# Create and start postgres container
docker run -d \
  --name uno-postgres \
  -p 5438:5432 \
  -e POSTGRES_DB=amba \
  -e POSTGRES_USER=user \
  -e POSTGRES_PASSWORD=password \
  -v uno-postgres-data:/var/lib/postgresql/data \
  postgres:15-alpine

# Verify it's running
docker ps | grep uno-postgres
```

### PostgreSQL Management Commands

```bash
# Stop postgres
docker stop uno-postgres

# Start existing postgres container
docker start uno-postgres

# View logs
docker logs uno-postgres

# Connect to database with psql
docker exec -it uno-postgres psql -U user -d amba

# Remove container (data preserved in volume)
docker rm uno-postgres

# Remove container AND data
docker rm uno-postgres
docker volume rm uno-postgres-data
```

## Running uno-app

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:password@host.docker.internal:5438/amba` |
| `CSRF_SECRET_KEY` | CSRF protection secret (32+ chars) | `your-csrf-secret-key-at-least-32-characters` |
| `ADMIN_API_KEY` | Admin API authentication key (32+ chars) | `your-admin-api-key-at-least-32-characters` |
| `LEPTOS_ENV` | Environment mode | `DEV` or `PROD` |
| `RUST_ENV` | Rust environment | `development` or `production` |

### Start uno-app Container (Development Mode)

```bash
docker run -d \
  --name uno-app \
  -p 3000:3000 \
  -e DATABASE_URL="postgres://user:password@host.docker.internal:5438/amba" \
  -e CSRF_SECRET_KEY="local-development-csrf-secret-key-32chars" \
  -e ADMIN_API_KEY="local-development-admin-api-key-32chars" \
  -e LEPTOS_ENV=DEV \
  -e RUST_ENV=development \
  -e RUST_LOG=info \
  katson360/uno-app:latest
```

### Start uno-app Container (Production Mode)

Production mode requires additional configuration:

```bash
docker run -d \
  --name uno-app \
  -p 3000:3000 \
  -e DATABASE_URL="postgres://user:password@db-host:5432/amba" \
  -e CSRF_SECRET_KEY="your-production-csrf-secret-key-32chars" \
  -e SESSION_SECRET="your-production-session-secret-32chars" \
  -e ADMIN_API_KEY="your-production-admin-api-key-32chars" \
  -e ADMIN_CLIENT_ID="your-admin-client-id" \
  -e ADMIN_SECRET_KEY="your-admin-secret-key" \
  -e LEPTOS_ENV=PROD \
  -e RUST_ENV=production \
  -e RUST_LOG=info \
  -v /path/to/media:/var/lib/uno-app/media \
  katson360/uno-app:latest
```

### uno-app Management Commands

```bash
# Stop uno-app
docker stop uno-app

# Start existing uno-app container
docker start uno-app

# View logs
docker logs uno-app

# Follow logs in real-time
docker logs -f uno-app

# Check health status
curl http://localhost:3000/api/v1/ready

# Remove container
docker rm -f uno-app
```

## Running Both Containers Together

### Quick Start Script

```bash
#!/bin/bash
# start-docker.sh

# Start postgres if not running
if ! docker ps | grep -q uno-postgres; then
    if docker ps -a | grep -q uno-postgres; then
        docker start uno-postgres
    else
        docker run -d \
            --name uno-postgres \
            -p 5438:5432 \
            -e POSTGRES_DB=amba \
            -e POSTGRES_USER=user \
            -e POSTGRES_PASSWORD=password \
            -v uno-postgres-data:/var/lib/postgresql/data \
            postgres:15-alpine
    fi
    echo "Waiting for postgres to be ready..."
    sleep 3
fi

# Start uno-app if not running
if ! docker ps | grep -q "uno-app"; then
    docker rm -f uno-app 2>/dev/null
    docker run -d \
        --name uno-app \
        -p 3000:3000 \
        -e DATABASE_URL="postgres://user:password@host.docker.internal:5438/amba" \
        -e CSRF_SECRET_KEY="local-development-csrf-secret-key-32chars" \
        -e ADMIN_API_KEY="local-development-admin-api-key-32chars" \
        -e LEPTOS_ENV=DEV \
        -e RUST_ENV=development \
        -e RUST_LOG=info \
        katson360/uno-app:latest
fi

echo "Services started:"
echo "  uno-app:  http://localhost:3000"
echo "  postgres: localhost:5438"
```

### Stop All Services

```bash
docker stop uno-app uno-postgres
```

## Troubleshooting

### Network Issues During Build

If you see timeout errors during `cargo` dependency downloads:

1. Restart Colima with network-address flag:
   ```bash
   colima stop
   colima start --cpu 6 --memory 8 --network-address
   ```

2. Verify network connectivity:
   ```bash
   docker run --rm alpine wget -qO- https://crates.io
   ```

### Database Connection Errors

- **"password authentication failed"**: Check `POSTGRES_USER` and `POSTGRES_PASSWORD` match between containers
- **"connection refused"**: Ensure postgres container is running and port mapping is correct
- Use `host.docker.internal` (not `localhost`) to connect from uno-app to postgres

### Container Exits Immediately

Check logs for specific errors:
```bash
docker logs uno-app
```

Common issues:
- Missing required environment variables (`SESSION_SECRET`, `CSRF_SECRET_KEY`, `ADMIN_API_KEY`)
- Database not accessible
- Invalid secret lengths (must be 32+ characters in production)

### "Session configuration error"

This occurs in production mode without proper session configuration. Either:
- Set `RUST_ENV=development` and `LEPTOS_ENV=DEV` for development
- Or configure all required production secrets

## Optional Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Logging level | `info` |
| `GEOIP_DATABASE_PATH` | Path to GeoLite2 database | (disabled) |
| `WEBHOOK_URL` | Outbox webhook URL | (disabled) |
| `ADMIN_CLIENT_ID` | Admin API client ID | (disabled) |
| `ADMIN_SECRET_KEY` | Admin API secret key | (disabled) |
| `FILE_STORAGE_BACKEND` | Storage backend (`local`, `gcs`) | `local` |
| `FILE_STORAGE_LOCAL_PATH` | Local file storage path | `/var/lib/uno-app/media` |

## Image Details

- **Base image**: `debian:bookworm-slim`
- **Exposed port**: 3000
- **Health check**: `curl http://localhost:3000/api/v1/ready`
- **User**: `uno` (UID 1000)
- **Working directory**: `/app`

---

## Podman Alternative

Podman can be used as a Docker alternative. Most commands are identical, just replace `docker` with `podman`.

### Podman Setup (macOS)

```bash
# Install Podman
brew install podman

# Initialize and start Podman machine
podman machine init --cpus 6 --memory 8192 --disk-size 50
podman machine start

# Verify Podman is working
podman info
```

### Podman Commands

Podman commands are drop-in replacements for Docker:

```bash
# Build image (from u-network/ root)
podman build -f uno-app/Dockerfile -t katson360/uno-app:latest .

# Run PostgreSQL
podman run -d \
  --name uno-postgres \
  -p 5438:5432 \
  -e POSTGRES_DB=amba \
  -e POSTGRES_USER=user \
  -e POSTGRES_PASSWORD=password \
  -v uno-postgres-data:/var/lib/postgresql/data \
  postgres:15-alpine

# Run uno-app
podman run -d \
  --name uno-app \
  -p 3000:3000 \
  -e DATABASE_URL="postgres://user:password@host.containers.internal:5438/amba" \
  -e CSRF_SECRET_KEY="local-development-csrf-secret-key-32chars" \
  -e ADMIN_API_KEY="local-development-admin-api-key-32chars" \
  -e LEPTOS_ENV=DEV \
  -e RUST_ENV=development \
  -e RUST_LOG=info \
  katson360/uno-app:latest
```

**Note**: With Podman, use `host.containers.internal` instead of `host.docker.internal` for host networking.

### Podman Machine Management

```bash
# Stop machine
podman machine stop

# Start machine
podman machine start

# Remove machine (warning: removes all containers/images)
podman machine rm

# SSH into machine
podman machine ssh
```

### Known Issues with Podman

- **Network performance**: Podman on macOS may experience slower network performance during image pulls and dependency downloads compared to Colima/Docker
- **Host networking**: Use `host.containers.internal` instead of `host.docker.internal`
- If builds are timing out, consider using Docker with Colima instead (see Colima Setup section above)
