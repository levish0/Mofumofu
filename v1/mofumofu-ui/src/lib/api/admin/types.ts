// Admin API 응답 타입들
export interface AdminStatusResponse {
	is_admin: boolean;
}

export interface AdminTaskResponse {
	success: boolean;
	message: string;
	data?: any;
}
