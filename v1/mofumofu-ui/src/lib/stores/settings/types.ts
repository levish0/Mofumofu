export type PersonalInfo = {
	handle: string;
	name: string;
	bio: string;
	location: string;
	website: string;
	profileImage: string | null;
	bannerImage: string | null;
	profileImageFile: Blob | null;
	bannerImageFile: Blob | null;
};

export type DisplaySettings = {
	theme: 'light' | 'dark' | 'system';
	fontSize: 'small' | 'medium' | 'large';
	language: string;
};

export type NotificationSettings = {
	// Push notifications
	newPosts: boolean;
	comments: boolean;
	likes: boolean;
	newFollowers: boolean;
	mentions: boolean;
	// Email notifications
	weeklyDigest: boolean;
	securityAlerts: boolean;
	productUpdates: boolean;
	marketing: boolean;
	// Do not disturb
	doNotDisturbEnabled: boolean;
	doNotDisturbStart: string;
	doNotDisturbEnd: string;
	weekendMode: boolean;
};

export type PrivacySettings = {
	profileVisibility: 'public' | 'private';
	searchableByEmail: boolean;
	allowDirectMessages: boolean;
};

export type AccountSettings = {
	email: string;
	password: string;
	twoFactorEnabled: boolean;
	oauthConnections: string[];
	isOAuthOnly: boolean;
};

export type WritingSettings = {
	defaultVisibility: 'public' | 'unlisted' | 'private';
	autoSave: boolean;
	spellCheck: boolean;
};

export type SettingsState = {
	personal: PersonalInfo;
	display: DisplaySettings;
	notifications: NotificationSettings;
	privacy: PrivacySettings;
	account: AccountSettings;
	writing: WritingSettings;
	hasChanges: boolean;
	isLoading: boolean;
	errors: Record<string, string>;
	validationErrors: Record<string, Record<string, string>>;
};
