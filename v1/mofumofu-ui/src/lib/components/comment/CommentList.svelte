<script lang="ts">
	import CommentView from './CommentView.svelte';
	import type { CommentInfo } from '$lib/api/comment/types';

	interface Props {
		postId: string;
		initialComments?: CommentInfo[];
		initialCount?: number;
		perPage?: number;
		replyPerPage?: number;
		defaultSort?: 'latest' | 'oldest' | 'popular';
	}

	let { postId, initialComments = [], initialCount = 0, perPage, replyPerPage, defaultSort }: Props = $props();
</script>

<CommentView {postId} {initialComments} {initialCount} {perPage} {replyPerPage} {defaultSort} />
