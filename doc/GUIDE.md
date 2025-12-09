# Noto Cloud: Comprehensive Guide

## Overview of File Structure

### Libraries (`libs`)
- `libs/client-api`: API client for interfacing with Noto-Cloud.
- `libs/database`: Houses database schema and migration scripts.
- `libs/database-entity`: Definitions for database entities.
- `libs/gotrue`: Contains the GoTrue Authentication Server code.
- `libs/gotrue-entity`: Entity definitions for the GoTrue Auth Server.
- `libs/realtime`: Realtime server implementation.
- `libs/collab-rt-entity`: Realtime server entity definitions.
- `libs/infra`: Scripts and tools for infrastructure management.
- `libs/app_error`: Custom error types specific to Noto-Cloud.

### Source Code (`src`)
- `src/api`: Endpoints and handlers for the Noto-Cloud API.
- `src/biz`: Core business logic of the application.
- `src/middleware`: Middleware components for API processing.

### Configuration and Migration
- `configurations`: Contains essential configuration files for various services.
- `migrations`: Scripts for managing and migrating the Postgres database.

## Service Routing and Access

### Access Points Post Deployment
After executing `docker compose up -d`, Noto-Cloud is accessible at `http://localhost` on ports 80 and 443 with the following routing:

- `/gotrue`: Redirects to the GoTrue Auth Server.
- `/api`: Noto-Cloud's HTTP API endpoint.
- `/ws`: WebSocket endpoint for Noto-Cloud.
- `/console`: User Admin Frontend for Noto.
- `/pgadmin`: Interface for Postgres database management.
- `/minio`: User interface for Minio object storage.
- `/`, `/app`: Noto Web.

![Deployment Architecture](../assets/images/deployment_arch.png)

## Dockerization and Continuous Integration

#### Docker Images
Noto leverages Docker for efficient deployment and scaling. Docker images are available at:
- `Noto_cloud`: [Docker Hub](https://hub.docker.com/repository/docker/Notoinc/Noto_cloud/general)
- `admin_frontend`: [Docker Hub](https://hub.docker.com/repository/docker/Notoinc/admin_frontend/general)
- `Noto_web`: [Docker Hub](https://hub.docker.com/repository/docker/Notoinc/Noto_web/general)

#### Automated Builds with GitHub Tags
The Docker images are automatically built and updated through a GitHub Actions workflow:

1. **Tag Creation**: A new tag in the GitHub repository indicates a new version or release.
2. **Automated Build Trigger**: This tag initiates the Docker image building process via GitHub Actions.
3. **Docker Hub Updates**: The `Noto_cloud` and `admin_frontend` images are updated on Docker Hub with the latest build.
