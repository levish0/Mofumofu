import { browser } from '$app/environment';

let _handle = $state(browser ? (sessionStorage.getItem('oauth_handle') ?? '') : '');

export const oauthHandleStore = {
	get currentHandle() {
		return _handle;
	},

	setHandle(handle: string) {
		_handle = handle;
		if (browser) {
			sessionStorage.setItem('oauth_handle', handle);
		}
	},

	clearHandle() {
		_handle = '';
		if (browser) {
			sessionStorage.removeItem('oauth_handle');
		}
	}
};
