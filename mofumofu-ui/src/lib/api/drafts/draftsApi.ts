import type { KyInstance } from 'ky';
import { Api } from '../api';
import type {
	DraftResponse,
	DraftListResponse,
	UpdateDraftRequest,
	PublishDraftRequest,
	PostResponse
} from '../types';

export async function getDrafts(api: KyInstance = Api): Promise<DraftListResponse> {
	return api.get('v0/drafts').json<DraftListResponse>();
}

export async function createDraft(api: KyInstance = Api): Promise<DraftResponse> {
	return api.post('v0/drafts').json<DraftResponse>();
}

export async function getDraft(
	draftId: string,
	api: KyInstance = Api
): Promise<DraftResponse> {
	return api.get(`v0/drafts/${encodeURIComponent(draftId)}`).json<DraftResponse>();
}

export async function updateDraft(
	draftId: string,
	data: UpdateDraftRequest,
	api: KyInstance = Api
): Promise<DraftResponse> {
	return api
		.patch(`v0/drafts/${encodeURIComponent(draftId)}`, { json: data })
		.json<DraftResponse>();
}

export async function deleteDraft(
	draftId: string,
	api: KyInstance = Api
): Promise<void> {
	await api.delete(`v0/drafts/${encodeURIComponent(draftId)}`);
}

export async function publishDraft(
	draftId: string,
	data: PublishDraftRequest,
	api: KyInstance = Api
): Promise<PostResponse> {
	return api
		.post(`v0/drafts/${encodeURIComponent(draftId)}/publish`, { json: data })
		.json<PostResponse>();
}
