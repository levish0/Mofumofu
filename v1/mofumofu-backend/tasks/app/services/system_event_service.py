from app.models import SystemEvent
from app.services.base_db_service import base_db_service
from datetime import datetime, timedelta
import logging

logger = logging.getLogger(__name__)


class SystemEventService:
    """시스템 이벤트 관련 데이터베이스 서비스 (ORM 기반)"""
    
    def __init__(self):
        self.db = base_db_service

    def cleanup_old_system_events(self, current_time: datetime) -> dict:
        """
        30일 이상 된 시스템 이벤트를 정리합니다.
        
        Args:
            current_time: 현재 시각 (UTC)
            
        Returns:
            dict: 정리 결과 (success, deleted_count, error)
        """
        with self.db.session_factory() as session:
            try:
                # 30일 전 날짜 계산
                cutoff_date = current_time - timedelta(days=30)
                
                # 30일 이상 된 시스템 이벤트들 조회
                old_events = (
                    session.query(SystemEvent)
                    .filter(SystemEvent.created_at < cutoff_date)
                    .all()
                )
                deleted_count = len(old_events)
                
                # 실제 삭제 수행
                for event in old_events:
                    session.delete(event)
                
                session.commit()
                
                logger.info(f"시스템 이벤트 정리 완료: {deleted_count}개 삭제")
                
                return {
                    "success": True,
                    "deleted_count": deleted_count,
                }
                
            except Exception as e:
                logger.error(f"시스템 이벤트 정리 실패: {str(e)}")
                session.rollback()
                return {
                    "success": False,
                    "deleted_count": 0,
                    "error": str(e),
                }


# 전역 시스템 이벤트 서비스 인스턴스
system_event_service = SystemEventService()