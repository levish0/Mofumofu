import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile } from '$lib/api/user';
import { searchPosts } from '$lib/api/posts';

export const load: PageServerLoad = async ({ params, locals }) => {
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	try {
		const profile = await getUserProfile(handle, locals.api);
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
		console.error('[profile] Failed to load profile:', handle, e);
		throw error(404, 'Profile not found');
	}
};
