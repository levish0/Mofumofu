from sqlalchemy import Column, DateTime, ForeignKey, Enum, Text, JSON
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.sql import func
from .base import Base
import uuid
import enum


class ReportTargetType(enum.Enum):
    USER = "user"
    POST = "post"
    COMMENT = "comment"


class ReportStatus(enum.Enum):
    PENDING = "pending"
    REVIEWING = "reviewing"
    RESOLVED = "resolved"
    DISMISSED = "dismissed"


class Report(Base):
    __tablename__ = "reports"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    reporter_id = Column(UUID(as_uuid=True), ForeignKey("users.id"), nullable=True)
    target_type = Column(Enum(ReportTargetType), nullable=False)
    target_id = Column(UUID(as_uuid=True), nullable=False)
    reasons = Column(JSON, nullable=False)
    description = Column(Text, nullable=True)
    status = Column(Enum(ReportStatus), nullable=False, default=ReportStatus.PENDING)
    admin_note = Column(Text, nullable=True)
    resolved_by = Column(UUID(as_uuid=True), ForeignKey("users.id"), nullable=True)
    resolved_at = Column(DateTime(timezone=True), nullable=True)
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(DateTime(timezone=True), onupdate=func.now(), nullable=True)

    # 관계 정의는 __init__.py에서 모든 모델 로드 후 설정

    def __repr__(self):
        return f"<Report(target_type='{self.target_type}', target_id='{self.target_id}', status='{self.status}')>"
