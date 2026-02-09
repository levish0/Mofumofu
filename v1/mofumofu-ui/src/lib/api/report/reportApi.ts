import { privateApi } from '../private';
import { publicApi } from '../public';
import { authStore } from '../../stores/auth.svelte';
import type {
	CreateReportRequest,
	CreateReportResponse,
	GetReportsRequest,
	GetReportsResponse,
	ProcessReportRequest
} from './types';

/**
 * Create a new report
 * Uses privateApi if user is authenticated (to record reporter ID),
 * otherwise uses publicApi for anonymous reports
 */
export async function createReport(request: CreateReportRequest): Promise<CreateReportResponse> {
	try {
		const api = authStore.isAuthenticated ? privateApi : publicApi;
		const response = await api.post('v0/report', { json: request });
		return response.json<CreateReportResponse>();
	} catch (error) {
		console.error('Failed to create report:', error);
		throw error;
	}
}

/**
 * Get reports (admin only)
 */
export async function getReports(request: GetReportsRequest = {}): Promise<GetReportsResponse> {
	try {
		const response = await privateApi.post('v0/report/list', { json: request });
		return response.json<GetReportsResponse>();
	} catch (error) {
		console.error('Failed to get reports:', error);
		throw error;
	}
}

/**
 * Process a report (admin only)
 */
export async function processReport(request: ProcessReportRequest): Promise<void> {
	try {
		await privateApi.post('v0/report/process', { json: request });
	} catch (error) {
		console.error('Failed to process report:', error);
		throw error;
	}
}
