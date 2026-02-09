import { error } from '@sveltejs/kit';
import { getPostForEdit } from '$lib/api/post/postApi';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const { slug } = params;

	try {
		const post = await getPostForEdit({ slug });

		return {
			post,
			slug
		};
	} catch (err) {
		console.error('Failed to load post for editing:', err);
		error(404, 'Post not found or access denied');
	}
};
