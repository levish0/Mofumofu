from sqlalchemy import func
from app.models import User, Post, Like, Follow
from app.services.base_db_service import base_db_service
import logging

logger = logging.getLogger(__name__)


class CountService:
    """Count synchronization service for likes and follows"""

    def __init__(self, db_service=None):
        self.db_service = db_service or base_db_service

    def sync_post_like_counts(self) -> dict:
        """
        모든 포스트의 like_count를 실제 likes 테이블의 개수로 동기화
        """
        try:
            logger.info("Starting post like count synchronization")
            
            with self.db_service.get_session() as session:
                # 실제 like 개수 계산하고 업데이트
                subquery = (
                    session.query(
                        Like.post_id,
                        func.count(Like.id).label('actual_count')
                    )
                    .group_by(Like.post_id)
                    .subquery()
                )
                
                # like_count가 다른 포스트들을 찾아서 업데이트
                updated_posts = (
                    session.query(Post)
                    .join(subquery, Post.id == subquery.c.post_id)
                    .filter(Post.like_count != subquery.c.actual_count)
                    .all()
                )
                
                updated_count = 0
                for post in updated_posts:
                    # 해당 포스트의 실제 like 개수 조회
                    actual_count = (
                        session.query(func.count(Like.id))
                        .filter(Like.post_id == post.id)
                        .scalar() or 0
                    )
                    
                    post.like_count = actual_count
                    updated_count += 1
                
                # like가 0인 포스트들도 처리 (subquery에서 누락된 것들)
                posts_without_likes = (
                    session.query(Post)
                    .outerjoin(Like, Post.id == Like.post_id)
                    .filter(Like.post_id.is_(None))
                    .filter(Post.like_count != 0)
                    .all()
                )
                
                for post in posts_without_likes:
                    post.like_count = 0
                    updated_count += 1
                
                session.commit()
                
            logger.info(f"Post like count sync completed. Updated {updated_count} posts.")
            
            return {
                "success": True,
                "updated_count": updated_count,
                "message": f"Successfully synchronized like counts for {updated_count} posts"
            }
            
        except Exception as exc:
            logger.error(f"Post like count synchronization failed: {exc}")
            return {
                "success": False,
                "error": str(exc)
            }

    def sync_user_follow_counts(self) -> dict:
        """
        모든 유저의 follower_count와 following_count를 실제 follows 테이블의 개수로 동기화
        """
        try:
            logger.info("Starting user follow count synchronization")
            
            with self.db_service.get_session() as session:
                follower_updated_count = 0
                following_updated_count = 0
                
                # follower_count 동기화
                follower_subquery = (
                    session.query(
                        Follow.followee_id,
                        func.count(Follow.id).label('actual_follower_count')
                    )
                    .group_by(Follow.followee_id)
                    .subquery()
                )
                
                users_with_followers = (
                    session.query(User)
                    .join(follower_subquery, User.id == follower_subquery.c.followee_id)
                    .filter(User.follower_count != follower_subquery.c.actual_follower_count)
                    .all()
                )
                
                for user in users_with_followers:
                    actual_follower_count = (
                        session.query(func.count(Follow.id))
                        .filter(Follow.followee_id == user.id)
                        .scalar() or 0
                    )
                    
                    user.follower_count = actual_follower_count
                    follower_updated_count += 1
                
                # follower가 0인 유저들도 처리
                users_without_followers = (
                    session.query(User)
                    .outerjoin(Follow, User.id == Follow.followee_id)
                    .filter(Follow.followee_id.is_(None))
                    .filter(User.follower_count != 0)
                    .all()
                )
                
                for user in users_without_followers:
                    user.follower_count = 0
                    follower_updated_count += 1
                
                # following_count 동기화
                following_subquery = (
                    session.query(
                        Follow.follower_id,
                        func.count(Follow.id).label('actual_following_count')
                    )
                    .group_by(Follow.follower_id)
                    .subquery()
                )
                
                users_with_following = (
                    session.query(User)
                    .join(following_subquery, User.id == following_subquery.c.follower_id)
                    .filter(User.following_count != following_subquery.c.actual_following_count)
                    .all()
                )
                
                for user in users_with_following:
                    actual_following_count = (
                        session.query(func.count(Follow.id))
                        .filter(Follow.follower_id == user.id)
                        .scalar() or 0
                    )
                    
                    user.following_count = actual_following_count
                    following_updated_count += 1
                
                # following이 0인 유저들도 처리
                users_without_following = (
                    session.query(User)
                    .outerjoin(Follow, User.id == Follow.follower_id)
                    .filter(Follow.follower_id.is_(None))
                    .filter(User.following_count != 0)
                    .all()
                )
                
                for user in users_without_following:
                    user.following_count = 0
                    following_updated_count += 1
                
                session.commit()
                
            total_updated = follower_updated_count + following_updated_count
            logger.info(f"User follow count sync completed. Updated {follower_updated_count} follower counts and {following_updated_count} following counts.")
            
            return {
                "success": True,
                "follower_updated_count": follower_updated_count,
                "following_updated_count": following_updated_count,
                "total_updated": total_updated,
                "message": f"Successfully synchronized follow counts. Follower: {follower_updated_count}, Following: {following_updated_count}"
            }
            
        except Exception as exc:
            logger.error(f"User follow count synchronization failed: {exc}")
            return {
                "success": False,
                "error": str(exc)
            }

    def sync_all_counts(self) -> dict:
        """
        모든 카운트(like, follow) 동기화를 한번에 실행
        """
        try:
            logger.info("Starting full count synchronization")
            
            # 포스트 좋아요 개수 동기화
            like_result = self.sync_post_like_counts()
            if not like_result["success"]:
                return {
                    "success": False,
                    "error": f"Like count sync failed: {like_result['error']}"
                }
            
            # 유저 팔로우 개수 동기화
            follow_result = self.sync_user_follow_counts()
            if not follow_result["success"]:
                return {
                    "success": False,
                    "error": f"Follow count sync failed: {follow_result['error']}"
                }
            
            logger.info("Full count synchronization completed successfully")
            
            return {
                "success": True,
                "like_sync": like_result,
                "follow_sync": follow_result,
                "message": "All count synchronization tasks completed successfully"
            }
            
        except Exception as exc:
            logger.error(f"Full count synchronization failed: {exc}")
            return {
                "success": False,
                "error": str(exc)
            }


# 전역 count 서비스 인스턴스
count_service = CountService()