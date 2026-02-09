import {
	updateProfile,
	uploadAvatar,
	uploadBanner,
	getMyProfile,
	checkHandleAvailability
} from '$lib/api/user/userApi';
import type { UpdateProfileRequest } from '$lib/api/user/types';
import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
import { safeParse } from 'valibot';
import type { PersonalInfo } from './types';
import { userStore } from '../user.svelte';

export class PersonalSettingsStore {
	private state = $state<PersonalInfo>({
		handle: '',
		name: '',
		bio: '',
		location: '',
		website: '',
		profileImage: null,
		bannerImage: null,
		profileImageFile: null,
		bannerImageFile: null
	});

	private originalState = $state<PersonalInfo>({
		handle: '',
		name: '',
		bio: '',
		location: '',
		website: '',
		profileImage: null,
		bannerImage: null,
		profileImageFile: null,
		bannerImageFile: null
	});

	private errors = $state<Record<string, string>>({});
	private validationErrors = $state<Record<string, string>>({});
	private handleVerificationState = $state<'verified' | 'unverified' | 'checking' | 'unavailable'>('verified');

	// Getters
	get data() {
		return this.state;
	}

	get handle() {
		return this.state.handle;
	}

	get name() {
		return this.state.name;
	}

	get bio() {
		return this.state.bio;
	}

	get location() {
		return this.state.location;
	}

	get website() {
		return this.state.website;
	}

	get profileImage() {
		return this.state.profileImage;
	}

	get bannerImage() {
		return this.state.bannerImage;
	}

	get profileImageFile() {
		return this.state.profileImageFile;
	}

	get bannerImageFile() {
		return this.state.bannerImageFile;
	}

	get hasChanges() {
		const current = JSON.stringify({
			handle: this.state.handle,
			name: this.state.name,
			bio: this.state.bio,
			location: this.state.location,
			website: this.state.website,
			profileImage: this.state.profileImage,
			bannerImage: this.state.bannerImage
		});
		const original = JSON.stringify({
			handle: this.originalState.handle,
			name: this.originalState.name,
			bio: this.originalState.bio,
			location: this.originalState.location,
			website: this.originalState.website,
			profileImage: this.originalState.profileImage,
			bannerImage: this.originalState.bannerImage
		});
		return current !== original || this.state.profileImageFile !== null || this.state.bannerImageFile !== null;
	}

	get currentErrors() {
		return this.errors;
	}

	get currentValidationErrors() {
		return this.validationErrors;
	}

	get originalData() {
		return this.originalState;
	}

	get handleNeedsVerification() {
		return (
			this.state.handle !== this.originalState.handle &&
			this.state.handle.trim() !== '' &&
			this.handleVerificationState === 'unverified'
		);
	}

	get handleVerificationStatus() {
		return this.handleVerificationState;
	}

	// Initialize with server data
	initialize(data: Partial<PersonalInfo>) {
		this.state = { ...this.state, ...data };
		this.originalState = JSON.parse(JSON.stringify(this.state));
	}

	// Initialize with defaults (called from main settings store)
	initializeWithDefaults() {
		this.originalState = JSON.parse(JSON.stringify(this.state));
	}

	// Update without triggering change detection (for initial load)
	updateSilent(updates: Partial<PersonalInfo>) {
		this.state = { ...this.state, ...updates };
		// Update original state too so it doesn't count as a change
		this.originalState = { ...this.originalState, ...updates };
	}

	// Update with change detection
	update(updates: Partial<PersonalInfo>) {
		const oldHandle = this.state.handle;
		this.state = { ...this.state, ...updates };

		// 핸들이 변경되었으면 검증 상태 리셋
		if (updates.handle !== undefined && updates.handle !== oldHandle) {
			const isBackToOriginal = updates.handle === this.originalState.handle;
			this.handleVerificationState = isBackToOriginal ? 'verified' : 'unverified';
		}
	}

	// Check handle availability
	async checkHandleAvailability(): Promise<void> {
		if (!this.state.handle || this.state.handle.trim() === '') return;

		this.handleVerificationState = 'checking';
		try {
			const result = await checkHandleAvailability(this.state.handle.trim());
			this.handleVerificationState = result.is_available ? 'verified' : 'unavailable';
		} catch (error) {
			console.error('Handle availability check failed:', error);
			this.handleVerificationState = 'unverified';
		}
	}

	// Validate personal info (including handle verification)
	async validate(): Promise<boolean> {
		const result = safeParse(createPersonalInfoSchema(), this.state);
		const errors: Record<string, string> = {};

		if (!result.success) {
			for (const issue of result.issues) {
				if (issue.path && issue.path.length > 0) {
					const field = issue.path[0].key as string;
					errors[field] = issue.message;
				}
			}
		}

		// 핸들 검증 확인 (메시지는 컴포넌트에서 처리하므로 키만 설정)
		if (this.handleNeedsVerification) {
			errors.handle = 'HANDLE_VERIFICATION_REQUIRED';
		} else if (this.handleVerificationStatus === 'unavailable') {
			errors.handle = 'HANDLE_UNAVAILABLE';
		}

		this.validationErrors = errors;
		return Object.keys(errors).length === 0;
	}

	// Check if personal info has changes including files
	private hasPersonalChanges(): boolean {
		const current = this.state;
		const original = this.originalState;

		return (
			current.handle !== original.handle ||
			current.name !== original.name ||
			current.bio !== original.bio ||
			current.location !== original.location ||
			current.website !== original.website ||
			current.profileImage !== original.profileImage ||
			current.bannerImage !== original.bannerImage ||
			current.profileImageFile !== null ||
			current.bannerImageFile !== null
		);
	}

	// Save changes
	async save(accountPassword?: string): Promise<{ success: boolean; error?: string }> {
		try {
			// Validate before saving
			const isValid = await this.validate();
			if (!isValid) {
				this.errors = { general: 'Please fix validation errors before saving.' };
				return { success: false, error: 'Validation failed' };
			}

			if (this.hasPersonalChanges() || (accountPassword && accountPassword.trim() !== '')) {
				// Upload profile image if file exists and use the returned URL
				if (this.state.profileImageFile) {
					try {
						const profileUploadResult = await uploadAvatar(this.state.profileImageFile as File);
						// Update the profileImage with the server URL immediately
						this.state.profileImage = profileUploadResult.public_url;
					} catch (error) {
						console.error('Profile image upload failed:', error);
						this.errors = { general: 'Failed to upload profile image.' };
						return { success: false, error: 'Profile image upload failed' };
					}
				}

				// Upload banner image if file exists and use the returned URL
				if (this.state.bannerImageFile) {
					try {
						const bannerUploadResult = await uploadBanner(this.state.bannerImageFile as File);
						// Update the bannerImage with the server URL immediately
						this.state.bannerImage = bannerUploadResult.public_url;
					} catch (error) {
						console.error('Banner image upload failed:', error);
						this.errors = { general: 'Failed to upload banner image.' };
						return { success: false, error: 'Banner image upload failed' };
					}
				}

				// Update basic profile info (handle, name, bio, location, website, password)
				const personalData: UpdateProfileRequest = {
					handle: this.state.handle || null,
					name: this.state.name || null,
					bio: this.state.bio || null,
					location: this.state.location || null,
					website: this.state.website || null
				};

				// Add password if provided
				if (accountPassword && accountPassword.trim() !== '') {
					personalData.password = accountPassword;
				}

				const updatedProfile = await updateProfile(personalData);
				console.log('updateProfile response:', updatedProfile);

				// Update the personal info with the response from updateProfile
				this.state = {
					...this.state,
					handle: updatedProfile.handle,
					name: updatedProfile.name,
					bio: updatedProfile.bio || '',
					location: updatedProfile.location || '',
					website: updatedProfile.website || '',
					// Use server URLs from the response (images already updated from upload)
					profileImage: this.state.profileImage || updatedProfile.profile_image || null,
					bannerImage: this.state.bannerImage || updatedProfile.banner_image || null,
					profileImageFile: null, // Clear file after successful upload
					bannerImageFile: null // Clear file after successful upload
				};

				// Update original state
				this.originalState = JSON.parse(JSON.stringify(this.state));

				// Update user store with the latest profile data
				console.log('Updating userStore with:', updatedProfile);
				userStore.updateUser(updatedProfile);
				console.log('userStore after update:', userStore.user);
				this.errors = {};
			}

			return { success: true };
		} catch (error) {
			console.error('Failed to save personal settings:', error);
			this.errors = { general: 'Failed to save personal settings. Please try again.' };
			return { success: false, error: error instanceof Error ? error.message : 'Unknown error' };
		}
	}

	// Reset to original state
	reset() {
		this.state = JSON.parse(JSON.stringify(this.originalState));
		this.errors = {};
		this.validationErrors = {};
	}

	// Error management
	setError(field: string, message: string) {
		this.errors = { ...this.errors, [field]: message };
	}

	clearError(field: string) {
		const { [field]: _, ...rest } = this.errors;
		this.errors = rest;
	}

	clearErrors() {
		this.errors = {};
	}

	setValidationErrors(errors: Record<string, string>) {
		this.validationErrors = errors;
	}

	clearValidationErrors() {
		this.validationErrors = {};
	}

	hasValidationErrors(): boolean {
		return Object.keys(this.validationErrors).length > 0;
	}
}

export const personalSettingsStore = new PersonalSettingsStore();
