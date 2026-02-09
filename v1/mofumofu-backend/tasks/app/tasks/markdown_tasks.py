import asyncio
from celery import current_task
from app.core.celery_app import celery_app
from app.core.config import settings
from app.services.db_service import db_service
import logging
import httpx
from typing import Dict, Any
from app.utils import create_failure_response

logger = logging.getLogger(__name__)

def _update_post_render(post_id: str, html_content: str, toc_items: list) -> None:
    """데이터베이스의 포스트 render와 toc 컬럼을 업데이트"""
    try:
        success = db_service.posts.update_post_render_and_toc(post_id, html_content, toc_items)
        if success:
            logger.info(f"포스트 render, toc 컬럼 업데이트 완료: post_id={post_id}")
        else:
            logger.error(f"포스트 render, toc 컬럼 업데이트 실패: post_id={post_id}")
            raise Exception("Database update failed")
    except Exception as e:
        logger.error(f"포스트 render, toc 컬럼 업데이트 실패: post_id={post_id}, error={str(e)}")
        raise


@celery_app.task(bind=True, name="render_markdown")
def render_markdown_task(self, post_id: str, content: str) -> Dict[str, Any]:
    """
    마크다운을 렌더링하고 데이터베이스에 저장
    
    Args:
        post_id: 포스트 ID
        content: 마크다운 콘텐츠
    
    Returns:
        dict: 렌더링 결과 또는 에러
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state="PROGRESS", 
            meta={"status": "마크다운 렌더링 중...", "content_length": len(content)}
        )
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_render_markdown(content))
        
        # 데이터베이스 업데이트
        _update_post_render(post_id, result["html_content"], result["toc_items"])
        
        logger.info(f"마크다운 렌더링 완료 및 DB 저장: post_id={post_id}")
        
        return {
            "status": "SUCCESS",
            "data": result,
            "post_id": post_id
        }
        
    except Exception as exc:
        logger.error(f"마크다운 렌더링 실패: {str(exc)}")
        current_task.update_state(state="FAILURE", meta={"error": str(exc)})
        return create_failure_response(str(exc))




async def _render_markdown(content: str) -> Dict[str, Any]:
    """마크다운 서비스를 통한 렌더링"""
    
    # 마크다운 서비스 URL
    markdown_service_url = f"http://{settings.MARKDOWN_SERVICE_HOST}:{settings.MARKDOWN_SERVICE_PORT}"
    
    request_data = {
        "markdown": content
    }
    
    async with httpx.AsyncClient(timeout=30.0) as client:
        try:
            response = await client.post(
                f"{markdown_service_url}/render",
                json=request_data
            )
            
            if response.status_code != 200:
                raise Exception(f"마크다운 서비스 요청 실패: {response.status_code} - {response.text}")
            
            result = response.json()
            
            if not result.get("success"):
                error_msg = result.get("error", "알 수 없는 오류")
                raise Exception(f"마크다운 처리 실패: {error_msg}")
            
            data = result.get("data")
            if not data:
                raise Exception("마크다운 서비스 응답에서 데이터 누락")
            
            # 표준화된 형태로 반환
            return {
                "html_content": data.get("htmlContent", ""),
                "toc_items": data.get("tocItems", [])
            }
            
        except httpx.TimeoutException:
            raise Exception("마크다운 서비스 타임아웃")
        except httpx.RequestError as e:
            raise Exception(f"마크다운 서비스 연결 실패: {str(e)}")


