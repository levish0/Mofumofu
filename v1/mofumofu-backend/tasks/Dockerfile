# Python 3.11 slim 이미지 사용
FROM python:3.11-slim

# 작업 디렉토리 설정
WORKDIR /app

# 시스템 의존성 설치 (PostgreSQL 클라이언트 라이브러리 포함)
RUN apt-get update && apt-get install -y \
    curl \
    && rm -rf /var/lib/apt/lists/*

# uv 설치 (Python 패키지 매니저)
RUN pip install uv

# lock 파일과 pyproject.toml을 먼저 복사하여 Docker 레이어 캐시를 활용합니다.
COPY pyproject.toml uv.lock ./

# lock 파일을 기반으로 정확한 의존성을 설치합니다.
RUN uv sync --frozen

# 애플리케이션 코드 복사
COPY app/ ./app/
COPY start_worker.py start_beat.py monitor_celery.py ./

# 환경 변수 설정
ENV PYTHONPATH=/app
ENV PYTHONUNBUFFERED=1

# 포트 노출
EXPOSE 7000

# 기본 명령어 (FastAPI 서버 시작)
CMD ["uv", "run", "uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "7000"]