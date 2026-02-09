from fastapi import APIRouter
from celery.result import AsyncResult
from app.core.celery_app import celery_app
from app.tasks.token_tasks import cleanup_expired_refresh_tokens, cleanup_old_system_events
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/token-cleanup", tags=["Token Cleanup"])


@router.post("/cleanup")
async def cleanup_tokens():
    """
    만료된 리프레시 토큰을 수동으로 정리합니다.

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = cleanup_expired_refresh_tokens.delay()

        return {
            "message": "토큰 정리 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"토큰 정리 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.get("/cleanup/status/{task_id}")
async def get_cleanup_status(task_id: str):
    """
    토큰 정리 작업의 상태를 확인합니다.

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
                "message": "작업이 대기 중입니다",
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


@router.get("/health")
async def token_cleanup_health():
    """
    토큰 정리 서비스 상태를 확인합니다.

    Returns:
        dict: 서비스 상태
    """
    return {
        "service": "Token Cleanup Service",
        "status": "healthy",
        "message": "토큰 정리 서비스가 정상 작동 중입니다",
    }


@router.post("/cleanup-events")
async def cleanup_events():
    """
    오래된 시스템 이벤트를 수동으로 정리합니다.

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = cleanup_old_system_events.delay()

        return {
            "message": "시스템 이벤트 정리 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"시스템 이벤트 정리 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}
