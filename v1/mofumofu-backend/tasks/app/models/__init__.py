from sqlalchemy.orm import relationship

from .base import Base
from .user import User
from .post import Post, post_hash_tags
from .hashtag import HashTag
from .comment import Comment
from .refresh_token import UserRefreshToken
from .system_event import SystemEvent
from .like import Like
from .follow import Follow
from .report import Report

# 모든 클래스가 로드된 후 관계 설정
# User 관계
User.posts = relationship("Post", back_populates="user", lazy="select")
User.refresh_tokens = relationship(
    "UserRefreshToken", back_populates="user", lazy="select"
)

# Post 관계
Post.user = relationship("User", back_populates="posts")
Post.hashtags = relationship(
    "HashTag", secondary=post_hash_tags, back_populates="posts"
)
Post.comments = relationship("Comment", back_populates="post", lazy="select")

# HashTag 관계
HashTag.posts = relationship(
    "Post", secondary=post_hash_tags, back_populates="hashtags"
)

# Comment 관계
Comment.post = relationship("Post", back_populates="comments")
Comment.user = relationship("User")

# UserRefreshToken 관계
UserRefreshToken.user = relationship("User", back_populates="refresh_tokens")

# SystemEvent 관계
SystemEvent.user = relationship("User", back_populates="system_events")

# User에 system_events 관계 추가
User.system_events = relationship("SystemEvent", back_populates="user", lazy="select")

# Like 관계
Like.user = relationship("User")
Like.post = relationship("Post")
Like.comment = relationship("Comment")

# Follow 관계
Follow.follower = relationship("User", foreign_keys=[Follow.follower_id])
Follow.followee = relationship("User", foreign_keys=[Follow.followee_id])

# Report 관계
Report.reporter = relationship("User", foreign_keys=[Report.reporter_id])
Report.resolved_by_user = relationship("User", foreign_keys=[Report.resolved_by])

__all__ = [
    "Base",
    "User",
    "Post",
    "HashTag",
    "Comment",
    "UserRefreshToken",
    "SystemEvent",
    "Like",
    "Follow",
    "Report",
    "post_hash_tags",
]
