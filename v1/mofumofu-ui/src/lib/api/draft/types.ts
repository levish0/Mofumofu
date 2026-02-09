// Draft API types based on swagger schema

export interface DraftInfo {
	draft_id: string;
	slug: string;
	title?: string | null;
	content?: string | null;
	summary?: string | null;
	thumbnail_image?: string | null;
	created_at: string;
	updated_at?: string | null;
}

export interface CreateDraftResponse {
	draft_id: string;
}

export interface CreateDraftRequest {
	slug: string;
	title?: string | null;
	content?: string | null;
	summary?: string | null;
	thumbnail_image?: string | null;
}

export interface UpdateDraftRequest {
	draft_id: string;
	title?: string | null;
	content?: string | null;
	summary?: string | null;
	thumbnail_image?: string | null;
}

export interface GetDraftRequest {
	draft_id: string;
}

export interface DeleteDraftRequest {
	draft_id: string;
}

export interface GetDraftsResponse {
	drafts: DraftInfo[];
}
