from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import Optional, Dict, Any
from app.tasks.markdown_tasks import render_markdown_task
import logging

logger = logging.getLogger(__name__)
router = APIRouter()


class MarkdownRenderRequest(BaseModel):
    post_id: str
    content: str


class TaskStatusResponse(BaseModel):
    task_id: str
    status: str
    result: Optional[Dict[str, Any]] = None
    meta: Optional[Dict[str, Any]] = None


@router.post("/render", summary="마크다운 렌더링", tags=["markdown"])
async def render_markdown(request: MarkdownRenderRequest) -> Dict[str, Any]:
    """
    마크다운 렌더링을 비동기로 요청하고 태스크 ID를 반환합니다.
    """
    try:
        task = render_markdown_task.delay(request.post_id, request.content)
        
        return {
            "success": True,
            "task_id": task.id,
            "message": "마크다운 렌더링 태스크가 시작되었습니다"
        }
        
    except Exception as e:
        logger.error(f"마크다운 렌더링 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))




@router.get("/task-status/{task_id}", summary="태스크 상태 조회", tags=["markdown"])
async def get_task_status(task_id: str) -> TaskStatusResponse:
    """
    마크다운 관련 태스크의 상태를 조회합니다.
    """
    try:
        from app.core.celery_app import celery_app
        
        task = celery_app.AsyncResult(task_id)
        
        return TaskStatusResponse(
            task_id=task_id,
            status=task.status,
            result=task.result if task.ready() else None,
            meta=task.info if not task.ready() else None
        )
        
    except Exception as e:
        logger.error(f"태스크 상태 조회 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/health", summary="마크다운 서비스 헬스 체크", tags=["markdown"])
async def health_check() -> Dict[str, Any]:
    """
    마크다운 서비스의 헬스 체크
    """
    try:
        return {
            "status": "healthy",
            "message": "마크다운 렌더링 서비스가 정상적으로 작동 중입니다"
        }
        
    except Exception as e:
        logger.error(f"헬스 체크 실패: {str(e)}")
        raise HTTPException(status_code=503, detail=f"서비스 비정상: {str(e)}")