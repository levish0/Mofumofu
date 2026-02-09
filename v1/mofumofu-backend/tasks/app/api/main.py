from fastapi import APIRouter
from app.api.routes.common.status import router as status_router
from app.api.routes.token.cleanup import router as token_cleanup_router
from app.api.routes.search.reindex import router as search_reindex_router
from app.api.routes.search.index import router as search_index_router
from app.api.routes.markdown.render import router as markdown_render_router
from app.api.routes.count.sync import router as count_sync_router
from app.api.routes.email.send import router as email_send_router

api_router = APIRouter()

# 공통 기능 라우터 추가
api_router.include_router(status_router)

# 토큰 관련 라우터 추가
api_router.include_router(token_cleanup_router)

# 검색 관련 라우터 추가
api_router.include_router(search_reindex_router)
api_router.include_router(search_index_router)

# 마크다운 관련 라우터 추가
api_router.include_router(markdown_render_router, prefix="/markdown")

# 카운트 동기화 관련 라우터 추가
api_router.include_router(count_sync_router, prefix="/count")

# 이메일 관련 라우터 추가
api_router.include_router(email_send_router)
