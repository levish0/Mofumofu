import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getPostBySlug } from '$lib/api/posts';
import { isApiError } from '$lib/api/errors';

export const load: PageServerLoad = async ({ params, locals }) => {
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	try {
		const post = await getPostBySlug(handle, params.slug, locals.api);
		return { post };
	} catch (e) {
		console.error('[post] Failed to load post:', handle, params.slug, e);
		if (isApiError(e) && e.status === 404) {
			throw error(404, 'Post not found');
		}
		throw error(502, 'Failed to load post');
	}
};
