# Auth Thing

A small (less than 10MB) self-hosted authentication and authorization system designed to provide secure user management for web applications.

![Screenshot](docs/screenshot.png)

Key features:
- User signup and authentication 
- Email verification system with configurable code length
- Action logging for security audit trails
- Per application JWT keys with support for multiple keys
- [in-progress] Two-factor authentication support
- [todo] Support for multiple applications/tenants via application slugs
- [todo] Configurable email settings
- [todo] Frontend integration options

The system is built with:
- Rust backend using Axum web framework
- React frontend with Tanstack Query and Router styled with TailwindCSS and Shadcn/UI
- Database integration for persistent storage
- Email delivery capabilities for verification
- Security features like password hashing
- Comprehensive API documentation using utoipa

The project aims to provide a robust, secure, and customizable authentication solution that can be self-hosted and integrated into various web applications while maintaining high security standards and user privacy.

## Docker

The Dockerfile at the root of the project creates a single Docker image that bundles the Rust backend and the frontend. It is currently less than 10MB in size (and one of my goal to keep it that way).

```
docker build -t auth-thing:latest .
```

## Development

Local development currently doesn't rely on Docker. You'll need Rust installed alongside Node.js and pnpm.

### Running the applicaiton

*Backend*

```
cd backend && cargo run --bin server
```

*Frontend*

```
cd frontend && pnpm dev
```

### Updating the OpenAPI schema

The frontend API client code is generated from the OpenAPI schema. If you make changes to the backend, you'll need to update the schema and regenerate the frontend code.

```
make gen-api-client
```

### Updating the database offline files

To run tests or to build the backend without needing a live database, you can generate the offline files.

```
cd backend && make db-prepare
```
