import type { components } from './generated';

// Common schema aliases
export type UserResponse = components['schemas']['UserResponse'];
export type PublicUserProfile = components['schemas']['PublicUserProfile'];
export type CreateUserRequest = components['schemas']['CreateUserRequest'];
export type CreateUserResponse = components['schemas']['CreateUserResponse'];
export type LoginRequest = components['schemas']['LoginRequest'];
export type TotpRequiredResponse = components['schemas']['TotpRequiredResponse'];
export type TotpVerifyRequest = components['schemas']['TotpVerifyRequest'];
export type TotpSetupResponse = components['schemas']['TotpSetupResponse'];
export type TotpEnableRequest = components['schemas']['TotpEnableRequest'];
export type TotpEnableResponse = components['schemas']['TotpEnableResponse'];
export type TotpDisableRequest = components['schemas']['TotpDisableRequest'];
export type TotpStatusResponse = components['schemas']['TotpStatusResponse'];
export type TotpRegenerateBackupCodesRequest =
	components['schemas']['TotpRegenerateBackupCodesRequest'];
export type TotpBackupCodesResponse = components['schemas']['TotpBackupCodesResponse'];
export type VerifyEmailRequest = components['schemas']['VerifyEmailRequest'];
export type ForgotPasswordRequest = components['schemas']['ForgotPasswordRequest'];
export type ResetPasswordRequest = components['schemas']['ResetPasswordRequest'];
export type ChangePasswordRequest = components['schemas']['ChangePasswordRequest'];
export type ChangeEmailRequest = components['schemas']['ChangeEmailRequest'];
export type ConfirmEmailChangeRequest = components['schemas']['ConfirmEmailChangeRequest'];
export type UpdateMyProfileRequest = components['schemas']['UpdateMyProfileRequest'];
export type CheckHandleAvailableResponse = components['schemas']['CheckHandleAvailableResponse'];
export type SearchUsersResponse = components['schemas']['SearchUsersResponse'];
export type UploadUserImageResponse = components['schemas']['UploadUserImageResponse'];
export type OAuthUrlResponse = components['schemas']['OAuthUrlResponse'];
export type OAuthPendingSignupResponse = components['schemas']['OAuthPendingSignupResponse'];
export type OAuthProvider = components['schemas']['OAuthProvider'];
export type OAuthConnectionListResponse = components['schemas']['OAuthConnectionListResponse'];
export type CompleteSignupRequest = components['schemas']['CompleteSignupRequest'];
export type GoogleLoginRequest = components['schemas']['GoogleLoginRequest'];
export type GithubLoginRequest = components['schemas']['GithubLoginRequest'];
export type GoogleLinkRequest = components['schemas']['GoogleLinkRequest'];
export type GithubLinkRequest = components['schemas']['GithubLinkRequest'];
export type UnlinkOAuthRequest = components['schemas']['UnlinkOAuthRequest'];

// Discriminated unions for multi-status responses
export type LoginResult = { kind: 'success' } | { kind: 'totp_required'; temp_token: string };

export type OAuthLoginResult =
	| { kind: 'existing_user' }
	| { kind: 'new_user'; data: OAuthPendingSignupResponse };
