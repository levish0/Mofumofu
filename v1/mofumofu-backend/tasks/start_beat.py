#!/usr/bin/env python3
"""
Celery Beat 스케줄러 시작 스크립트

주기적 작업(정리 작업 등)을 스케줄링하는 Celery Beat을 시작합니다.
이 스크립트는 백그라운드 작업의 스케줄링을 담당합니다.

사용법:
    python start_beat.py

주기적 작업:
    - cleanup_expired_refresh_tokens: 1시간마다 만료된 리프레시 토큰 정리
    - sync_all_counts: 24시간마다 like/follow 개수 동기화
"""

import subprocess
import sys
import os


def start_celery_beat():
    """Celery Beat 스케줄러를 시작합니다."""

    # 현재 디렉토리를 tasks로 설정
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    # Celery Beat 실행 명령어
    cmd = [
        "uv",
        "run",
        "celery",
        "-A",
        "app.core.celery_app",
        "beat",
        "--loglevel=info",
    ]

    print("=== Celery Beat 스케줄러 시작 ===")
    print("주기적 작업 스케줄:")
    print("- 리프레시 토큰 정리: 1시간마다")
    print("- 포스트 재색인: 24시간마다")
    print("- Meilisearch 헬스체크: 30분마다")
    print("- 시스템 이벤트 정리: 24시간마다")
    print("- Like/Follow 개수 동기화: 24시간마다")
    print("스케줄러를 중지하려면 Ctrl+C를 누르세요.")
    print("=" * 50)
    print(f"Command: {' '.join(cmd)}")

    try:
        subprocess.run(cmd, check=True)
    except KeyboardInterrupt:
        print("\n스케줄러가 중지되었습니다.")
    except subprocess.CalledProcessError as e:
        print(f"스케줄러 시작 중 오류 발생: {e}")
        sys.exit(1)


if __name__ == "__main__":
    start_celery_beat()
