#!/usr/bin/env python3
"""
Celery worker 시작 스크립트
"""

import subprocess
import sys
import os

from app.utils import build_email_templates


def start_celery_worker():
    """Celery worker를 시작합니다."""

    # 현재 디렉토리를 tasks로 설정
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    print("Building email templates...")
    try:
        build_email_templates()
        print("Email templates built successfully.")
    except Exception as e:
        print(f"Failed to build email templates: {e}")

    # Celery worker 실행 명령어
    cmd = [
        "uv",
        "run",
        "celery",
        "-A",
        "app.core.celery_app",
        "worker",
        "--loglevel=info",
        "--concurrency=2",  # 동시 처리 작업 수
        "--pool=solo"
        if sys.platform == "win32"
        else "--pool=prefork",  # Windows에서는 solo pool 사용
    ]

    print("Starting Celery worker...")
    print(f"Command: {' '.join(cmd)}")

    try:
        subprocess.run(cmd, check=True)
    except KeyboardInterrupt:
        print("\nCelery worker stopped by user")
    except subprocess.CalledProcessError as e:
        print(f"Error starting Celery worker: {e}")
        sys.exit(1)


if __name__ == "__main__":
    start_celery_worker()
