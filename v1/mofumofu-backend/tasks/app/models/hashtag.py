from sqlalchemy import Column, String
from sqlalchemy.dialects.postgresql import UUID
from .base import Base
import uuid


class HashTag(Base):
    __tablename__ = "hash_tags"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(50), nullable=False, unique=True, index=True)

    # 관계 정의는 __init__.py에서 모든 모델 로드 후 설정

    def __repr__(self):
        return f"<HashTag(name='{self.name}')>"
