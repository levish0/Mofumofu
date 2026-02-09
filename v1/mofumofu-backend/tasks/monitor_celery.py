"""
Celery 모니터링 스크립트
"""

import subprocess
import sys
import os


def monitor_celery():
    """Celery 상태를 모니터링합니다."""

    # 현재 디렉토리를 tasks로 설정
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    # Celery flower (웹 모니터링) 실행 명령어
    cmd = [
        "uv",
        "run",
        "celery",
        "-A",
        "app.core.celery_app",
        "flower",
        "--port=5555",
        "--broker=redis://localhost:6379/0",
    ]

    print("Starting Celery Flower monitoring...")
    print("Open http://localhost:5555 in your browser to view the monitoring dashboard")
    print(f"Command: {' '.join(cmd)}")

    try:
        subprocess.run(cmd, check=True)
    except KeyboardInterrupt:
        print("\nCelery Flower stopped by user")
    except subprocess.CalledProcessError as e:
        print(f"Error starting Celery Flower: {e}")
        print("Note: You need to install flower first: uv add flower")
        sys.exit(1)


if __name__ == "__main__":
    monitor_celery()
