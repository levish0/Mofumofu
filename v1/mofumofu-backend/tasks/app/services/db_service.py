# 하위 호환성을 위한 기존 db_service 인터페이스 유지
from app.services.base_db_service import base_db_service
from app.services.post_service import post_service
from app.services.token_service import token_service
from app.services.system_event_service import system_event_service
from app.services.count_service import count_service
from typing import Optional, List, Dict, Any
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class DatabaseService:
    """
    기존 코드와의 호환성을 위한 통합 데이터베이스 서비스 클래스
    실제 구현은 개별 서비스 클래스들에 위임
    """

    def __init__(self):
        self.base = base_db_service
        self.posts = post_service
        self.tokens = token_service
        self.system_events = system_event_service
        self.counts = count_service

    # 기본 데이터베이스 메서드들
    def get_session(self):
        return self.base.get_session()

    def close(self):
        return self.base.close()

    # 포스트 관련 메서드들 (post_service로 위임)
    def get_post_by_id(self, post_id: str):
        return self.posts.get_post_by_id(post_id)

    def update_post_thumbnail(self, post_id: str, thumbnail_url: Optional[str]) -> bool:
        return self.posts.update_post_thumbnail(post_id, thumbnail_url)

    def get_all_posts_for_indexing(self) -> List[Dict[str, Any]]:
        return self.posts.get_all_posts_for_indexing()

    def get_posts_by_ids(self, post_ids: List[str]) -> List[Dict[str, Any]]:
        return self.posts.get_posts_by_ids(post_ids)

    # 토큰 관련 메서드들 (token_service로 위임)
    def cleanup_expired_refresh_tokens(self, current_time: datetime) -> dict:
        return self.tokens.cleanup_expired_refresh_tokens(current_time)
    
    # 시스템 이벤트 관련 메서드들 (system_event_service로 위임)
    def cleanup_old_system_events(self, current_time: datetime) -> dict:
        return self.system_events.cleanup_old_system_events(current_time)
    
    # 카운트 동기화 관련 메서드들 (count_service로 위임)
    def sync_post_like_counts(self) -> dict:
        return self.counts.sync_post_like_counts()
    
    def sync_user_follow_counts(self) -> dict:
        return self.counts.sync_user_follow_counts()
        
    def sync_all_counts(self) -> dict:
        return self.counts.sync_all_counts()


# 전역 데이터베이스 서비스 인스턴스 (기존 코드와 호환)
db_service = DatabaseService()
