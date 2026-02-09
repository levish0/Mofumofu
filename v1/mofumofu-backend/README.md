# 🐧 Mofumofu Backend

A soft and simple social blogging platform for everyone. Built with modern technologies for scalability and performance.

[![Version](https://img.shields.io/badge/version-0.7.2-blue.svg)](https://github.com/your-username/mofumofu-backend)
[![Rust](https://img.shields.io/badge/rust-1.86.0+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.10+-green.svg)](https://www.python.org/)

## 📋 Table of Contents

- [Features](#-features)
- [Architecture](#-architecture)
- [Tech Stack](#-tech-stack)
- [Quick Start](#-quick-start)
- [API Documentation](#-api-documentation)
- [Development](#-development)
- [Image Upload Specifications](#-image-upload-specifications)
- [Deployment](#-deployment)
- [Environment Configuration](#-environment-configuration)
- [Contributing](#-contributing)

## ✨ Features

### 🔐 Authentication & Security
- **JWT Authentication** with access/refresh token pattern
- **OAuth Integration** (Google, GitHub) with account linking/unlinking
- **Password Management** (set, reset, change) with secure email verification
- **Email Verification** with token-based validation
- **Structured Error Handling** with specific error codes (user:*, oauth:*, etc.)

### 👥 Social Blogging Platform
- **Rich Post Creation** with markdown support and syntax highlighting
- **Image Upload & Processing** with automatic WebP conversion and optimization
- **Social Interactions** - Follow/unfollow users, like posts
- **Real-time Search** powered by Meilisearch with auto-indexing
- **Trending Hashtags** with popularity algorithms
- **User Profiles** with customizable avatars and banners

### 🚀 Performance & Scalability
- **Microservices Architecture** with service isolation
- **Background Task Processing** with Celery workers (2 replicas)
- **Redis Caching** for improved response times
- **Database Optimization** with connection pooling
- **Async Processing** for image uploads and search indexing
- **Health Checks** for all services

### 🛠 Developer Experience
- **API-first Design** with comprehensive Swagger documentation
- **Multi-language Support** (Korean and English)
- **Docker Compose** for easy development setup
- **Structured Logging** with tracing and log rotation
- **Type-safe Database** queries with SeaORM

## 🏗 Architecture

Mofumofu Backend follows a modern microservices architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────────────┐
│                        Load Balancer                            │
└─────────────────────────────────────────────────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │                         │
┌──────────────────┐│    ┌─────────────────────┐│    ┌──────────────┐
│  Main Backend    ││    │    Tasks API       ││    │   Markdown   │
│  (Rust/Axum)    ││    │  (Python/FastAPI)  ││    │   Service    │
│   Port: 8000     ││    │    Port: 7000      ││    │ (Bun/Elysia) │
└──────────────────┘│    └─────────────────────┘│    │ Port: 6700   │
                    │                          │    └──────────────┘
                    │    ┌─────────────────────┐│
                    │    │   Celery Workers    ││
                    │    │  (Background Tasks) ││
                    │    └─────────────────────┘│
                    │                          │
                    │    ┌─────────────────────┐│
                    └────│      Redis          │
                         │ (Cache & Broker)    │
                         │   Port: 6379        │
                         └─────────────────────┘

┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   PostgreSQL    │    │   Meilisearch   │    │  Cloudflare R2  │
│   (Database)    │    │ (Search Engine) │    │ (File Storage)  │
│                 │    │  Port: 7700     │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Components

- **Main Backend** (Rust): Core API, authentication, business logic
- **Tasks API** (Python): Background task management and processing
- **Celery Workers**: Async image processing, search indexing, cleanup
- **Markdown Service** (Bun): High-performance markdown rendering
- **PostgreSQL**: Primary data storage
- **Redis**: Caching and message broker
- **Meilisearch**: Full-text search engine
- **Cloudflare R2**: Object storage for images

## 📁 Project Structure

### Main Backend (Rust)

```
src/
├── api/v0/routes/           # API route definitions
│   ├── auth/               # Authentication endpoints
│   ├── user/               # User management endpoints  
│   ├── post/               # Post CRUD and search
│   ├── follow/             # Social follow features
│   ├── like/               # Post like system
│   └── hashtag/            # Trending hashtags
├── service/                # Business logic layer
│   ├── auth/               # Auth services (JWT, OAuth)
│   ├── user/               # User management services
│   ├── post/               # Post processing services
│   ├── oauth/              # OAuth provider integrations
│   └── error/              # Structured error handling
├── repository/             # Data access layer
│   ├── auth/               # Auth data operations
│   ├── user/               # User data operations
│   ├── post/               # Post data operations
│   └── oauth/              # OAuth data operations
├── entity/                 # Database entity definitions (SeaORM)
├── dto/                    # Data Transfer Objects
│   ├── request/            # Request DTOs with validation
│   ├── response/           # Response DTOs
│   └── internal/           # Internal DTOs
├── middleware/             # Request/response middleware
├── connection/             # External service connections
├── config/                 # Configuration management
└── utils/                  # Utility functions
```

### Background Tasks (Python)

```
tasks/
├── app/
│   ├── api/routes/         # Task API endpoints
│   │   ├── user/           # User-related tasks
│   │   ├── search/         # Search indexing tasks
│   │   ├── email/          # Email sending tasks
│   │   └── count/          # Count sync tasks
│   ├── tasks/              # Celery task definitions
│   ├── services/           # Business logic services
│   ├── models/             # SQLAlchemy models
│   └── core/               # Core configuration
├── start_worker.py         # Celery worker starter
├── start_beat.py           # Celery scheduler starter
└── monitor_celery.py       # Flower monitoring
```

### Database Migrations

```
migration/
├── src/
│   ├── m*_users.rs         # User table migration
│   ├── m*_posts.rs         # Posts table migration
│   ├── m*_follows.rs       # Follows relationship
│   ├── m*_likes.rs         # Likes system
│   └── m*_hashtags.rs      # Hashtag system
└── main.rs                 # Migration runner
```

### Markdown Service (Bun)

```
markdown-service/
├── src/
│   ├── index.ts            # Main server file
│   └── markdown.ts         # Markdown processing
├── package.json            # Bun dependencies
└── Dockerfile              # Container config
```

### Key Design Patterns

**Layered Architecture:**
- **API Layer**: Route handlers, request validation
- **Service Layer**: Business logic, orchestration  
- **Repository Layer**: Data access, database queries
- **Entity Layer**: Database models and relationships

**Error Handling:**
- Centralized error management with structured codes
- Protocol-based error responses
- HTTP status code mapping

**Authentication Flow:**
- JWT access/refresh token pattern
- OAuth integration with account linking
- Middleware-based route protection

**Background Processing:**
- Async image processing and optimization
- Search index management
- Periodic cleanup tasks
- Task result tracking

## 🛠 Tech Stack

### Backend Services
- **Rust** - Main API server with Axum web framework
- **Python** - Background tasks with FastAPI and Celery
- **Bun/Elysia** - High-performance markdown rendering service

### Databases & Storage
- **PostgreSQL** - Primary database with SeaORM
- **Redis** - Caching and Celery message broker
- **Meilisearch** - Full-text search engine
- **Cloudflare R2** - S3-compatible object storage

### Key Libraries
- **Axum** - Modern async web framework for Rust
- **SeaORM** - Async ORM with compile-time checked queries
- **FastAPI** - High-performance Python web framework
- **Celery** - Distributed task queue
- **JWT** - Secure authentication with refresh tokens

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.86.0+
- **Python** 3.10+
- **Bun** (for markdown service)
- **Docker & Docker Compose** (recommended)
- **PostgreSQL** 14+
- **Redis** 6+

### Using Docker Compose (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/mofumofu-backend.git
   cd mofumofu-backend
   ```

2. **Set up environment**
   ```bash
   cp docker.env.example docker.env
   # Edit docker.env with your configuration
   ```

3. **Start all services**
   ```bash
   docker-compose up --build
   ```

4. **Access the services**
   - Main API: http://localhost:8000
   - API Documentation: http://localhost:8000/docs
   - Tasks API: http://localhost:7000
   - Flower (Task Monitor): http://localhost:5555
   - Meilisearch: http://localhost:7700

### Manual Setup

1. **Database Setup**
   ```bash
   # Run migrations
   cd migration
   cargo run
   ```

2. **Start Main Backend**
   ```bash
   cargo run
   ```

3. **Start Task Services**
   ```bash
   cd tasks
   uv sync
   uv run fastapi dev app/main.py
   
   # In separate terminals:
   python start_worker.py
   python start_beat.py
   ```

4. **Start Markdown Service**
   ```bash
   cd markdown-service
   bun install
   bun run dev
   ```

## 📚 API Documentation

### Endpoints Overview

The API is versioned under `/v0/`:

- **Authentication**: `/v0/auth/*`
  - OAuth sign-in (Google, GitHub)
  - Email/password authentication
  - Token management (access/refresh)
  - Password reset and email verification

- **User Management**: `/v0/user/*`
  - Profile management
  - Avatar and banner upload
  - Handle availability check

- **Content Management**: `/v0/post/*`
  - CRUD operations for posts
  - Image and thumbnail upload
  - Full-text search
  - View count tracking

- **Social Features**: `/v0/follow/*` & `/v0/like/*`
  - Follow/unfollow users
  - Like/unlike posts
  - Follower/following lists

- **Discovery**: `/v0/hashtag/*`
  - Trending hashtags
  - Content categorization

### Error Handling

The API uses structured error codes for better client integration:

```json
{
  "status": 400,
  "code": "password:incorrect",
  "details": "Current password is incorrect"
}
```

**Error Categories:**
- `user:*` - User-related errors (not_found, unauthorized, etc.)
- `oauth:*` - OAuth errors (account_already_linked, connection_not_found, etc.)  
- `password:*` - Password errors (incorrect, required_for_update, etc.)
- `token:*` - Token errors (expired_verification, invalid_reset, etc.)
- `file:*` - File errors (not_found, read_error, upload_error)
- `like:*` - Like errors (already_exists, not_found)
- `system:*` - System errors (database_error, internal_error)

### Interactive Documentation

- **Swagger UI**: http://localhost:8000/docs
- **ReDoc**: http://localhost:8000/redoc  
- **Tasks API Docs**: http://localhost:7000/docs

## 💻 Development

### Development Commands

**Main Backend (Rust):**
```bash
cargo check          # Quick type checking (recommended for development)
cargo build          # Build application
cargo run            # Start development server
cargo test           # Run tests
cargo clippy         # Run linter
cargo fmt            # Format code
```

**Background Tasks (Python):**
```bash
cd tasks
uv sync                              # Install dependencies
uv run fastapi dev app/main.py       # Start Tasks API (dev mode)
python start_worker.py               # Start Celery worker
python start_beat.py                 # Start Celery scheduler
python monitor_celery.py             # Start Flower monitoring
uv run ruff check .                  # Lint code
uv run ruff format .                 # Format code
```

**Markdown Service (TypeScript/Bun):**
```bash
cd markdown-service
bun install          # Install dependencies
bun run dev          # Start development server
```

### Running Tests

```bash
# Rust tests
cargo test

# Python tests (if available)  
cd tasks && uv run pytest

# Integration tests
cargo test --test integration_tests
```

### Database Migrations

```bash
# Run migrations
cd migration && cargo run

# Refresh (rollback and reapply all)
cd migration && cargo run -- refresh

# Create new migration (using sea-orm-cli)
cd migration
sea-orm-cli migrate generate <migration_name>
```

### Monitoring & Debugging

```bash
# View logs
docker-compose logs -f mofumofu-backend
docker-compose logs -f celery-worker

# Service health checks
curl http://localhost:8000/health     # Main API
curl http://localhost:7000/health     # Tasks API  
curl http://localhost:6700/health     # Markdown service

# Monitor background tasks
# Visit http://localhost:5555 (Flower UI)
```

## 📸 Image Upload Specifications

### Supported Formats
- **JPEG/JPG** - Automatically compressed and converted to WebP
- **PNG** - Automatically compressed and converted to WebP  
- **GIF** - Preserved as original format (no compression)
- **WebP** - Processed and optimized

### Image Dimensions & Compression
All images are automatically resized only if they exceed maximum dimensions, maintaining aspect ratio:

- **Avatar Images**: 512 × 512 pixels maximum
- **Banner Images**: 1600 × 400 pixels maximum  
- **Post Thumbnails**: 800 × 450 pixels maximum
- **Post Images**: 2000 × 2000 pixels maximum

### File Size Limits
- **Avatar**: 4MB maximum
- **Banner**: 8MB maximum
- **Thumbnails**: 4MB maximum  
- **Post Images**: 8MB maximum

### Automatic Optimization
- Non-GIF images are converted to WebP format for better compression
- Images larger than maximum dimensions are resized using high-quality Lanczos3 algorithm
- Quality setting: 90 for optimal balance between file size and image quality

## 🚀 Deployment

### Docker Deployment

```bash
# Build the image
docker build -t mofumofu-backend .

# Run with environment file
docker run -p 8000:8000 --env-file docker.env mofumofu-backend

# Or use docker-compose for full stack
docker-compose -f deploy_docker-compose.yml up
```

### Manual Deployment

```bash
# Build for production
cargo build --release

# Run the binary
./target/release/mofumofu-backend
```

## ⚙️ Environment Configuration

### Required Environment Variables

Create a `.env` file (copy from `.env.example`):

```env
# Application Environment
ENVIRONMENT=dev

# Database Configuration
POSTGRES_USER=postgres
POSTGRES_PASSWORD=your_password
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_NAME=mofumofu
POSTGRES_MAX_CONNECTION=100
POSTGRES_MIN_CONNECTION=10

# JWT Authentication  
JWT_SECRET=your-secret-key-here  # Generate with: openssl rand -base64 32
AUTH_ACCESS_TOKEN_EXPIRE_TIME=30  # minutes
AUTH_REFRESH_TOKEN_EXPIRE_TIME=14 # days

# OAuth Integration (Optional)
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:5173/account/oauth/callback/google
GOOGLE_LINK_REDIRECT_URI=http://localhost:5173/account/oauth/link/google

GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret
GITHUB_REDIRECT_URI=http://localhost:5173/account/oauth/callback/github

# File Storage (Cloudflare R2)
R2_PUBLIC_DOMAIN=your-r2-public-domain
R2_ACCOUNT_ID=your-r2-account-id
R2_BUCKET_NAME=your-bucket-name
R2_ACCESS_KEY_ID=your-access-key
R2_SECRET_ACCESS_KEY=your-secret-key

# Redis Configuration
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
REDIS_TTL=3600

# Celery (Background Tasks)
CELERY_BROKER_URL=redis://localhost:6379/0
CELERY_RESULT_BACKEND=redis://localhost:6379/0

# Meilisearch (Full-text Search)
MEILISEARCH_HOST=http://localhost:7700
MEILISEARCH_API_KEY=your-meilisearch-key

# CORS Configuration
CORS_ALLOWED_ORIGINS=http://localhost:5173
CORS_ALLOWED_HEADERS=Content-Type
CORS_MAX_AGE=86400

# Server Configuration
HOST=127.0.0.1
PORT=8000
```

### JWT Secret Generation

```bash
openssl rand -base64 32
```
Use the output as your `JWT_SECRET` in your environment variables.

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. **Fork** the repository
2. **Create** your feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** your changes and test them
4. **Commit** your changes (`git commit -m 'feat: add amazing feature'`)
5. **Push** to the branch (`git push origin feature/amazing-feature`)
6. **Open** a Pull Request

### Development Guidelines

**Code Quality:**
- Follow Rust naming conventions and idioms
- Use `cargo clippy` and `cargo fmt` for Rust code
- Use `ruff` for Python code formatting and linting
- Write tests for new features (`cargo test`, `pytest`)
- Update API documentation for endpoint changes

**Commit Messages:**
- Use [Conventional Commits](https://www.conventionalcommits.org/) format
- Examples: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`

**Pull Request Requirements:**
- Ensure all tests pass (`cargo test`)
- Update CLAUDE.md if adding new architecture components
- Add API documentation for new endpoints
- Include screenshots for UI-related changes

### Code Review Process

1. All submissions require review from maintainers
2. We may suggest changes, improvements, or alternatives
3. Once approved, maintainers will merge the PR
4. Please be patient during the review process

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust, Python, and Bun
- Inspired by modern social media platforms
- Special thanks to the open-source community

---

**Mofumofu Backend** - Making social blogging soft and simple for everyone! 🐧