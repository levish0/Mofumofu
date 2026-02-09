from app.core.celery_app import celery_app
from app.services.db_service import db_service
import logging

logger = logging.getLogger(__name__)


@celery_app.task(name="sync_post_like_counts")
def sync_post_like_counts():
    """
    모든 포스트의 like_count를 실제 likes 테이블의 개수로 동기화하는 태스크
    
    Returns:
        dict: 동기화 결과 및 업데이트된 포스트 수
    """
    try:
        logger.info("포스트 좋아요 개수 동기화 작업 시작")
        
        result = db_service.sync_post_like_counts()
        
        if result["success"]:
            logger.info(f"포스트 좋아요 개수 동기화 완료: {result['updated_count']}개 포스트 업데이트")
            return {
                "status": "SUCCESS",
                "updated_count": result["updated_count"],
                "message": result["message"]
            }
        else:
            logger.error(f"포스트 좋아요 개수 동기화 실패: {result.get('error', 'Unknown error')}")
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}
            
    except Exception as exc:
        logger.error(f"포스트 좋아요 개수 동기화 작업 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}


@celery_app.task(name="sync_user_follow_counts")
def sync_user_follow_counts():
    """
    모든 유저의 follower_count와 following_count를 실제 follows 테이블의 개수로 동기화하는 태스크
    
    Returns:
        dict: 동기화 결과 및 업데이트된 유저 수
    """
    try:
        logger.info("유저 팔로우 개수 동기화 작업 시작")
        
        result = db_service.sync_user_follow_counts()
        
        if result["success"]:
            logger.info(
                f"유저 팔로우 개수 동기화 완료: follower {result['follower_updated_count']}개, "
                f"following {result['following_updated_count']}개 업데이트"
            )
            return {
                "status": "SUCCESS",
                "follower_updated_count": result["follower_updated_count"],
                "following_updated_count": result["following_updated_count"],
                "total_updated": result["total_updated"],
                "message": result["message"]
            }
        else:
            logger.error(f"유저 팔로우 개수 동기화 실패: {result.get('error', 'Unknown error')}")
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}
            
    except Exception as exc:
        logger.error(f"유저 팔로우 개수 동기화 작업 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}


@celery_app.task(name="sync_all_counts")
def sync_all_counts():
    """
    모든 카운트(like, follow) 동기화를 한번에 실행하는 태스크
    
    Returns:
        dict: 전체 동기화 결과
    """
    try:
        logger.info("전체 카운트 동기화 작업 시작")
        
        result = db_service.sync_all_counts()
        
        if result["success"]:
            logger.info("전체 카운트 동기화 완료")
            return {
                "status": "SUCCESS",
                "like_sync": result["like_sync"],
                "follow_sync": result["follow_sync"],
                "message": result["message"]
            }
        else:
            logger.error(f"전체 카운트 동기화 실패: {result.get('error', 'Unknown error')}")
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}
            
    except Exception as exc:
        logger.error(f"전체 카운트 동기화 작업 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}