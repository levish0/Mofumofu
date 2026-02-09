import { personalSettingsStore } from './personal.svelte';
import type { PersonalInfo } from '$lib/schemas/personal-info';
import type {
	DisplaySettings,
	NotificationSettings,
	PrivacySettings,
	AccountSettings,
	WritingSettings,
	SettingsState
} from './types';

class SettingsStore {
	private state = $state<{
		display: DisplaySettings;
		notifications: NotificationSettings;
		privacy: PrivacySettings;
		account: AccountSettings;
		writing: WritingSettings;
		hasChanges: boolean;
		isLoading: boolean;
		errors: Record<string, string>;
		validationErrors: Record<string, Record<string, string>>;
	}>({
		display: {
			theme: 'system',
			fontSize: 'medium',
			language: 'en'
		},
		notifications: {
			newPosts: true,
			comments: true,
			likes: false,
			newFollowers: true,
			mentions: true,
			weeklyDigest: true,
			securityAlerts: true,
			productUpdates: false,
			marketing: false,
			doNotDisturbEnabled: false,
			doNotDisturbStart: '22:00',
			doNotDisturbEnd: '08:00',
			weekendMode: false
		},
		privacy: {
			profileVisibility: 'public',
			searchableByEmail: true,
			allowDirectMessages: true
		},
		account: {
			email: '',
			password: '',
			twoFactorEnabled: false,
			oauthConnections: [],
			isOAuthOnly: false
		},
		writing: {
			defaultVisibility: 'public',
			autoSave: true,
			spellCheck: true
		},
		hasChanges: false,
		isLoading: false,
		errors: {},
		validationErrors: {}
	});

	private originalState = $state<Partial<SettingsState>>({});

	// Getters - delegate personal to personalSettingsStore
	get personal() {
		return personalSettingsStore.data;
	}

	get originalPersonal() {
		return personalSettingsStore.originalData;
	}

	get display() {
		return this.state.display;
	}

	get notifications() {
		return this.state.notifications;
	}

	get privacy() {
		return this.state.privacy;
	}

	get account() {
		return this.state.account;
	}

	get writing() {
		return this.state.writing;
	}

	get hasChanges() {
		// Check if any section has changes
		return personalSettingsStore.hasChanges || this.state.hasChanges;
	}

	get isLoading() {
		return this.state.isLoading;
	}

	get errors() {
		return { ...this.state.errors, ...personalSettingsStore.currentErrors };
	}

	get validationErrors() {
		return {
			...this.state.validationErrors,
			personal: personalSettingsStore.currentValidationErrors
		};
	}

	// Initialize with current user data
	async loadSettings() {
		this.state.isLoading = true;
		try {
			// TODO: API call to load current settings
			// const response = await api.get('/user/settings');
			// this.state = { ...this.state, ...response.data };

			// Store original state for comparison
			this.originalState = JSON.parse(JSON.stringify(this.state));
			this.state.hasChanges = false;
		} catch (error) {
			console.error('Failed to load settings:', error);
		} finally {
			this.state.isLoading = false;
		}
	}

	// Initialize settings with server data
	initializeWithDefaults() {
		this.state.isLoading = false;
		// State is already initialized with defaults, just mark as ready
		this.originalState = JSON.parse(JSON.stringify(this.state));

		// Initialize personal settings store
		personalSettingsStore.initializeWithDefaults();
	}

	initializeSettings(serverSettings: Partial<SettingsState>) {
		this.state.isLoading = true;
		try {
			// Update state with server data, keeping existing structure
			if (serverSettings.display) {
				this.state.display = { ...this.state.display, ...serverSettings.display };
			}
			if (serverSettings.notifications) {
				this.state.notifications = { ...this.state.notifications, ...serverSettings.notifications };
			}
			if (serverSettings.privacy) {
				this.state.privacy = { ...this.state.privacy, ...serverSettings.privacy };
			}
			if (serverSettings.account) {
				this.state.account = { ...this.state.account, ...serverSettings.account };
			}
			if (serverSettings.writing) {
				this.state.writing = { ...this.state.writing, ...serverSettings.writing };
			}

			// Store original state for comparison
			this.originalState = JSON.parse(JSON.stringify(this.state));
			this.state.hasChanges = false;
			this.state.errors = {};
			this.state.validationErrors = {};
		} catch (error) {
			console.error('Failed to initialize settings:', error);
		} finally {
			this.state.isLoading = false;
		}
	}

	// Update specific section - delegate personal to personalSettingsStore
	updatePersonal(updates: Partial<PersonalInfo>) {
		personalSettingsStore.update(updates);
		this.checkForChanges();
	}

	// Update personal info without triggering change detection (for initial load)
	updatePersonalSilent(updates: Partial<PersonalInfo>) {
		personalSettingsStore.updateSilent(updates);
	}

	updateDisplay(updates: Partial<DisplaySettings>) {
		this.state.display = { ...this.state.display, ...updates };
		this.checkForChanges();
	}

	updateNotifications(updates: Partial<NotificationSettings>) {
		this.state.notifications = { ...this.state.notifications, ...updates };
		this.checkForChanges();
	}

	updatePrivacy(updates: Partial<PrivacySettings>) {
		this.state.privacy = { ...this.state.privacy, ...updates };
		this.checkForChanges();
	}

	updateAccount(updates: Partial<AccountSettings>) {
		this.state.account = { ...this.state.account, ...updates };
		this.checkForChanges();
	}

	// Update account info without triggering change detection (for initial load)
	updateAccountSilent(updates: Partial<AccountSettings>) {
		this.state.account = { ...this.state.account, ...updates };
		// originalState도 함께 업데이트하여 변경사항으로 감지되지 않도록 함
		if (this.originalState.account) {
			this.originalState.account = { ...this.originalState.account, ...updates };
		}
	}

	updateWriting(updates: Partial<WritingSettings>) {
		this.state.writing = { ...this.state.writing, ...updates };
		this.checkForChanges();
	}

	// Check if current state differs from original (excluding personal)
	private checkForChanges() {
		const currentSettings = {
			display: this.state.display,
			notifications: this.state.notifications,
			privacy: this.state.privacy,
			account: this.state.account,
			writing: this.state.writing
		};

		const originalSettings = {
			display: this.originalState.display,
			notifications: this.originalState.notifications,
			privacy: this.originalState.privacy,
			account: this.originalState.account,
			writing: this.originalState.writing
		};

		this.state.hasChanges = JSON.stringify(currentSettings) !== JSON.stringify(originalSettings);
	}

	// Save all changes
	async saveChanges() {
		this.state.isLoading = true;
		this.state.errors = {};

		try {
			// Save personal info first
			const personalResult = await personalSettingsStore.save(this.state.account.password);
			if (!personalResult.success) {
				return personalResult;
			}

			// Clear password after successful personal update
			if (this.state.account.password && this.state.account.password.trim() !== '') {
				this.state.account.password = '';
			}

			// TODO: API calls to save other sections
			// await api.post('/user/settings/display', this.state.display);
			// await api.post('/user/settings/notifications', this.state.notifications);
			// await api.post('/user/settings/privacy', this.state.privacy);
			// await api.post('/user/settings/account', this.state.account);
			// await api.post('/user/settings/writing', this.state.writing);

			console.log('Settings saved successfully');

			// Update original state to current state
			this.originalState = JSON.parse(JSON.stringify(this.state));
			this.state.hasChanges = false;

			return { success: true };
		} catch (error) {
			console.error('Failed to save settings:', error);
			this.state.errors = { general: 'Failed to save settings. Please try again.' };
			return { success: false, error };
		} finally {
			this.state.isLoading = false;
		}
	}

	// Reset to original state
	resetChanges() {
		// Reset personal settings
		personalSettingsStore.reset();

		// Reset other settings
		if (this.originalState) {
			const originalStateCopy = JSON.parse(JSON.stringify(this.originalState));
			this.state = {
				...this.state,
				...originalStateCopy,
				// Always ensure these states are properly reset
				hasChanges: false,
				isLoading: false,
				errors: {},
				validationErrors: {}
			};
		}
	}

	// Set validation errors
	setError(field: string, message: string) {
		this.state.errors = { ...this.state.errors, [field]: message };
	}

	// Clear specific error
	clearError(field: string) {
		const { [field]: _, ...rest } = this.state.errors;
		this.state.errors = rest;
	}

	// Clear all errors
	clearErrors() {
		this.state.errors = {};
		personalSettingsStore.clearErrors();
	}

	// Set validation errors for a specific section
	setValidationErrors(section: string, errors: Record<string, string>) {
		if (section === 'personal') {
			personalSettingsStore.setValidationErrors(errors);
		} else {
			this.state.validationErrors = {
				...this.state.validationErrors,
				[section]: errors
			};
		}
	}

	// Clear validation errors for a specific section
	clearValidationErrors(section: string) {
		if (section === 'personal') {
			personalSettingsStore.clearValidationErrors();
		} else {
			const { [section]: _, ...rest } = this.state.validationErrors;
			this.state.validationErrors = rest;
		}
	}

	// Check if there are any validation errors
	hasValidationErrors(): boolean {
		return (
			personalSettingsStore.hasValidationErrors() ||
			Object.values(this.state.validationErrors).some((sectionErrors) => Object.keys(sectionErrors).length > 0)
		);
	}

	// Validate all sections before saving
	async validateAll(): Promise<boolean> {
		let isValid = true;

		// Validate personal info
		const personalValid = await personalSettingsStore.validate();
		if (!personalValid) {
			isValid = false;
		}

		// Add other section validations here as needed
		// TODO: Add display, notifications, privacy, account, writing validations

		return isValid;
	}
}

export const settingsStore = new SettingsStore();
