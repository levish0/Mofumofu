/**
 * Backend error response structure.
 */
export interface ErrorResponse {
	status: number;
	code: string;
	details?: string;
}

export class ApiError extends Error {
	public readonly status: number;
	public readonly code: string;
	public readonly details?: string;
	public readonly data?: unknown;

	constructor(status: number, code: string, details?: string, data?: unknown) {
		super(details ?? code);
		this.name = 'ApiError';
		this.status = status;
		this.code = code;
		this.details = details;
		this.data = data;
	}

	/**
	 * 비표준 응답 데이터 가져오기 (예: 409 conflict 응답)
	 */
	getData<T>(): T | undefined {
		return this.data as T | undefined;
	}
}

// Type guard to check if an error is an ApiError.
export function isApiError(error: unknown): error is ApiError {
	return error instanceof ApiError;
}

// Convert unknown error to ApiError.
export function toApiError(error: unknown): ApiError {
	if (isApiError(error)) {
		return error;
	}

	if (error instanceof Error) {
		return new ApiError(500, 'unknown_error', error.message);
	}

	return new ApiError(500, 'unknown_error', String(error));
}
