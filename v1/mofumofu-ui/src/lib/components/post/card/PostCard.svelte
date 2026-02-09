<script lang="ts">
	import PostCardImage from './PostCardImage.svelte';
	import PostCardContent from './PostCardContent.svelte';
	import PostCardFooter from './PostCardFooter.svelte';
	import type { PostListItem } from '$lib/api/post/types';

	const {
		post = undefined
	}: {
		post?: PostListItem;
	} = $props();

	// date formatting 함수 - 정확한 날짜/시간 표시
	const formatDate = (dateStr: string) => {
		const date = new Date(dateStr);
		const year = date.getFullYear();
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const hours = String(date.getHours()).padStart(2, '0');
		const minutes = String(date.getMinutes()).padStart(2, '0');

		return `${year}.${month}.${day} ${hours}:${minutes}`;
	};

	// skeleton 모드인지 확인 (post가 없으면 skeleton)
	let isSkeleton = $derived(!post);

	// 파생된 값들
	let formattedDate = $derived(post ? formatDate(post.created_at) : '');
	let authorAvatar = $derived(post ? post.user_avatar : '');
</script>

<!-- 카드 전체 -->
<div
	data-sveltekit-preload-data="false"
	class="dark:bg-mofu-dark-800 bg-mofu-light-800 border-mofu-light-800 dark:border-mofu-dark-800 group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-lg md:min-h-[300px]"
>
	<a href={post ? `/@${post.user_handle}/post/${post.slug}` : '#'} class="flex flex-grow flex-col">
		<!-- 이미지 영역 (스켈레톤이거나 이미지가 있을 때만) -->
		{#if isSkeleton || post?.thumbnail_image}
			<PostCardImage image={post?.thumbnail_image || undefined} title={post?.title || ''} {isSkeleton} />
		{/if}

		<!-- 텍스트 영역 -->
		<PostCardContent
			title={post?.title || ''}
			summary={post?.summary || ''}
			date={formattedDate}
			comments={post?.comment_count || 0}
			hashtags={post?.hashtags || []}
			{isSkeleton}
		/>
	</a>

	<!-- 구분선과 푸터 -->
	<div class="dark:border-mofu-dark-600 border-mofu-light-600 border-t">
		<PostCardFooter
			author_name={post?.user_name || ''}
			author_avatar={authorAvatar || ''}
			author_handle={post?.user_handle || ''}
			post_id={post?.id || ''}
			likes={post?.like_count || 0}
			views={post?.view_count || 0}
			{isSkeleton}
		/>
	</div>
</div>
