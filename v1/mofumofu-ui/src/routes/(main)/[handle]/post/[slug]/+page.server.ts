import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getPostByHandleAndSlug } from '$lib/api/post/postApi';

export const load: PageServerLoad = async ({ params }) => {
	if (!params.handle || !params.slug) {
		throw error(404, 'Post not found');
	}

	// Remove @ prefix if present
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	try {
		// Call the actual API (now returns rendered HTML and TOC from backend)
		const postData = await getPostByHandleAndSlug({
			handle,
			slug: params.slug
		});

		return {
			post: postData,
			author: postData.author,
			handle,
			slug: params.slug
		};
	} catch (err) {
		console.error('Failed to load post:', err);
		throw error(404, 'Post not found');
	}
};
