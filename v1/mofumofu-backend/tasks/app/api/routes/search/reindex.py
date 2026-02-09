from fastapi import APIRouter
from celery.result import AsyncResult
from app.core.celery_app import celery_app
from app.tasks.search_tasks import (
    reindex_all_posts_task,
    check_meilisearch_health_task,
    get_search_index_stats_task,
)
from app.services.meilisearch_service import meilisearch_service
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/search-reindex", tags=["Search Reindex"])


@router.post("/reindex")
async def reindex_posts():
    """
    전체 포스트를 수동으로 재색인합니다.

    Celery 태스크를 사용하여 백그라운드에서 처리됩니다.

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = reindex_all_posts_task.delay()

        return {
            "message": "포스트 재색인 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"포스트 재색인 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.get("/reindex/status/{task_id}")
async def get_reindex_status(task_id: str):
    """
    포스트 재색인 작업의 상태를 확인합니다.

    Args:
        task_id: 작업 ID

    Returns:
        dict: 작업 상태와 결과
    """
    try:
        task_result = AsyncResult(task_id, app=celery_app)

        if task_result.state == "PENDING":
            response = {
                "task_id": task_id,
                "state": task_result.state,
                "message": "재색인 작업이 대기 중입니다",
            }
        elif task_result.state == "SUCCESS":
            response = {
                "task_id": task_id,
                "state": task_result.state,
                "result": task_result.result,
            }
        elif task_result.state == "FAILURE":
            response = {
                "task_id": task_id,
                "state": task_result.state,
                "error": str(task_result.info),
            }
        else:
            response = {
                "task_id": task_id,
                "state": task_result.state,
                "info": str(task_result.info),
            }

        return response

    except Exception as e:
        logger.error(f"작업 상태 조회 실패: {str(e)}")
        return {
            "error": f"작업 상태 조회 실패: {str(e)}",
            "task_id": task_id,
            "state": "ERROR",
        }


@router.get("/stats")
async def get_search_stats():
    """
    검색 색인 통계를 직접 조회합니다.

    Returns:
        dict: 색인 통계 정보
    """
    try:
        result = meilisearch_service.get_index_stats()
        return result
    except Exception as e:
        logger.error(f"검색 통계 조회 실패: {str(e)}")
        return {"error": f"검색 통계 조회 실패: {str(e)}", "status": "FAILED"}


@router.get("/stats/task")
async def get_search_stats_task():
    """
    검색 색인 통계를 Celery 태스크로 조회합니다.

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = get_search_index_stats_task.delay()

        return {
            "message": "검색 통계 조회 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"검색 통계 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.get("/health")
async def search_health():
    """
    검색 서비스 상태를 직접 확인합니다.

    Returns:
        dict: 서비스 상태
    """
    try:
        result = meilisearch_service.get_health()
        return result
    except Exception as e:
        logger.error(f"검색 서비스 상태 확인 실패: {str(e)}")
        return {"error": f"검색 서비스 상태 확인 실패: {str(e)}", "status": "FAILED"}


@router.get("/health/task")
async def search_health_task():
    """
    검색 서비스 상태를 Celery 태스크로 확인합니다.

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = check_meilisearch_health_task.delay()

        return {
            "message": "검색 서비스 헬스체크 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"헬스체크 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}
