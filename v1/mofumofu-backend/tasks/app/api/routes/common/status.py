from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from celery.result import AsyncResult
from app.core.celery_app import celery_app
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/tasks", tags=["common"])


class TaskStatusResponse(BaseModel):
    task_id: str
    status: str
    result: dict = None
    error: str = None


@router.get("/status/{task_id}", response_model=TaskStatusResponse)
async def get_task_status(task_id: str):
    """
    태스크 실행 상태와 결과를 조회
    """
    try:
        # Celery 태스크 결과 조회
        result = AsyncResult(task_id, app=celery_app)

        response = TaskStatusResponse(task_id=task_id, status=result.status)

        if result.status == "PENDING":
            response.result = {"message": "태스크가 대기 중입니다"}
        elif result.status == "PROGRESS":
            response.result = result.result
        elif result.status == "SUCCESS":
            response.result = result.result
        elif result.status == "FAILURE":
            response.error = str(result.result)

        return response

    except Exception as e:
        logger.error(f"태스크 상태 조회 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 상태 조회 실패: {str(e)}")


@router.get("/health")
async def health_check():
    """
    태스크 서비스 헬스 체크
    """
    return {"status": "healthy", "service": "tasks"}
