// src/lib/api/error/common_error.ts

import type { ErrorResponse } from './types';

export const ErrorCodes = {
	// user module errors
	UserInvalidPassword: 'user:invalid_password',
	UserNotVerified: 'user:not_verified',
	UserNotFound: 'user:not_found',
	UserUnauthorized: 'user:unauthorized',
	UserHandleAlreadyExists: 'user:handle_already_exists',
	UserTokenExpired: 'user:token_expired',
	UserNoRefreshToken: 'user:no_refresh_token',
	UserInvalidToken: 'user:invalid_token',

	// post module errors
	PostNotFound: 'post:not_found',

	// follow module errors
	FollowCannotFollowSelf: 'follow:cannot_follow_self',
	FollowAlreadyFollowing: 'follow:already_following',
	FollowNotExist: 'follow:not_exist',

	// oauth module errors
	OauthInvalidAuthUrl: 'oauth:invalid_auth_url',
	OauthInvalidTokenUrl: 'oauth:invalid_token_url',
	OauthInvalidRedirectUrl: 'oauth:invalid_redirect_url',
	OauthTokenExchangeFailed: 'oauth:token_exchange_failed',
	OauthUserInfoFetchFailed: 'oauth:user_info_fetch failed',
	OauthUserInfoParseFailed: 'oauth:user_info_parse failed',
	OauthAccountAlreadyLinked: 'oauth:account_already_linked',
	OauthConnectionNotFound: 'oauth:connection_not_found',
	OauthCannotUnlinkLastConnection: 'oauth:cannot_unlink_last_connection',
	OauthInvalidImageUrl: 'oauth:invalid_image_url',

	// password module errors
	PasswordRequiredForUpdate: 'password:required_for_update',
	PasswordIncorrect: 'password:incorrect',
	PasswordCannotUpdateOauthOnly: 'password:cannot_update_oauth_only',
	PasswordNewPasswordMissing: 'password:new_password_missing',
	PasswordAlreadySet: 'password:already_set',

	// token module errors
	TokenInvalidVerification: 'token:invalid_verification',
	TokenExpiredVerification: 'token:expired_verification',
	TokenEmailMismatch: 'token:email_mismatch',
	TokenInvalidReset: 'token:invalid_reset',
	TokenExpiredReset: 'token:expired_reset',

	// email module errors
	EmailAlreadyVerified: 'email:already_verified',

	// file module errors
	FileUploadError: 'file:upload_error',
	FileNotFound: 'file:not_found',
	FileReadError: 'file:read_error',

	// like module errors
	LikeAlreadyExists: 'like:already_exists',
	LikeNotFound: 'like:not_found',

	// markdown module errors
	MarkdownRenderFailed: 'markdown:render_failed',

	// general module errors
	BadRequest: 'general:bad_request',
	ValidationError: 'general:validation_error',

	// system module errors
	SysInternalError: 'system:internal_error',
	SysHashingError: 'system:hashing_error',
	SysNotFound: 'system:not_found',
	SysTransactionError: 'system:transaction_error',
	SysDatabaseError: 'system:database_error',
	SysTokenCreationError: 'system:token_creation_error'
};

export class ApiError extends Error {
	status: number;
	body: ErrorResponse | null;

	constructor(code: string, status: number, body: ErrorResponse | null) {
		super(code);
		this.status = status;
		this.body = body;
	}
}

// User module error classes
export class UserInvalidPassword extends ApiError {}
export class UserNotVerified extends ApiError {}
export class UserNotFound extends ApiError {}
export class UserUnauthorized extends ApiError {}
export class UserHandleAlreadyExists extends ApiError {}
export class UserTokenExpired extends ApiError {}
export class UserNoRefreshToken extends ApiError {}
export class UserInvalidToken extends ApiError {}

// Post module error classes
export class PostNotFound extends ApiError {}

// Follow module error classes
export class FollowCannotFollowSelf extends ApiError {}
export class FollowAlreadyFollowing extends ApiError {}
export class FollowNotExist extends ApiError {}

// OAuth module error classes
export class OauthInvalidAuthUrl extends ApiError {}
export class OauthInvalidTokenUrl extends ApiError {}
export class OauthInvalidRedirectUrl extends ApiError {}
export class OauthTokenExchangeFailed extends ApiError {}
export class OauthUserInfoFetchFailed extends ApiError {}
export class OauthUserInfoParseFailed extends ApiError {}
export class OauthAccountAlreadyLinked extends ApiError {}
export class OauthConnectionNotFound extends ApiError {}
export class OauthCannotUnlinkLastConnection extends ApiError {}
export class OauthInvalidImageUrl extends ApiError {}

// Password module error classes
export class PasswordRequiredForUpdate extends ApiError {}
export class PasswordIncorrect extends ApiError {}
export class PasswordCannotUpdateOauthOnly extends ApiError {}
export class PasswordNewPasswordMissing extends ApiError {}
export class PasswordAlreadySet extends ApiError {}

// Token module error classes
export class TokenInvalidVerification extends ApiError {}
export class TokenExpiredVerification extends ApiError {}
export class TokenEmailMismatch extends ApiError {}
export class TokenInvalidReset extends ApiError {}
export class TokenExpiredReset extends ApiError {}

// Email module error classes
export class EmailAlreadyVerified extends ApiError {}

// File module error classes
export class FileUploadError extends ApiError {}
export class FileNotFound extends ApiError {}
export class FileReadError extends ApiError {}

// Like module error classes
export class LikeAlreadyExists extends ApiError {}
export class LikeNotFound extends ApiError {}

// Markdown module error classes
export class MarkdownRenderFailed extends ApiError {}

// General module error classes
export class BadRequestError extends ApiError {}
export class ValidationError extends ApiError {}

// System module error classes
export class SysInternalError extends ApiError {}
export class SysHashingError extends ApiError {}
export class SysNotFound extends ApiError {}
export class SysTransactionError extends ApiError {}
export class SysDatabaseError extends ApiError {}
export class SysTokenCreationError extends ApiError {}
