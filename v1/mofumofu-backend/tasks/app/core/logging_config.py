import logging
import sys
from typing import Dict, Any
from datetime import datetime


class TaskFormatter(logging.Formatter):
    """태스크 전용 로깅 포매터"""

    def format(self, record):
        # 시간 포맷 설정
        record.created_time = datetime.fromtimestamp(record.created).strftime(
            "%Y-%m-%d %H:%M:%S"
        )

        # 태스크 관련 정보 추가
        if hasattr(record, "task_id"):
            record.task_info = f"[Task:{record.task_id}]"
        else:
            record.task_info = ""

        if hasattr(record, "user_id"):
            record.user_info = f"[User:{record.user_id}]"
        else:
            record.user_info = ""

        return super().format(record)


def setup_logging() -> None:
    """로깅 설정 초기화"""

    # 기본 로거 설정
    root_logger = logging.getLogger()
    root_logger.setLevel(logging.INFO)

    # 기존 핸들러 제거
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)

    # 콘솔 핸들러 생성
    console_handler = logging.StreamHandler(sys.stdout)
    console_handler.setLevel(logging.INFO)

    # 포매터 설정
    formatter = TaskFormatter(
        "%(created_time)s - %(name)s - %(levelname)s %(task_info)s %(user_info)s - %(message)s"
    )
    console_handler.setFormatter(formatter)

    # 핸들러 추가
    root_logger.addHandler(console_handler)

    # 외부 라이브러리 로깅 레벨 조정
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("celery").setLevel(logging.INFO)
    logging.getLogger("meilisearch").setLevel(logging.WARNING)
    logging.getLogger("sqlalchemy").setLevel(logging.WARNING)
    logging.getLogger("boto3").setLevel(logging.WARNING)
    logging.getLogger("botocore").setLevel(logging.WARNING)


def get_task_logger(
    name: str, task_id: str = None, user_id: str = None
) -> logging.Logger:
    """태스크별 전용 로거 생성"""
    logger = logging.getLogger(name)

    # 기존 어댑터가 있으면 제거
    if hasattr(logger, "_task_adapter"):
        logger = logger._task_adapter

    # 태스크 정보가 있으면 어댑터 생성
    if task_id or user_id:
        adapter = logging.LoggerAdapter(
            logger, {"task_id": task_id, "user_id": user_id}
        )
        logger._task_adapter = adapter
        return adapter

    return logger


class TaskMetrics:
    """태스크 메트릭 수집"""

    _metrics: Dict[str, Any] = {}

    @classmethod
    def increment_counter(cls, metric_name: str, tags: Dict[str, str] = None):
        """카운터 메트릭 증가"""
        key = f"{metric_name}_{tags}" if tags else metric_name
        cls._metrics[key] = cls._metrics.get(key, 0) + 1

    @classmethod
    def record_duration(
        cls, metric_name: str, duration: float, tags: Dict[str, str] = None
    ):
        """실행 시간 메트릭 기록"""
        key = f"{metric_name}_duration_{tags}" if tags else f"{metric_name}_duration"
        if key not in cls._metrics:
            cls._metrics[key] = []
        cls._metrics[key].append(duration)

    @classmethod
    def get_metrics(cls) -> Dict[str, Any]:
        """현재 메트릭 반환"""
        return cls._metrics.copy()

    @classmethod
    def reset_metrics(cls):
        """메트릭 초기화"""
        cls._metrics.clear()


def log_task_start(logger: logging.Logger, task_name: str, **kwargs):
    """태스크 시작 로깅"""
    logger.info(f"태스크 시작: {task_name}", extra=kwargs)
    TaskMetrics.increment_counter("task_started", {"task": task_name})


def log_task_success(
    logger: logging.Logger, task_name: str, duration: float = None, **kwargs
):
    """태스크 성공 로깅"""
    msg = f"태스크 완료: {task_name}"
    if duration:
        msg += f" (소요시간: {duration:.2f}초)"
    logger.info(msg, extra=kwargs)
    TaskMetrics.increment_counter(
        "task_completed", {"task": task_name, "status": "success"}
    )
    if duration:
        TaskMetrics.record_duration("task_duration", duration, {"task": task_name})


def log_task_error(
    logger: logging.Logger, task_name: str, error: str, duration: float = None, **kwargs
):
    """태스크 실패 로깅"""
    msg = f"태스크 실패: {task_name} - {error}"
    if duration:
        msg += f" (소요시간: {duration:.2f}초)"
    logger.error(msg, extra=kwargs)
    TaskMetrics.increment_counter(
        "task_completed", {"task": task_name, "status": "failed"}
    )
    if duration:
        TaskMetrics.record_duration("task_duration", duration, {"task": task_name})
