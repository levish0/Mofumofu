import { privateApi } from '../private';
import type { AdminStatusResponse, AdminTaskResponse } from './types';

// 관리자 권한 확인
export async function checkAdminStatus(): Promise<AdminStatusResponse> {
	try {
		const response = await privateApi.get('v0/admin/status');
		return await response.json<AdminStatusResponse>();
	} catch (error) {
		console.error('Admin status check failed:', error);
		throw error;
	}
}

// 오래된 시스템 이벤트 정리
export async function cleanupOldEvents(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/cleanup/events');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Event cleanup failed:', error);
		throw error;
	}
}

// 만료된 리프레시 토큰 정리
export async function cleanupExpiredTokens(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/cleanup/tokens');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Token cleanup failed:', error);
		throw error;
	}
}

// Meilisearch 헬스체크
export async function checkSearchHealth(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/search/health');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Search health check failed:', error);
		throw error;
	}
}

// 전체 포스트 재색인
export async function reindexAllPosts(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/search/reindex-all');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Post reindex failed:', error);
		throw error;
	}
}

// 검색 색인 통계 조회
export async function getSearchStats(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/search/stats');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Search stats failed:', error);
		throw error;
	}
}

// 전체 카운트 동기화
export async function syncAllCounts(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/sync/all');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Count sync failed:', error);
		throw error;
	}
}

// 유저 팔로우 수 동기화
export async function syncFollows(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/sync/follows');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Follow sync failed:', error);
		throw error;
	}
}

// 포스트 좋아요 수 동기화
export async function syncLikes(): Promise<AdminTaskResponse> {
	try {
		const response = await privateApi.post('v0/admin/sync/likes');
		return await response.json<AdminTaskResponse>();
	} catch (error) {
		console.error('Like sync failed:', error);
		throw error;
	}
}
