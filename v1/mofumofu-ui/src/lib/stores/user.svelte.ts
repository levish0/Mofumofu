import { browser } from '$app/environment';
import { getMyProfile } from '$lib/api/user/userApi';
import type { UserInfoResponse } from '$lib/api/user/types';
import { authStore } from './auth.svelte';

class UserStore {
	private _user = $state<UserInfoResponse | null>(null);
	private _isLoading = $state(false);
	private _error = $state<string | null>(null);
	private _initialized = $state(false);

	get user() {
		return this._user;
	}

	get isLoading() {
		return this._isLoading;
	}

	get error() {
		return this._error;
	}

	get isInitialized() {
		return this._initialized;
	}

	async loadProfile(force = false) {
		if (!browser) return;
		if (!authStore.isAuthenticated) {
			this.clear();
			return;
		}
		if (this._isLoading) return;
		if (this._user && !force) return;

		this._isLoading = true;
		this._error = null;

		try {
			this._user = await getMyProfile();
			this._initialized = true;
		} catch (error) {
			console.error('Failed to load user profile:', error);
			this._error = error instanceof Error ? error.message : 'Failed to load profile';
			this._user = null;
		} finally {
			this._isLoading = false;
		}
	}

	updateUser(updates: Partial<UserInfoResponse>) {
		if (this._user) {
			this._user = { ...this._user, ...updates };
		}
	}

	async refresh() {
		await this.loadProfile(true);
	}

	clear() {
		this._user = null;
		this._error = null;
		this._initialized = false;
	}

	setError(error: string) {
		this._error = error;
	}

	clearError() {
		this._error = null;
	}
}

export const userStore = new UserStore();
