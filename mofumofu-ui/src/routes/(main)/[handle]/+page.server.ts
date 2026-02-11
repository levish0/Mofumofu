import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile } from '$lib/api/user';
import { searchPosts } from '$lib/api/posts';
import { isApiError } from '$lib/api/errors';

export const load: PageServerLoad = async ({ params, locals }) => {
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	let profile;
	try {
		profile = await getUserProfile(handle, locals.api);
	} catch (e) {
		console.error('[profile] Failed to load profile:', handle, e);
		if (isApiError(e) && e.status === 404) {
			throw error(404, 'Profile not found');
		}
		throw error(502, 'Failed to load profile');
	}

	try {
		const postsResult = await searchPosts(
			{ user_id: profile.id, page: 1, page_size: 30, sort_by: 'CreatedAt', sort_order: 'Desc' },
			locals.api
		);

		return {
			profile,
			posts: postsResult.posts,
			totalPages: postsResult.total_pages
		};
	} catch (e) {
		console.error('[profile] Failed to load posts for:', handle, e);
		return {
			profile,
			posts: [],
			totalPages: 0
		};
	}
};
