// src/lib/api/error/error_class_map.ts

import type { ErrorResponse } from './types';
import {
	ApiError,
	ErrorCodes,
	// User module
	UserInvalidPassword,
	UserNotVerified,
	UserNotFound,
	UserUnauthorized,
	UserHandleAlreadyExists,
	UserTokenExpired,
	UserNoRefreshToken,
	UserInvalidToken,
	// Post module
	PostNotFound,
	// Follow module
	FollowCannotFollowSelf,
	FollowAlreadyFollowing,
	FollowNotExist,
	// OAuth module
	OauthInvalidAuthUrl,
	OauthInvalidTokenUrl,
	OauthInvalidRedirectUrl,
	OauthTokenExchangeFailed,
	OauthUserInfoFetchFailed,
	OauthUserInfoParseFailed,
	OauthAccountAlreadyLinked,
	OauthConnectionNotFound,
	OauthCannotUnlinkLastConnection,
	OauthInvalidImageUrl,
	// Password module
	PasswordRequiredForUpdate,
	PasswordIncorrect,
	PasswordCannotUpdateOauthOnly,
	PasswordNewPasswordMissing,
	PasswordAlreadySet,
	// Token module
	TokenInvalidVerification,
	TokenExpiredVerification,
	TokenEmailMismatch,
	TokenInvalidReset,
	TokenExpiredReset,
	// Email module
	EmailAlreadyVerified,
	// File module
	FileUploadError,
	FileNotFound,
	FileReadError,
	// Like module
	LikeAlreadyExists,
	LikeNotFound,
	// Markdown module
	MarkdownRenderFailed,
	// General module
	BadRequestError,
	ValidationError,
	// System module
	SysInternalError,
	SysHashingError,
	SysNotFound,
	SysTransactionError,
	SysDatabaseError,
	SysTokenCreationError
} from './common_error';

export const ErrorClassMap: Record<string, new (code: string, status: number, body: ErrorResponse | null) => ApiError> =
	{
		// User module
		[ErrorCodes.UserInvalidPassword]: UserInvalidPassword,
		[ErrorCodes.UserNotVerified]: UserNotVerified,
		[ErrorCodes.UserNotFound]: UserNotFound,
		[ErrorCodes.UserUnauthorized]: UserUnauthorized,
		[ErrorCodes.UserHandleAlreadyExists]: UserHandleAlreadyExists,
		[ErrorCodes.UserTokenExpired]: UserTokenExpired,
		[ErrorCodes.UserNoRefreshToken]: UserNoRefreshToken,
		[ErrorCodes.UserInvalidToken]: UserInvalidToken,

		// Post module
		[ErrorCodes.PostNotFound]: PostNotFound,

		// Follow module
		[ErrorCodes.FollowCannotFollowSelf]: FollowCannotFollowSelf,
		[ErrorCodes.FollowAlreadyFollowing]: FollowAlreadyFollowing,
		[ErrorCodes.FollowNotExist]: FollowNotExist,

		// OAuth module
		[ErrorCodes.OauthInvalidAuthUrl]: OauthInvalidAuthUrl,
		[ErrorCodes.OauthInvalidTokenUrl]: OauthInvalidTokenUrl,
		[ErrorCodes.OauthInvalidRedirectUrl]: OauthInvalidRedirectUrl,
		[ErrorCodes.OauthTokenExchangeFailed]: OauthTokenExchangeFailed,
		[ErrorCodes.OauthUserInfoFetchFailed]: OauthUserInfoFetchFailed,
		[ErrorCodes.OauthUserInfoParseFailed]: OauthUserInfoParseFailed,
		[ErrorCodes.OauthAccountAlreadyLinked]: OauthAccountAlreadyLinked,
		[ErrorCodes.OauthConnectionNotFound]: OauthConnectionNotFound,
		[ErrorCodes.OauthCannotUnlinkLastConnection]: OauthCannotUnlinkLastConnection,
		[ErrorCodes.OauthInvalidImageUrl]: OauthInvalidImageUrl,

		// Password module
		[ErrorCodes.PasswordRequiredForUpdate]: PasswordRequiredForUpdate,
		[ErrorCodes.PasswordIncorrect]: PasswordIncorrect,
		[ErrorCodes.PasswordCannotUpdateOauthOnly]: PasswordCannotUpdateOauthOnly,
		[ErrorCodes.PasswordNewPasswordMissing]: PasswordNewPasswordMissing,
		[ErrorCodes.PasswordAlreadySet]: PasswordAlreadySet,

		// Token module
		[ErrorCodes.TokenInvalidVerification]: TokenInvalidVerification,
		[ErrorCodes.TokenExpiredVerification]: TokenExpiredVerification,
		[ErrorCodes.TokenEmailMismatch]: TokenEmailMismatch,
		[ErrorCodes.TokenInvalidReset]: TokenInvalidReset,
		[ErrorCodes.TokenExpiredReset]: TokenExpiredReset,

		// Email module
		[ErrorCodes.EmailAlreadyVerified]: EmailAlreadyVerified,

		// File module
		[ErrorCodes.FileUploadError]: FileUploadError,
		[ErrorCodes.FileNotFound]: FileNotFound,
		[ErrorCodes.FileReadError]: FileReadError,

		// Like module
		[ErrorCodes.LikeAlreadyExists]: LikeAlreadyExists,
		[ErrorCodes.LikeNotFound]: LikeNotFound,

		// Markdown module
		[ErrorCodes.MarkdownRenderFailed]: MarkdownRenderFailed,

		// General module
		[ErrorCodes.BadRequest]: BadRequestError,
		[ErrorCodes.ValidationError]: ValidationError,

		// System module
		[ErrorCodes.SysInternalError]: SysInternalError,
		[ErrorCodes.SysHashingError]: SysHashingError,
		[ErrorCodes.SysNotFound]: SysNotFound,
		[ErrorCodes.SysTransactionError]: SysTransactionError,
		[ErrorCodes.SysDatabaseError]: SysDatabaseError,
		[ErrorCodes.SysTokenCreationError]: SysTokenCreationError
	};
