## Project Dependency Installation and Management
### 1. Install Only Essential Dependencies

Use this if you only want to install the **essential dependencies** required to run the project. These are the packages defined in the `[project].dependencies` section of your `pyproject.toml` file. This excludes development and testing tools (like **Ruff** or code formatters).

```bash
uv pip install .
```

This command sets up the lightest possible environment to get your project running.

### 2. Install All Dependencies (Including Development Dependencies)

Use this when you want to install **all project dependencies**, including those needed for development. This covers both the essential dependencies from `[project].dependencies` and the development and testing tools (like **Ruff**) defined under `[tool.uv].dev-dependencies` in your `pyproject.toml` file.

```bash
uv sync
```

The `uv sync` command synchronizes and installs all dependencies specified in `pyproject.toml` according to the `uv.lock` file. This method is useful for setting up a complete development environment.

### Which command should you use?

* **To simply run the project or set up a deployment environment:** Use `uv pip install .`
* **To develop, test, or perform code checks on the project:** Use `uv sync`

---

## Starting the Complete System

To run the full background task system, you need to start **3 services** in this order:

### 1. Redis Server (required for Celery)
Make sure Redis is running on your system:
```bash
# On Windows (if using Windows Subsystem for Linux)
sudo service redis-server start

# On macOS (using Homebrew)
brew services start redis

# On Docker
docker run -d -p 6379:6379 redis:alpine
```

### 2. FastAPI Task Server
Start the FastAPI server that provides task management endpoints:
```bash
uvicorn app.main:app --host 0.0.0.0 --port 7000
```

### 3. Celery Worker
Start the Celery worker to process background tasks:
```bash
python start_worker.py
```

### Development Mode
For development, use FastAPI's auto-reload:
```bash
uv run fastapi dev app/main.py --port 7000
```

### Monitoring (Optional)
To monitor Celery tasks with Flower web interface:
```bash
python monitor_celery.py
# Then open http://localhost:5555 in your browser
```

---

## Docker Deployment

### Using Docker Compose (Recommended)

This project is designed to work with the main Rust backend. The complete deployment involves two docker-compose files:

1. **Tasks services** (this directory)
2. **Main Rust backend** (root directory)

#### Step 1: Configure Environment
The tasks system uses the main project's `docker.env` file:
```bash
# Make sure docker.env exists in the root directory
# ../docker.env should contain all required environment variables
```

#### Step 2: Start Tasks Services First
```bash
# In the tasks/ directory
docker-compose up -d
```

This starts:
- **redis-celery**: Message broker (port 6380) with persistent volume
- **tasks-api**: FastAPI server (port 7000)
- **celery-worker**: Background task processor (2 replicas)
- **celery-beat**: Periodic task scheduler
- **flower**: Celery monitoring web UI (port 5555)

#### Step 3: Start Main Backend
```bash
# In the root directory
cd ..
docker-compose up -d
```

The main Rust backend will connect to the tasks network automatically.

#### Monitoring and Logs
```bash
# View all task service logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f celery-worker
docker-compose logs -f tasks-api
docker-compose logs -f redis-celery

# Monitor Celery tasks via Flower
# Open http://localhost:5555 in your browser
```

#### Stopping Services
```bash
# Stop tasks services
docker-compose down

# Stop main backend (from root directory)
cd .. && docker-compose down
```

### Services Overview:
- **redis-celery**: Celery message broker with persistent storage
- **tasks-api**: FastAPI server providing task management API
- **celery-worker**: Background task processors (2 replicas for load balancing)
- **celery-beat**: Handles scheduled/periodic tasks
- **flower**: Web-based monitoring interface for Celery

### Environment Configuration:
All services use `../docker.env` file which should contain:
- PostgreSQL connection details
- Cloudflare R2 credentials
- Celery broker settings (overridden for internal network communication)
- Other application settings

### Manual Docker Build

If you prefer to build images manually:

```bash
# Build FastAPI server
docker build -t tasks-api -f Dockerfile .

# Build Celery worker
docker build -t celery-worker -f Dockerfile.worker .

# Run with external Redis and PostgreSQL
docker run -d --name tasks-api -p 7000:7000 \
  -e POSTGRES_HOST=your_db_host \
  -e CELERY_BROKER_URL=redis://your_redis_host:6379/0 \
  tasks-api
```

---

## Using Ruff

**Ruff** is an fast Python linter and code formatter. Once you've installed your development dependencies (using `uv sync`), you can use Ruff to check and format your code.

### Check for linting errors and formatting issues:

```bash
ruff check .
```

This command will analyze your entire project (current directory `.` ) for any linting errors or style violations according to your `pyproject.toml` configuration.

### Automatically fix fixable issues:

```bash
ruff check . --fix
```

This command will not only check for issues but also automatically fix any problems that Ruff knows how to correct, such as formatting inconsistencies or simple linting errors.

### Format your code:

```bash
ruff format .
```

Use this command to automatically reformat your code according to Ruff's default or configured style. This ensures consistent code style across your project.
