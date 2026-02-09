export interface UserInfoResponse {
	handle: string;
	name: string;
	email: string;
	is_verified: boolean;
	bio?: string | null;
	location?: string | null;
	website?: string | null;
	profile_image?: string | null;
	banner_image?: string | null;
	created_at: string;
}

export interface HandleCheckResponse {
	is_available: boolean;
}

export interface CreateUserRequest {
	email: string;
	handle: string;
	name: string;
	password: string;
}

export interface GetUserProfileRequest {
	handle: string;
}

export interface UpdateProfileRequest {
	handle?: string | null;
	name?: string | null;
	bio?: string | null;
	location?: string | null;
	website?: string | null;
	password?: string | null;
	current_password?: string | null;
}

export interface ImageUploadResponse {
	public_url: string;
}
