import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getPostBySlug } from '$lib/api/posts';

export const load: PageServerLoad = async ({ params, locals }) => {
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	try {
		const post = await getPostBySlug(handle, params.slug, locals.api);
		return { post };
	} catch {
		throw error(404, 'Post not found');
	}
};
