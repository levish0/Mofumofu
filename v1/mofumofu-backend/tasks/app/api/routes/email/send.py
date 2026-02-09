from fastapi import APIRouter
from app.tasks.email_tasks import (
    send_email_task,
    send_reset_password_email_task,
    send_email_verification_task,
    get_email_task_status,
)
from pydantic import BaseModel, EmailStr
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/email", tags=["Email"])


class EmailRequest(BaseModel):
    email_to: EmailStr
    subject: str
    html_content: str


class ResetPasswordEmailRequest(BaseModel):
    email_to: EmailStr
    email: EmailStr
    token: str


class EmailVerificationRequest(BaseModel):
    email_to: EmailStr
    username: str
    verification_token: str


@router.post("/send")
async def send_email(request: EmailRequest):
    """
    이메일을 보냅니다.

    Args:
        request: 이메일 정보가 포함된 요청 객체

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = send_email_task.delay(
            email_to=request.email_to,
            subject=request.subject,
            html_content=request.html_content,
        )

        return {
            "message": "이메일 전송 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"이메일 전송 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.post("/send-reset-password")
async def send_reset_password_email(request: ResetPasswordEmailRequest):
    """
    비밀번호 재설정 이메일을 보냅니다.

    Args:
        request: 비밀번호 재설정 이메일 정보가 포함된 요청 객체

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = send_reset_password_email_task.delay(
            email_to=request.email_to,
            email=request.email,
            token=request.token,
        )

        return {
            "message": "비밀번호 재설정 이메일 전송 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"비밀번호 재설정 이메일 전송 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}


@router.post("/send-verification")
async def send_email_verification(request: EmailVerificationRequest):
    """
    이메일 인증 메일을 보냅니다.

    Args:
        request: 이메일 인증 정보가 포함된 요청 객체

    Returns:
        dict: 작업 ID와 상태
    """
    try:
        task = send_email_verification_task.delay(
            email_to=request.email_to,
            username=request.username,
            verification_token=request.verification_token,
        )

        return {
            "message": "이메일 인증 메일 전송 작업이 큐에 추가되었습니다",
            "task_id": task.id,
            "status": "PENDING",
        }
    except Exception as e:
        logger.error(f"이메일 인증 메일 전송 작업 큐 추가 실패: {str(e)}")
        return {"error": f"작업 큐 추가 실패: {str(e)}", "status": "FAILED"}



@router.get("/status/{task_id}")
async def get_email_status(task_id: str):
    """
    이메일 작업의 상태를 확인합니다.

    Args:
        task_id: 작업 ID

    Returns:
        dict: 작업 상태와 결과
    """
    try:
        return get_email_task_status(task_id)
    except Exception as e:
        logger.error(f"이메일 작업 상태 조회 실패: {str(e)}")
        return {
            "error": f"작업 상태 조회 실패: {str(e)}",
            "task_id": task_id,
            "state": "ERROR",
        }