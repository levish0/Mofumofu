import logging
from typing import Dict, Any, Optional
import meilisearch
from app.core.config import settings
from app.services.db_service import db_service

logger = logging.getLogger(__name__)


class MeilisearchService:
    def __init__(self) -> None:
        self.url: str = getattr(settings, "MEILISEARCH_HOST", "http://localhost:7700")
        api_key: str = getattr(settings, "MEILISEARCH_API_KEY", "")
        # API 키가 빈 문자열이면 None으로 설정 (로컬 개발환경)
        self.api_key: Optional[str] = api_key if api_key else None
        self.client: meilisearch.Client = meilisearch.Client(self.url, self.api_key)
        self.index_name: str = "posts"

    def get_index_stats(self) -> Dict[str, Any]:
        """색인 통계 조회"""
        try:
            index = self.client.index(self.index_name)
            stats = index.get_stats()
            
            # Meilisearch 응답 객체를 딕셔너리로 변환 (올바른 속성명 사용)
            field_dist = getattr(stats, "field_distribution", None)
            field_dist_dict = {}
            if field_dist:
                # FieldDistribution 객체의 실제 딕셔너리 데이터 추출
                if hasattr(field_dist, '__dict__'):
                    # 내부 딕셔너리에서 실제 필드 분포 데이터 찾기
                    for key, value in field_dist.__dict__.items():
                        if isinstance(value, dict) and not key.startswith('_'):
                            field_dist_dict = value
                            break
                    if not field_dist_dict and '_FieldDistribution__dict' in field_dist.__dict__:
                        field_dist_dict = field_dist.__dict__['_FieldDistribution__dict']
            
            stats_dict = {
                "numberOfDocuments": getattr(stats, "number_of_documents", 0),
                "isIndexing": getattr(stats, "is_indexing", False),
                "fieldDistribution": field_dist_dict,
            }
            
            return {"status": "success", "stats": stats_dict}
        except Exception as e:
            logger.error(f"색인 통계 조회 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def clear_index(self) -> Dict[str, Any]:
        """색인 전체 삭제 (문서만)"""
        try:
            index = self.client.index(self.index_name)
            task = index.delete_all_documents()
            return {
                "status": "success",
                "task_uid": task.task_uid,
                "message": "색인 삭제 작업이 시작되었습니다",
            }
        except Exception as e:
            logger.error(f"색인 삭제 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def delete_index(self) -> Dict[str, Any]:
        """색인 완전 삭제 (색인 자체 삭제)"""
        try:
            self.client.delete_index(self.index_name)
            logger.info(f"색인 '{self.index_name}' 완전 삭제 완료")
            return {
                "status": "success",
                "message": f"색인 '{self.index_name}' 완전 삭제 완료",
            }
        except Exception as e:
            logger.error(f"색인 완전 삭제 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def reindex_all_posts(self) -> Dict[str, Any]:
        """전체 포스트 재색인 (Rust 패턴과 동일)"""
        try:
            # 1. 기존 색인 완전 삭제
            self.delete_index()

            # 2. DB에서 모든 포스트 조회
            posts = db_service.get_all_posts_for_indexing()

            if not posts:
                return {
                    "status": "success",
                    "message": "색인할 포스트가 없습니다",
                    "indexed_count": 0,
                }

            # 3. 색인 설정
            self.setup_index()

            # 4. Meilisearch 형태로 변환 (경량화 - 검색용 필드만)
            documents = []
            for post in posts:
                document = {
                    "id": str(post["id"]),
                    "title": post.get("title", ""),
                    "summary": post.get("summary"),
                    "content": post.get("content", ""),
                    "user_id": post["user_id"],
                    "user_handle": post.get("user_handle", ""),
                    "user_name": post.get("user_name", ""),
                    "hashtags": post.get("hashtags", [])
                    if post.get("hashtags")
                    else [],
                    "created_at": int(post["created_at"].timestamp())
                    if post.get("created_at")
                    else 0,
                    "like_count": post.get("like_count", 0),
                    "comment_count": post.get("comment_count", 0),
                    "view_count": post.get("view_count", 0),
                }
                documents.append(document)

            # 5. 색인에 문서 추가 (Rust index_post와 동일하게)
            index = self.client.index(self.index_name)
            task = index.add_documents(documents, primary_key="id")

            return {
                "status": "success",
                "message": f"전체 재색인 완료: {len(documents)}개 포스트",
                "indexed_count": len(documents),
                "task_uid": task.task_uid,
            }

        except Exception as e:
            logger.error(f"전체 재색인 실패: {str(e)}")
            return {"status": "error", "error": str(e), "indexed_count": 0}

    def get_task_status(self, task_uid: int) -> Dict[str, Any]:
        """Meilisearch 태스크 상태 확인"""
        try:
            task = self.client.get_task(task_uid)
            return {"status": "success", "task": task}
        except Exception as e:
            logger.error(f"태스크 상태 확인 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def get_health(self) -> Dict[str, Any]:
        """Meilisearch 상태 확인"""
        try:
            health = self.client.health()
            # 헬스체크 응답을 딕셔너리로 변환
            health_dict = {"status": getattr(health, "status", "unknown")}
            return {"status": "success", "health": health_dict}
        except Exception as e:
            logger.error(f"Meilisearch 상태 확인 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def setup_index(self) -> None:
        """포스트 색인 설정 (Rust setup_posts_index와 동일)"""
        index = self.client.index(self.index_name)

        # 검색 가능한 필드 설정
        index.update_searchable_attributes(
            ["title", "content", "summary", "hashtags", "user_handle", "user_name"]
        )

        # 필터링 가능한 필드 설정
        index.update_filterable_attributes(
            [
                "user_id",
                "user_handle",
                "hashtags",
                "created_at",
                "like_count",
                "view_count",
            ]
        )

        # 정렬 가능한 필드 설정
        index.update_sortable_attributes(
            ["created_at", "like_count", "view_count", "comment_count"]
        )

    def index_single_post(self, post_id: str) -> Dict[str, Any]:
        """단일 포스트 색인 (경량화 버전)"""
        try:
            # DB에서 해당 포스트 조회
            posts = db_service.get_posts_by_ids([post_id])

            if not posts:
                return {"status": "error", "error": f"포스트를 찾을 수 없음: {post_id}"}

            post = posts[0]
            document = {
                "id": str(post["id"]),
                "title": post.get("title", ""),
                "summary": post.get("summary"),
                "content": post.get("content", ""),
                "user_id": post["user_id"],
                "user_handle": post.get("user_handle", ""),
                "user_name": post.get("user_name", ""),
                "hashtags": post.get("hashtags", []) if post.get("hashtags") else [],
                "created_at": int(post["created_at"].timestamp())
                if post.get("created_at")
                else 0,
                "like_count": post.get("like_count", 0),
                "comment_count": post.get("comment_count", 0),
                "view_count": post.get("view_count", 0),
            }

            index = self.client.index(self.index_name)
            task = index.add_documents([document], primary_key="id")

            return {
                "status": "success",
                "message": f"포스트 색인 완료: {post_id}",
                "task_uid": task.task_uid,
            }

        except Exception as e:
            logger.error(f"단일 포스트 색인 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def update_single_post(self, post_id: str) -> Dict[str, Any]:
        """단일 포스트 업데이트 (경량화 버전)"""
        try:
            # DB에서 해당 포스트 조회
            posts = db_service.get_posts_by_ids([post_id])

            if not posts:
                return {"status": "error", "error": f"포스트를 찾을 수 없음: {post_id}"}

            post = posts[0]
            document = {
                "id": str(post["id"]),
                "title": post.get("title", ""),
                "summary": post.get("summary"),
                "content": post.get("content", ""),
                "user_id": post["user_id"],
                "user_handle": post.get("user_handle", ""),
                "user_name": post.get("user_name", ""),
                "hashtags": post.get("hashtags", []) if post.get("hashtags") else [],
                "created_at": int(post["created_at"].timestamp())
                if post.get("created_at")
                else 0,
                "like_count": post.get("like_count", 0),
                "comment_count": post.get("comment_count", 0),
                "view_count": post.get("view_count", 0),
            }

            index = self.client.index(self.index_name)
            task = index.add_documents([document], primary_key="id")

            return {
                "status": "success",
                "message": f"포스트 업데이트 완료: {post_id}",
                "task_uid": task.task_uid,
            }

        except Exception as e:
            logger.error(f"단일 포스트 업데이트 실패: {str(e)}")
            return {"status": "error", "error": str(e)}

    def delete_single_post(self, post_id: str) -> Dict[str, Any]:
        """단일 포스트 삭제 (Rust delete_post와 동일)"""
        try:
            index = self.client.index(self.index_name)
            task = index.delete_document(post_id)

            return {
                "status": "success",
                "message": f"포스트 삭제 완료: {post_id}",
                "task_uid": task.task_uid,
            }

        except Exception as e:
            logger.error(f"단일 포스트 삭제 실패: {str(e)}")
            return {"status": "error", "error": str(e)}


meilisearch_service = MeilisearchService()
