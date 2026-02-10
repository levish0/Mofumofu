import { getMe } from '$lib/api/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	const user = await getMe(locals.api);
	return { user };
};
