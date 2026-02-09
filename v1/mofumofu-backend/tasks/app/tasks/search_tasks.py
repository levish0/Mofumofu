from app.core.celery_app import celery_app
from app.services.meilisearch_service import meilisearch_service
import logging

logger = logging.getLogger(__name__)


@celery_app.task(name="reindex_all_posts")
def reindex_all_posts_task():
    """
    전체 포스트를 Meilisearch에 재색인하는 태스크

    - 기존 색인을 모두 삭제
    - 데이터베이스에서 모든 포스트를 조회
    - Meilisearch에 새로 색인 생성

    주기적 실행(예: 매일 새벽)이나 수동 실행 모두 가능

    Returns:
        dict: 재색인 결과 및 색인된 포스트 수
    """
    try:
        logger.info("전체 포스트 재색인 작업 시작")

        result = meilisearch_service.reindex_all_posts()

        if result["status"] == "success":
            logger.info(f"전체 포스트 재색인 완료: {result['indexed_count']}개 포스트")
            return {
                "status": "SUCCESS",
                "indexed_count": result["indexed_count"],
                "message": result["message"],
                "task_uids": result.get("task_uids", []),
            }
        else:
            logger.error(
                f"전체 포스트 재색인 실패: {result.get('error', 'Unknown error')}"
            )
            return {
                "status": "FAILURE",
                "error": result.get("error", "Unknown error"),
                "indexed_count": result.get("indexed_count", 0),
            }

    except Exception as exc:
        logger.error(f"전체 포스트 재색인 작업 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc), "indexed_count": 0}


@celery_app.task(name="check_meilisearch_health")
def check_meilisearch_health_task():
    """
    Meilisearch 서버 상태를 확인하는 태스크

    헬스체크나 모니터링 목적으로 사용

    Returns:
        dict: Meilisearch 서버 상태
    """
    try:
        logger.info("Meilisearch 헬스체크 시작")

        result = meilisearch_service.get_health()

        if result["status"] == "success":
            logger.info("Meilisearch 헬스체크 성공")
            return {
                "status": "SUCCESS",
                "health": result["health"],
                "message": "Meilisearch 서버가 정상 작동 중입니다",
            }
        else:
            logger.error(
                f"Meilisearch 헬스체크 실패: {result.get('error', 'Unknown error')}"
            )
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}

    except Exception as exc:
        logger.error(f"Meilisearch 헬스체크 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}


@celery_app.task(name="get_search_index_stats")
def get_search_index_stats_task():
    """
    검색 색인 통계를 조회하는 태스크

    모니터링이나 관리 목적으로 사용

    Returns:
        dict: 색인 통계 정보
    """
    try:
        logger.info("검색 색인 통계 조회 시작")

        result = meilisearch_service.get_index_stats()

        if result["status"] == "success":
            logger.info("검색 색인 통계 조회 성공")
            return {
                "status": "SUCCESS",
                "stats": result["stats"],
                "message": "검색 색인 통계 조회 완료",
            }
        else:
            logger.error(
                f"검색 색인 통계 조회 실패: {result.get('error', 'Unknown error')}"
            )
            return {"status": "FAILURE", "error": result.get("error", "Unknown error")}

    except Exception as exc:
        logger.error(f"검색 색인 통계 조회 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}


@celery_app.task(name="index_single_post")
def index_single_post_task(post_id: str):
    """
    단일 포스트를 Meilisearch에 색인하는 태스크

    Args:
        post_id: 포스트 UUID

    Returns:
        dict: 색인 결과
    """
    try:
        logger.info(f"단일 포스트 색인 시작: {post_id}")

        result = meilisearch_service.index_single_post(post_id)

        if result["status"] == "success":
            logger.info(f"단일 포스트 색인 완료: {post_id}")
            return {
                "status": "SUCCESS",
                "message": result["message"],
                "post_id": post_id,
            }
        else:
            logger.error(
                f"단일 포스트 색인 실패: {result.get('error', 'Unknown error')}"
            )
            return {
                "status": "FAILURE",
                "error": result.get("error", "Unknown error"),
                "post_id": post_id,
            }

    except Exception as exc:
        logger.error(f"단일 포스트 색인 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc), "post_id": post_id}


@celery_app.task(name="update_single_post")
def update_single_post_task(post_id: str):
    """
    단일 포스트 색인을 업데이트하는 태스크

    Args:
        post_id: 포스트 UUID

    Returns:
        dict: 업데이트 결과
    """
    try:
        logger.info(f"단일 포스트 색인 업데이트 시작: {post_id}")

        result = meilisearch_service.update_single_post(post_id)

        if result["status"] == "success":
            logger.info(f"단일 포스트 색인 업데이트 완료: {post_id}")
            return {
                "status": "SUCCESS",
                "message": result["message"],
                "post_id": post_id,
            }
        else:
            logger.error(
                f"단일 포스트 색인 업데이트 실패: {result.get('error', 'Unknown error')}"
            )
            return {
                "status": "FAILURE",
                "error": result.get("error", "Unknown error"),
                "post_id": post_id,
            }

    except Exception as exc:
        logger.error(f"단일 포스트 색인 업데이트 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc), "post_id": post_id}


@celery_app.task(name="delete_single_post")
def delete_single_post_task(post_id: str):
    """
    단일 포스트를 Meilisearch에서 삭제하는 태스크

    Args:
        post_id: 포스트 UUID

    Returns:
        dict: 삭제 결과
    """
    try:
        logger.info(f"단일 포스트 색인 삭제 시작: {post_id}")

        result = meilisearch_service.delete_single_post(post_id)

        if result["status"] == "success":
            logger.info(f"단일 포스트 색인 삭제 완료: {post_id}")
            return {
                "status": "SUCCESS",
                "message": result["message"],
                "post_id": post_id,
            }
        else:
            logger.error(
                f"단일 포스트 색인 삭제 실패: {result.get('error', 'Unknown error')}"
            )
            return {
                "status": "FAILURE",
                "error": result.get("error", "Unknown error"),
                "post_id": post_id,
            }

    except Exception as exc:
        logger.error(f"단일 포스트 색인 삭제 중 예외 발생: {str(exc)}")
        return {"status": "FAILURE", "error": str(exc), "post_id": post_id}
