from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from sqlalchemy.engine import Engine
from app.core.config import settings
import logging
from typing import Generator

logger = logging.getLogger(__name__)


class BaseDatabaseService:
    """기본 데이터베이스 서비스 클래스"""

    def __init__(self) -> None:
        # PostgreSQL 동기 연결 URL (psycopg2 드라이버 사용)
        self.database_url = f"postgresql://{settings.POSTGRES_USER}:{settings.POSTGRES_PASSWORD}@{settings.POSTGRES_HOST}:{settings.POSTGRES_PORT}/{settings.POSTGRES_NAME}"

        # 동기 엔진 생성
        self.engine: Engine = create_engine(
            self.database_url,
            echo=False,  # 개발 시에는 True로 설정하면 SQL 쿼리 로그를 볼 수 있음
            pool_size=5,
            max_overflow=10,
            pool_pre_ping=True,  # 연결 상태 확인
        )

        # 동기 세션 팩토리
        self.session_factory: sessionmaker[Session] = sessionmaker(
            self.engine, expire_on_commit=False
        )

    def get_session(self) -> Session:
        """데이터베이스 세션을 반환합니다."""
        return self.session_factory()

    def get_session_context(self) -> Generator[Session, None, None]:
        """컨텍스트 매니저로 세션 사용"""
        session = self.get_session()
        try:
            yield session
            session.commit()
        except Exception:
            session.rollback()
            raise
        finally:
            session.close()

    def close(self) -> None:
        """데이터베이스 연결을 종료합니다."""
        self.engine.dispose()


# 전역 기본 데이터베이스 서비스 인스턴스
base_db_service = BaseDatabaseService()
