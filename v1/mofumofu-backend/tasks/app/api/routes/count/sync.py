"""
Count synchronization API endpoints
"""

from fastapi import APIRouter
from app.tasks.count_tasks import sync_post_like_counts, sync_user_follow_counts, sync_all_counts
import logging

logger = logging.getLogger(__name__)

router = APIRouter(tags=["count"])


@router.post("/sync/likes")
async def sync_likes():
    """
    포스트 좋아요 개수 동기화를 수동으로 실행
    """
    try:
        # 비동기로 작업 큐에 추가
        task = sync_post_like_counts.delay()
        
        logger.info(f"Post like count sync task queued: {task.id}")
        
        return {
            "status": "success",
            "task_id": task.id,
            "message": "Post like count synchronization task has been queued"
        }
    except Exception as e:
        logger.error(f"Failed to queue like count sync task: {e}")
        return {
            "status": "error",
            "message": f"Failed to queue task: {str(e)}"
        }


@router.post("/sync/follows")
async def sync_follows():
    """
    유저 팔로우 개수 동기화를 수동으로 실행
    """
    try:
        # 비동기로 작업 큐에 추가
        task = sync_user_follow_counts.delay()
        
        logger.info(f"User follow count sync task queued: {task.id}")
        
        return {
            "status": "success",
            "task_id": task.id,
            "message": "User follow count synchronization task has been queued"
        }
    except Exception as e:
        logger.error(f"Failed to queue follow count sync task: {e}")
        return {
            "status": "error",
            "message": f"Failed to queue task: {str(e)}"
        }


@router.post("/sync/all")
async def sync_all():
    """
    모든 카운트 동기화를 수동으로 실행
    """
    try:
        # 비동기로 작업 큐에 추가
        task = sync_all_counts.delay()
        
        logger.info(f"Full count sync task queued: {task.id}")
        
        return {
            "status": "success",
            "task_id": task.id,
            "message": "Full count synchronization task has been queued"
        }
    except Exception as e:
        logger.error(f"Failed to queue full count sync task: {e}")
        return {
            "status": "error",
            "message": f"Failed to queue task: {str(e)}"
        }


@router.get("/sync/status/{task_id}")
async def get_sync_status(task_id: str):
    """
    동기화 작업의 상태를 확인
    """
    try:
        from celery.result import AsyncResult
        from app.core.celery_app import celery_app
        
        # 작업 결과 확인
        result = AsyncResult(task_id, app=celery_app)
        
        if result.state == 'PENDING':
            response = {
                "task_id": task_id,
                "status": "PENDING",
                "message": "Task is waiting to be processed"
            }
        elif result.state == 'SUCCESS':
            response = {
                "task_id": task_id,
                "status": "SUCCESS",
                "result": result.result,
                "message": "Task completed successfully"
            }
        elif result.state == 'FAILURE':
            response = {
                "task_id": task_id,
                "status": "FAILURE",
                "error": str(result.result),
                "message": "Task failed"
            }
        else:
            response = {
                "task_id": task_id,
                "status": result.state,
                "message": f"Task state: {result.state}"
            }
            
        logger.info(f"Task {task_id} status: {result.state}")
        return response
        
    except Exception as e:
        logger.error(f"Failed to get task status for {task_id}: {e}")
        return {
            "task_id": task_id,
            "status": "ERROR",
            "message": f"Failed to get task status: {str(e)}"
        }