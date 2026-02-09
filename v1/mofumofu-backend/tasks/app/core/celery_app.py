from celery import Celery
from app.core.config import settings

celery_app = Celery(
    "tasks",
    broker=settings.CELERY_BROKER_URL,
    backend=settings.CELERY_RESULT_BACKEND,
    include=[
        "app.tasks.token_tasks",
        "app.tasks.post_tasks",
        "app.tasks.search_tasks",
        "app.tasks.markdown_tasks",
        "app.tasks.count_tasks",
    ],
)

celery_app.conf.update(
    task_serializer="json",
    accept_content=["json"],
    result_serializer="json",
    timezone="UTC",
    enable_utc=True,
    task_track_started=True,
    task_time_limit=300,  # 5분 타임아웃
    worker_prefetch_multiplier=1,
    task_acks_late=True,
    # 결과 만료 설정 (Redis 메모리 관리)
    result_expires=3600,  # 1시간 후 결과 삭제
    # 작업 결과 압축
    result_compression="gzip",
    # 주기적 작업 스케줄
    beat_schedule={
        "cleanup-expired-refresh-tokens": {
            "task": "cleanup_expired_refresh_tokens",
            "schedule": 3600.0,  # 1시간마다 실행 (3600초)
        },
        "reindex-all-posts-daily": {
            "task": "reindex_all_posts",
            "schedule": 86400.0,  # 24시간마다 실행 (86400초 = 1일)
        },
        "check-meilisearch-health": {
            "task": "check_meilisearch_health",
            "schedule": 1800.0,  # 30분마다 헬스체크 (1800초)
        },
        "cleanup-old-system-events": {
            "task": "cleanup_old_system_events",
            "schedule": 86400.0,  # 24시간마다 실행 (86400초 = 1일)
        },
        "sync-all-counts-daily": {
            "task": "sync_all_counts",
            "schedule": 86400.0,  # 24시간마다 실행 (86400초 = 1일) - like/follow 개수 동기화
        },
    },
)
