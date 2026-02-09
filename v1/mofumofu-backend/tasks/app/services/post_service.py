from sqlalchemy.orm import joinedload, selectinload
from app.models import Post
from app.services.base_db_service import base_db_service
from typing import Optional, List, Dict, Any
import logging

logger = logging.getLogger(__name__)


class PostService:
    """포스트 관련 데이터베이스 서비스 (ORM 기반)"""

    def __init__(self):
        self.db = base_db_service

    def get_post_by_id(self, post_id: str) -> Optional[Post]:
        """
        post_id로 포스트를 조회합니다.
        
        Args:
            post_id: 포스트 UUID
            
        Returns:
            Optional[Post]: 포스트 객체 또는 None
        """
        with self.db.session_factory() as session:
            try:
                post = (
                    session.query(Post)
                    .options(joinedload(Post.user))
                    .filter(Post.id == post_id)
                    .first()
                )
                
                if post:
                    logger.info(f"포스트 조회 성공: post_id={post_id}")
                    return post
                else:
                    logger.warning(f"포스트를 찾을 수 없음: post_id={post_id}")
                    return None
                    
            except Exception as e:
                logger.error(f"포스트 조회 실패: {str(e)}")
                return None

    def update_post_thumbnail(self, post_id: str, thumbnail_url: Optional[str]) -> bool:
        """
        포스트의 썸네일 이미지 URL을 업데이트합니다.
        
        Args:
            post_id: 포스트 UUID
            thumbnail_url: 새로운 썸네일 이미지 URL (None이면 썸네일 제거)
            
        Returns:
            bool: 업데이트 성공 여부
        """
        with self.db.session_factory() as session:
            try:
                post = session.query(Post).filter(Post.id == post_id).first()
                
                if post:
                    post.thumbnail_image = thumbnail_url
                    session.commit()
                    
                    action = "제거" if thumbnail_url is None else "업데이트"
                    logger.info(
                        f"포스트 썸네일 {action} 성공: post_id={post_id}, url={thumbnail_url}"
                    )
                    return True
                else:
                    logger.warning(f"포스트를 찾을 수 없음: post_id={post_id}")
                    return False
                    
            except Exception as e:
                logger.error(f"포스트 썸네일 업데이트 실패: {str(e)}")
                session.rollback()
                return False

    def update_post_render_and_toc(self, post_id: str, html_content: str, toc_items: list) -> bool:
        """
        포스트의 렌더링된 HTML과 TOC를 업데이트합니다.
        
        Args:
            post_id: 포스트 UUID
            html_content: 렌더링된 HTML 콘텐츠
            toc_items: TOC 아이템 리스트
            
        Returns:
            bool: 업데이트 성공 여부
        """
        with self.db.session_factory() as session:
            try:
                logger.info(f"포스트 조회 시작: post_id={post_id}")
                post = session.query(Post).filter(Post.id == post_id).first()
                
                if post:
                    logger.info(f"포스트 찾음: post_id={post_id}, 기존 render 길이={len(post.render) if post.render else 0}")
                    
                    # 업데이트 전 값 로깅
                    old_render = post.render
                    old_toc = post.toc
                    
                    post.render = html_content
                    post.toc = toc_items
                    
                    logger.info(f"업데이트 시도: render 길이={len(html_content)}, toc_count={len(toc_items)}")
                    session.commit()
                    logger.info("세션 커밋 완료")
                    
                    # 업데이트 확인
                    session.refresh(post)
                    logger.info(f"업데이트 후 확인: render 길이={len(post.render) if post.render else 0}, toc_count={len(post.toc) if post.toc else 0}")
                    
                    return True
                else:
                    logger.warning(f"포스트를 찾을 수 없음: post_id={post_id}")
                    return False
                    
            except Exception as e:
                logger.error(f"포스트 render/toc 업데이트 실패: {str(e)}")
                logger.exception(e)  # 전체 스택 트레이스 로깅
                session.rollback()
                return False

    def get_all_posts_for_indexing(self) -> List[Dict[str, Any]]:
        """
        Meilisearch 색인을 위해 모든 포스트 데이터를 조회합니다.
        
        Returns:
            List[Dict[str, Any]]: 색인용 포스트 데이터 리스트
        """
        with self.db.session_factory() as session:
            try:
                logger.info("포스트 데이터 조회 시작...")
                
                # ORM을 사용하여 관계를 포함한 포스트들 조회
                posts = (
                    session.query(Post)
                    .options(
                        joinedload(Post.user),  # 사용자 정보 즉시 로드
                        selectinload(Post.hashtags)  # 해시태그들 별도 쿼리로 로드
                    )
                    .order_by(Post.created_at.desc())
                    .all()
                )
                
                total_posts = len(posts)
                logger.info(f"전체 포스트 수: {total_posts}")
                
                if total_posts == 0:
                    logger.warning("데이터베이스에 포스트가 없습니다")
                    return []
                
                # ORM 객체를 딕셔너리로 변환
                result_posts = []
                for idx, post in enumerate(posts, 1):
                    logger.info(f"포스트 {idx}: id={post.id}, title={post.title}")
                    
                    # 해시태그 이름들 추출
                    hashtag_names = [tag.name for tag in post.hashtags] if post.hashtags else []
                    
                    result_posts.append({
                        "id": str(post.id),
                        "user_id": str(post.user_id),
                        "user_handle": post.user.handle if post.user else "",
                        "user_name": post.user.name if post.user else "",
                        "user_avatar": post.user.profile_image if post.user else None,
                        "title": post.title or "",
                        "content": post.content or "",
                        "summary": post.summary,
                        "slug": post.slug or "",
                        "thumbnail_image": post.thumbnail_image,
                        "hashtags": hashtag_names,
                        "created_at": post.created_at,
                        "like_count": post.like_count or 0,
                        "view_count": post.view_count or 0,
                        "comment_count": post.comment_count or 0,
                    })
                
                logger.info(f"색인용 포스트 데이터 조회 완료: {len(result_posts)}개")
                return result_posts
                
            except Exception as e:
                logger.error(f"색인용 포스트 데이터 조회 실패: {str(e)}")
                import traceback
                
                logger.error(f"스택 트레이스: {traceback.format_exc()}")
                return []

    def get_posts_by_ids(self, post_ids: List[str]) -> List[Dict[str, Any]]:
        """
        특정 포스트 ID들로 포스트 데이터를 조회합니다.
        
        Args:
            post_ids: 조회할 포스트 UUID 리스트
            
        Returns:
            List[Dict[str, Any]]: 색인용 포스트 데이터 리스트
        """
        with self.db.session_factory() as session:
            try:
                # ORM으로 특정 ID들의 포스트 조회
                posts = (
                    session.query(Post)
                    .options(
                        joinedload(Post.user),
                        selectinload(Post.hashtags)
                    )
                    .filter(Post.id.in_(post_ids))
                    .order_by(Post.created_at.desc())
                    .all()
                )
                
                result_posts = []
                for post in posts:
                    # 해시태그 이름들 추출
                    hashtag_names = [tag.name for tag in post.hashtags] if post.hashtags else []
                    
                    result_posts.append({
                        "id": str(post.id),
                        "user_id": str(post.user_id),
                        "user_handle": post.user.handle if post.user else "",
                        "user_name": post.user.name if post.user else "",
                        "user_avatar": post.user.profile_image if post.user else None,
                        "title": post.title or "",
                        "content": post.content or "",
                        "summary": post.summary,
                        "slug": post.slug or "",
                        "thumbnail_image": post.thumbnail_image,
                        "hashtags": hashtag_names,
                        "created_at": post.created_at,
                        "like_count": post.like_count or 0,
                        "view_count": post.view_count or 0,
                        "comment_count": post.comment_count or 0,
                    })
                
                return result_posts
                
            except Exception as e:
                logger.error(f"특정 포스트 데이터 조회 실패: {str(e)}")
                import traceback
                
                logger.error(f"스택 트레이스: {traceback.format_exc()}")
                return []


# 전역 포스트 서비스 인스턴스
post_service = PostService()