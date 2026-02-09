from sqlalchemy import Column, String, DateTime, JSON, ForeignKey
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.sql import func
from .base import Base


class SystemEvent(Base):
    """시스템 이벤트 모델"""

    __tablename__ = "system_events"

    id = Column(
        UUID(as_uuid=True), primary_key=True, server_default=func.gen_random_uuid()
    )
    user_id = Column(
        UUID(as_uuid=True), ForeignKey("users.id", ondelete="SET NULL"), nullable=True
    )
    action_type = Column(String, nullable=False)
    target_id = Column(UUID(as_uuid=True), nullable=True)
    target_type = Column(
        String, nullable=True
    )  # "post", "hashtag", "user", "comment" 등
    event_metadata = Column("metadata", JSON, nullable=True)  # 추가 정보
    created_at = Column(
        DateTime(timezone=True), nullable=False, server_default=func.now()
    )
