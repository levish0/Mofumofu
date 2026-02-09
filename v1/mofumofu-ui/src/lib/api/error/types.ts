// src/lib/api/error/types.ts

export interface ErrorResponse {
	status: number;
	code: string;
	details: string | null;
}
