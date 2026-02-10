import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { getMe } from '$lib/api/auth';
import { getPost } from '$lib/api/posts';

export const load: PageServerLoad = async ({ params, locals }) => {
	const user = await getMe(locals.api);
	if (!user) {
		throw redirect(302, '/account/signin');
	}

	try {
		const post = await getPost(params.postId, locals.api);

		if (post.user_id !== user.id) {
			throw error(403, 'You do not have permission to edit this post.');
		}

		return { post, user };
	} catch (err) {
		if ((err as any)?.status === 403 || (err as any)?.status === 302) throw err;
		throw error(404, 'Post not found');
	}
};
