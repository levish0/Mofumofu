from sqlalchemy import Column, String, Boolean, Text, Integer
from sqlalchemy.dialects.postgresql import UUID
from .base import Base
import uuid


class User(Base):
    __tablename__ = "users"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(20), nullable=False)
    handle = Column(String(20), nullable=False, unique=True, index=True)
    email = Column(String(254), nullable=False, unique=True)
    password = Column(Text, nullable=True)
    is_verified = Column(Boolean, nullable=False, default=False)
    profile_image = Column(Text, nullable=True)
    banner_image = Column(Text, nullable=True)
    follower_count = Column(Integer, default=0, nullable=False)
    following_count = Column(Integer, default=0, nullable=False)

    # 관계 정의는 __init__.py에서 모든 모델 로드 후 설정

    def __repr__(self):
        return f"<User(handle='{self.handle}', email='{self.email}')>"
