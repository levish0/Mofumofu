from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.routing import APIRoute
import logging

from app.api.main import api_router
from app.core.config import settings
from app.utils import build_email_templates

logger = logging.getLogger(__name__)


def custom_generate_unique_id(route: APIRoute) -> str:
    return f"{route.tags[0]}-{route.name}"


@asynccontextmanager
async def lifespan(app: FastAPI):
    """앱 생명주기 관리"""
    # Startup
    logger.info("Starting up mofumofu_tasks application...")
    
    # 이메일 템플릿 빌드
    logger.info("Building email templates...")
    build_email_templates()
    
    logger.info("Application startup completed")
    
    yield
    
    # Shutdown
    logger.info("Shutting down mofumofu_tasks application...")


app = FastAPI(
    title="mofumofu_tasks",
    openapi_url=f"{settings.API_STR}/openapi.json",
    generate_unique_id_function=custom_generate_unique_id,
    lifespan=lifespan,
)

app.include_router(api_router, prefix=settings.API_STR)
