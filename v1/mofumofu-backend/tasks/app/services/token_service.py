from app.models import UserRefreshToken
from app.services.base_db_service import base_db_service
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class TokenService:
    """토큰 관련 데이터베이스 서비스 (ORM 기반)"""
    
    def __init__(self):
        self.db = base_db_service

    def cleanup_expired_refresh_tokens(self, current_time: datetime) -> dict:
        """
        만료되거나 폐기된 리프레시 토큰을 정리합니다.
        
        Args:
            current_time: 현재 시각 (UTC)
            
        Returns:
            dict: 정리 결과 (success, expired_count, revoked_count, error)
        """
        with self.db.session_factory() as session:
            try:
                # 만료된 토큰들 조회 및 삭제
                expired_tokens = (
                    session.query(UserRefreshToken)
                    .filter(UserRefreshToken.expires_at < current_time)
                    .all()
                )
                expired_count = len(expired_tokens)
                
                # 폐기된 토큰들 조회 및 삭제
                revoked_tokens = (
                    session.query(UserRefreshToken)
                    .filter(UserRefreshToken.revoked_at.isnot(None))
                    .all()
                )
                revoked_count = len(revoked_tokens)
                
                # 실제 삭제 수행
                for token in expired_tokens:
                    session.delete(token)
                
                for token in revoked_tokens:
                    session.delete(token)
                
                session.commit()
                
                logger.info(
                    f"토큰 정리 완료: 만료된 토큰 {expired_count}개, 폐기된 토큰 {revoked_count}개 삭제"
                )
                
                return {
                    "success": True,
                    "expired_count": expired_count,
                    "revoked_count": revoked_count,
                }
                
            except Exception as e:
                logger.error(f"토큰 정리 실패: {str(e)}")
                session.rollback()
                return {
                    "success": False,
                    "expired_count": 0,
                    "revoked_count": 0,
                    "error": str(e),
                }


# 전역 토큰 서비스 인스턴스
token_service = TokenService()