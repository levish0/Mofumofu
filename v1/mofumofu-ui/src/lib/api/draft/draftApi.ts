import { privateApi } from '../private';
import type {
	CreateDraftRequest,
	UpdateDraftRequest,
	GetDraftRequest,
	DeleteDraftRequest,
	DraftInfo,
	CreateDraftResponse,
	GetDraftsResponse
} from './types';

/**
 * Create a new draft
 */
export async function createDraft(data: CreateDraftRequest): Promise<CreateDraftResponse> {
	const response = await privateApi.post('v0/draft', { json: data }).json<CreateDraftResponse>();
	return response;
}

/**
 * Update an existing draft
 */
export async function updateDraft(data: UpdateDraftRequest): Promise<void> {
	await privateApi.post('v0/draft/update', { json: data });
}

/**
 * Get a specific draft by ID
 */
export async function getDraft(data: GetDraftRequest): Promise<DraftInfo> {
	const response = await privateApi.post('v0/draft/get', { json: data }).json<DraftInfo>();
	return response;
}

/**
 * Delete a draft
 */
export async function deleteDraft(data: DeleteDraftRequest): Promise<void> {
	await privateApi.post('v0/draft/delete', { json: data });
}

/**
 * Get all drafts for the authenticated user
 */
export async function getDrafts(): Promise<GetDraftsResponse> {
	const response = await privateApi.post('v0/drafts').json<GetDraftsResponse>();
	return response;
}

/**
 * Generate slug with priority: slug > title > timestamp
 */
export function generateDraftSlug(title?: string, slug?: string): string {
	// 1. If slug exists, use it
	if (slug && slug.trim()) {
		return slug.trim();
	}

	// 2. If title exists, generate from title
	if (title && title.trim()) {
		return generateSlugFromTitle(title.trim());
	}

	// 3. Fallback to timestamp
	return `draft-${Date.now()}`;
}

/**
 * Generate a slug from title
 */
function generateSlugFromTitle(title: string): string {
	return (
		title
			.toLowerCase()
			.trim()
			.replace(/[^a-z0-9가-힣\s-]/g, '') // Remove special chars except Korean
			.replace(/\s+/g, '-') // Replace spaces with dashes
			.replace(/-+/g, '-') // Replace multiple dashes with single
			.replace(/^-|-$/g, '') || // Remove leading/trailing dashes
		`untitled-${Date.now()}`
	); // Fallback if result is empty
}
