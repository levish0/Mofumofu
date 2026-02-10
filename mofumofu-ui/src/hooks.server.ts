import type { Handle } from '@sveltejs/kit';
import { createServerApi } from '$lib/api';

export const handle: Handle = async ({ event, resolve }) => {
	// cookie에서 인증 정보 추출하여 서버 API 인스턴스 생성
	const cookieHeader = event.request.headers.get('cookie');
	const api = createServerApi(event.fetch, cookieHeader);
	event.locals.api = api;

	return resolve(event);
};
