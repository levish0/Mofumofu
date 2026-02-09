import ky from 'ky';
import { API_URL } from './config';

export const publicApi = ky.create({
	prefixUrl: API_URL,
	credentials: 'include',
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	},
	timeout: 10 * 1000,
	retry: 2
});
