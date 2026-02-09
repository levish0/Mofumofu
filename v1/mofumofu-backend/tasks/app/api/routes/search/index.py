from fastapi import APIRouter
from celery.result import AsyncResult
from app.core.celery_app import celery_app
from app.tasks.search_tasks import (
    index_single_post_task,
    update_single_post_task,
    delete_single_post_task,
)
from pydantic import BaseModel
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/search", tags=["Search Index"])


class PostIndexRequest(BaseModel):
    post_id: str


@router.post("/index")
async def index_post(request: PostIndexRequest):
    """
    단일 포스트를 색인합니다.

    Args:
        request: 포스트 ID가 포함된 요청 객체

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = index_single_post_task.delay(request.post_id)

        return {
            "message": "포스트 색인 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"포스트 색인 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.put("/update")
async def update_post(request: PostIndexRequest):
    """
    단일 포스트 색인을 업데이트합니다.

    Args:
        request: 포스트 ID가 포함된 요청 객체

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = update_single_post_task.delay(request.post_id)

        return {
            "message": "포스트 업데이트 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"포스트 업데이트 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.delete("/delete")
async def delete_post(request: PostIndexRequest):
    """
    단일 포스트를 색인에서 삭제합니다.

    Args:
        request: 포스트 ID가 포함된 요청 객체

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = delete_single_post_task.delay(request.post_id)

        return {
            "message": "포스트 삭제 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"포스트 삭제 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.get("/index/status/{task_id}")
async def get_index_status(task_id: str):
    """
    색인 작업의 상태를 확인합니다.

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
                "message": "색인 작업이 대기 중입니다",
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
