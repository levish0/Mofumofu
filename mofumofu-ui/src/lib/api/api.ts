import ky, { type KyInstance } from 'ky';
import { API_URL } from './config';
import { ApiError } from './errors';

const apiOptions = {
	prefixUrl: API_URL,
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	},
	timeout: 10 * 1000,
	retry: 2,
	hooks: {
		afterResponse: [
			async (_request: Request, _options: object, response: Response) => {
				if (!response.ok) {
					const contentType = response.headers.get('content-type');

					if (contentType?.includes('application/json')) {
						try {
							const jsonData = await response.json();
							// 표준 에러 응답 형식인지 확인 (status, code 필드 존재)
							if (typeof jsonData.status === 'number' && typeof jsonData.code === 'string') {
								throw new ApiError(jsonData.status, jsonData.code, jsonData.details);
							} else {
								// 비표준 응답 (예: 409 conflict) - raw data 포함
								throw new ApiError(
									response.status,
									'non_standard_response',
									response.statusText,
									jsonData
								);
							}
						} catch (e) {
							if (e instanceof ApiError) throw e;
							// JSON 파싱 실패
							throw new ApiError(response.status, 'parse_error', response.statusText);
						}
					} else {
						let details = response.statusText;
						try {
							const text = await response.text();
							if (text) details = text;
						} catch {
							// ignore
						}
						throw new ApiError(response.status, 'unknown_error', details);
					}
				}
				return response;
			}
		]
	}
};

/** 브라우저용 API 클라이언트 (credentials: include로 쿠키 자동 전송) */
export const Api = ky.create({
	...apiOptions,
	credentials: 'include'
});

/** SSR용 API 클라이언트 생성 (SvelteKit fetch 주입으로 쿠키 자동 전송) */
export function createServerApi(
	fetch: typeof globalThis.fetch,
	cookieHeader?: string | null
): KyInstance {
	const headers: Record<string, string> = {
		...apiOptions.headers
	};

	// SSR에서 쿠키를 수동으로 전달 (cross-origin 요청)
	if (cookieHeader) {
		headers['Cookie'] = cookieHeader;
	}

	return ky.create({
		...apiOptions,
		fetch,
		credentials: 'include',
		headers
	});
}
