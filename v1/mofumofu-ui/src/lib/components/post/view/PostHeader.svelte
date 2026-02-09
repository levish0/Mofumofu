<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import PostLikeButton from './PostLikeButton.svelte';
	import PostActions from './PostActions.svelte';
	import ReportDialog from '../../report/ReportDialog.svelte';
	import { userStore } from '$lib/stores/user.svelte';

	import type { PostInfoResponse, PostAuthor } from '$lib/api/post/types';

	interface Props {
		post: PostInfoResponse;
		author: PostAuthor;
		onEdit: () => void;
		onDelete: () => void;
	}

	const { post, author, onEdit, onDelete }: Props = $props();

	// 신고 다이얼로그 상태
	let isReportDialogOpen = $state(false);

	const handleReport = () => {
		isReportDialogOpen = true;
	};

	const currentUser = $derived(userStore.user);
	const isAuthor = $derived(currentUser?.handle === author.handle);

	const formatDateTime = (dateStr: string) => {
		const date = new Date(dateStr);
		return date.toLocaleString(undefined, {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			timeZoneName: 'short'
		});
	};
</script>

<header>
	<h1 class="dark:text-mofu-200 mb-4 text-4xl font-bold break-all">
		{post.title}
	</h1>

	<div class="mb-4 flex items-center justify-between">
		<a href="/@{author.handle}/profile" class="flex items-center gap-4 transition-opacity hover:opacity-80">
			{#if author.profile_image}
				<img src={author.profile_image} alt={author.name} class="h-12 w-12 rounded-full object-cover" />
			{:else}
				<div class="dark:bg-mofu-dark-600 bg-mofu-light-600 flex h-12 w-12 items-center justify-center rounded-full">
					<span class="dark:text-mofu-dark-200 text-mofu-light-200 text-lg font-medium">
						{author.name.charAt(0).toUpperCase()}
					</span>
				</div>
			{/if}
			<div>
				<p class="font-medium text-black dark:text-white">{author.name}</p>
				<p class="dark:text-mofu-dark-400 text-mofu-light-400 text-sm">
					{formatDateTime(post.created_at)}
				</p>
			</div>
		</a>

		<div class="flex items-center gap-2">
			<PostLikeButton postId={post.id} initialLikeCount={post.like_count} />

			{#if isAuthor}
				<PostActions isOwner={true} {onEdit} {onDelete} onReport={handleReport} />
			{:else}
				<PostActions isOwner={false} onEdit={() => {}} onDelete={() => {}} onReport={handleReport} />
			{/if}
		</div>
	</div>

	<div class="flex flex-wrap gap-2">
		{#each post.tags as tag}
			<Badge>#{tag}</Badge>
		{/each}
	</div>
</header>

<!-- 포스트 신고 Dialog -->
<ReportDialog
	targetId={post.id}
	targetType="Post"
	open={isReportDialogOpen}
	onOpenChange={(open) => (isReportDialogOpen = open)}
/>
