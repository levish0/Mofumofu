"""이메일 관련 Celery 작업들"""

import logging
from typing import Any, Dict

from celery.result import AsyncResult

from app.core.celery_app import celery_app
from app.utils import (
    create_failure_response,
    create_success_response,
    generate_reset_password_email,
    generate_email_verification_email,
    send_email,
)

logger = logging.getLogger(__name__)


@celery_app.task(bind=True, name="send_email_task")
def send_email_task(
    self,
    email_to: str,
    subject: str,
    html_content: str,
) -> Dict[str, Any]:
    """이메일 보내기 작업
    
    Args:
        email_to: 수신자 이메일
        subject: 이메일 제목
        html_content: HTML 내용
        
    Returns:
        작업 결과 딕셔너리
    """
    try:
        logger.info(f"이메일 전송 시작: {email_to}")
        
        # 이메일 보내기
        send_email(
            email_to=email_to,
            subject=subject,
            html_content=html_content,
        )
        
        logger.info(f"이메일 전송 완료: {email_to}")
        return create_success_response(
            f"이메일이 성공적으로 전송되었습니다: {email_to}",
            email_to=email_to,
            subject=subject,
        )
        
    except Exception as e:
        logger.error(f"이메일 전송 실패: {email_to}, 오류: {str(e)}")
        return create_failure_response(
            f"이메일 전송 실패: {str(e)}",
            email_to=email_to,
            subject=subject,
        )


@celery_app.task(bind=True, name="send_reset_password_email_task")
def send_reset_password_email_task(
    self, email_to: str, email: str, token: str
) -> Dict[str, Any]:
    """비밀번호 재설정 이메일 보내기 작업
    
    Args:
        email_to: 수신자 이메일
        email: 사용자 이메일
        token: 재설정 토큰
        
    Returns:
        작업 결과 딕셔너리
    """
    try:
        logger.info(f"비밀번호 재설정 이메일 전송 시작: {email_to}")
        
        # 비밀번호 재설정 이메일 데이터 생성
        email_data = generate_reset_password_email(email_to, email, token)
        
        # 이메일 보내기
        send_email(
            email_to=email_to,
            subject=email_data.subject,
            html_content=email_data.html_content,
        )
        
        logger.info(f"비밀번호 재설정 이메일 전송 완료: {email_to}")
        return create_success_response(
            f"비밀번호 재설정 이메일이 성공적으로 전송되었습니다: {email_to}",
            email_to=email_to,
            subject=email_data.subject,
        )
        
    except Exception as e:
        logger.error(f"비밀번호 재설정 이메일 전송 실패: {email_to}, 오류: {str(e)}")
        return create_failure_response(
            f"비밀번호 재설정 이메일 전송 실패: {str(e)}",
            email_to=email_to,
        )


@celery_app.task(bind=True, name="send_email_verification_task")
def send_email_verification_task(
    self, email_to: str, username: str, verification_token: str
) -> Dict[str, Any]:
    """이메일 인증 메일 보내기 작업
    
    Args:
        email_to: 수신자 이메일
        username: 사용자명
        verification_token: 인증 토큰
        
    Returns:
        작업 결과 딕셔너리
    """
    try:
        logger.info(f"이메일 인증 메일 전송 시작: {email_to}")
        
        # 이메일 인증 메일 데이터 생성
        email_data = generate_email_verification_email(email_to, username, verification_token)
        
        # 이메일 보내기
        send_email(
            email_to=email_to,
            subject=email_data.subject,
            html_content=email_data.html_content,
        )
        
        logger.info(f"이메일 인증 메일 전송 완료: {email_to}")
        return create_success_response(
            f"이메일 인증 메일이 성공적으로 전송되었습니다: {email_to}",
            email_to=email_to,
            subject=email_data.subject,
        )
        
    except Exception as e:
        logger.error(f"이메일 인증 메일 전송 실패: {email_to}, 오류: {str(e)}")
        return create_failure_response(
            f"이메일 인증 메일 전송 실패: {str(e)}",
            email_to=email_to,
        )


def get_email_task_status(task_id: str) -> Dict[str, Any]:
    """이메일 작업 상태 확인
    
    Args:
        task_id: 작업 ID
        
    Returns:
        작업 상태 정보
    """
    try:
        result = AsyncResult(task_id, app=celery_app)
        
        if result.state == "PENDING":
            response = {
                "state": result.state,
                "status": "작업이 대기 중입니다",
            }
        elif result.state == "PROGRESS":
            response = {
                "state": result.state,
                "status": "작업이 진행 중입니다",
                "current": result.info.get("current", 0),
                "total": result.info.get("total", 1),
            }
        elif result.state == "SUCCESS":
            response = {
                "state": result.state,
                "status": "작업이 완료되었습니다",
                "result": result.result,
            }
        else:
            response = {
                "state": result.state,
                "status": "작업이 실패했습니다",
                "error": str(result.info),
            }
            
        return response
        
    except Exception as e:
        logger.error(f"작업 상태 확인 실패: {task_id}, 오류: {str(e)}")
        return {
            "state": "FAILURE",
            "status": "작업 상태 확인 실패",
            "error": str(e),
        }