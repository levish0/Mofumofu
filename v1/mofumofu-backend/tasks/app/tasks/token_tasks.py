from app.core.celery_app import celery_app
from app.services.db_service import db_service
import logging
from datetime import datetime, timezone

logger = logging.getLogger(__name__)


@celery_app.task(name="cleanup_expired_refresh_tokens")
def cleanup_expired_refresh_tokens():
    """
    만료되거나 폐기된 리프레시 토큰을 데이터베이스에서 정리하는 태스크

    - expires_at이 현재 시각보다 이전인 토큰들 삭제
    - revoked_at이 설정된 토큰들 삭제 (폐기된 토큰)

    Returns:
        dict: 정리 결과 및 삭제된 토큰 수
    """
    try:
        logger.info("만료된 리프레시 토큰 정리 작업 시작")

        current_time = datetime.now(timezone.utc)
        result = db_service.cleanup_expired_refresh_tokens(current_time)

        if result["success"]:
            logger.info(
                f"리프레시 토큰 정리 완료: 만료된 토큰 {result['expired_count']}개, 폐기된 토큰 {result['revoked_count']}개 삭제"
            )
            return {
                "status": "SUCCESS",
                "expired_tokens_deleted": result["expired_count"],
                "revoked_tokens_deleted": result["revoked_count"],
                "total_deleted": result["expired_count"] + result["revoked_count"],
                "message": f"총 {result['expired_count'] + result['revoked_count']}개의 토큰이 정리되었습니다",
            }
        else:
            logger.error(
                f"리프레시 토큰 정리 실패: {result.get('error', 'Unknown error')}"
            )
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}

    except Exception as exc:
        logger.error(f"리프레시 토큰 정리 작업 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}


@celery_app.task(name="cleanup_old_system_events")
def cleanup_old_system_events():
    """
    1달(30일) 이상 된 system_events 레코드를 정리하는 태스크
    
    - created_at이 30일 이전인 레코드들 삭제
    
    Returns:
        dict: 정리 결과 및 삭제된 레코드 수
    """
    try:
        logger.info("오래된 시스템 이벤트 정리 작업 시작")
        
        current_time = datetime.now(timezone.utc)
        result = db_service.cleanup_old_system_events(current_time)
        
        if result["success"]:
            logger.info(f"시스템 이벤트 정리 완료: {result['deleted_count']}개 삭제")
            return {
                "status": "SUCCESS",
                "deleted_count": result["deleted_count"],
                "message": f"30일 이상 된 시스템 이벤트 {result['deleted_count']}개가 정리되었습니다",
            }
        else:
            logger.error(f"시스템 이벤트 정리 실패: {result.get('error', 'Unknown error')}")
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}
            
    except Exception as exc:
        logger.error(f"시스템 이벤트 정리 작업 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}
