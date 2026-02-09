import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile } from '$lib/api/user/userApi';
import { getFollowerCount, getFollowingCount } from '$lib/api/follow/followApi';
import { getUserPosts } from '$lib/api/post/postApi';

export const load: PageServerLoad = async ({ params }) => {
	if (!params.handle) {
		throw error(404, 'Profile not found');
	}

	// Remove @ prefix if present
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	try {
		const [profile, followerCount, followingCount, userPosts] = await Promise.all([
			getUserProfile(handle),
			getFollowerCount({ user_handle: handle }),
			getFollowingCount({ user_handle: handle }),
			getUserPosts({ user_handle: handle })
		]);

		return {
			profile: profile,
			handle: params.handle,
			followerCount: followerCount.count,
			followingCount: followingCount.count,
			posts: userPosts.posts
		};
	} catch (err) {
		console.error('Failed to load profile:', err);
		throw error(404, 'Profile not found');
	}
};
