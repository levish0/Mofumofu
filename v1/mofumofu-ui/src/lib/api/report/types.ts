export type ReportReason = 'Spam' | 'Harassment' | 'InappropriateContent' | 'Copyright' | 'Other';

export type ReportTargetType = 'User' | 'Post' | 'Comment';

export type ReportStatus = 'Pending' | 'Reviewing' | 'Resolved' | 'Dismissed';

export interface CreateReportRequest {
	target_id: string;
	reasons: ReportReason[];
	target_type: ReportTargetType;
	description?: string;
}

export interface CreateReportResponse {
	report_id: string;
}

export interface GetReportsRequest {
	page?: number;
	per_page?: number;
	status?: ReportStatus | null;
}

export interface ReportInfo {
	id: string;
	target_id: string;
	reasons: ReportReason[];
	reporter_id: string | null;
	status: ReportStatus;
	target_type: ReportTargetType;
	description?: string;
	created_at: string;
	updated_at: string;
}

export interface GetReportsResponse {
	reports: ReportInfo[];
	total: number;
	page: number;
	per_page: number;
}

export interface ProcessReportRequest {
	report_id: string;
	status: ReportStatus;
}
