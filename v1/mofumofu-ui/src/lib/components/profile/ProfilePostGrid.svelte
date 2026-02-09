<script lang="ts">
	import PostCard from '../post/card/PostCard.svelte';
	import type { PostListItem } from '$lib/api/post/types';
	import { Badge } from '../ui/badge';
	import { getContext } from 'svelte';

	type Props = {
		profile: {
			handle: string;
			name: string;
			profile_image?: string;
		};
		posts: PostListItem[];
	};

	const { profile, posts = [] }: Props = $props();

	// 현재 로드된 포스트들의 해시태그와 개수
	let availableHashtags = $state<Array<{ tag: string; count: number }>>([]);
	let selectedTags = $state<string[]>([]);

	// 전체 포스트들에서 해시태그 추출하고 개수 계산
	function extractHashtagsFromPosts() {
		const hashtagCounts = new Map<string, number>();

		posts.forEach((post) => {
			post.hashtags.forEach((tag) => {
				if (tag.trim()) {
					const trimmedTag = tag.trim();
					hashtagCounts.set(trimmedTag, (hashtagCounts.get(trimmedTag) || 0) + 1);
				}
			});
		});

		availableHashtags = Array.from(hashtagCounts.entries())
			.map(([tag, count]) => ({ tag, count }))
			.sort((a, b) => b.count - a.count); // 개수 순으로 정렬
	}

	// 태그 토글 함수 - 클라이언트 사이드 필터링
	function toggleTag(tag: string) {
		if (selectedTags.includes(tag)) {
			selectedTags = selectedTags.filter((t) => t !== tag);
		} else {
			selectedTags = [...selectedTags, tag];
		}
	}

	// 필터링된 포스트들
	const filteredPosts = $derived(
		(() => {
			if (selectedTags.length === 0) {
				return posts; // 선택된 태그가 없으면 모든 포스트 표시
			}

			// 선택된 태그 중 하나라도 포함하는 포스트만 필터링
			return posts.filter((post) => {
				return selectedTags.some((selectedTag) =>
					post.hashtags.some((postTag) => postTag.toLowerCase().includes(selectedTag.toLowerCase()))
				);
			});
		})()
	);

	// navbar context 가져오기
	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// navbar 상태에 따른 sticky top 위치 계산
	const stickyTopPosition = $derived(navbar?.isVisible() ? '60px' : '0px');

	// 포스트가 변경될 때마다 해시태그 추출
	$effect(() => {
		if (posts.length > 0) {
			extractHashtagsFromPosts();
		}
	});
</script>

<div class="space-y-2">
	<!-- Sticky Hashtags Section -->
	<div
		class="dark:bg-mofu-dark-900 bg-mofu-light-900 sticky z-20 pt-2 pb-2 transition-all duration-100 ease-out"
		style="top: {stickyTopPosition}"
	>
		<div class="flex-1">
			{#if availableHashtags.length > 0}
				<div class="flex flex-wrap gap-2">
					{#each availableHashtags as { tag, count }}
						<Badge
							variant="secondary"
							class="cursor-pointer  {selectedTags.includes(tag) &&
								'bg-mofu dark:bg-mofu text-white dark:text-black'} transition-colors"
							onclick={() => toggleTag(tag)}
						>
							#{tag}({count})
						</Badge>
					{/each}
				</div>
			{:else}
				<div class="dark:text-mofu-dark-400 text-mofu-light-400 text-sm">필터할 태그가 없습니다.</div>
			{/if}
		</div>
	</div>

	<!-- Posts Grid -->
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
		{#each filteredPosts as post (`${post.user_handle}-${post.slug}`)}
			<PostCard {post} />
		{/each}
	</div>

	<!-- Empty state -->
	{#if filteredPosts.length === 0}
		<div class="flex flex-col items-center justify-center py-12 text-center">
			{#if selectedTags.length > 0}
				<div class="dark:text-mofu-dark-400 text-mofu-light-400 mb-2 text-lg">
					선택한 태그와 일치하는 포스트가 없습니다
				</div>
				<div class="dark:text-mofu-dark-500 text-mofu-light-500 mb-4 text-sm">
					다른 태그를 선택하거나 필터를 해제해보세요.
				</div>
				<button onclick={() => (selectedTags = [])} class="text-mofu hover:text-mofu/80 text-sm underline">
					모든 필터 해제
				</button>
			{:else if posts.length === 0}
				<div class="dark:text-mofu-dark-400 text-mofu-light-400 mb-2 text-lg">작성된 포스트가 없습니다</div>
				<div class="dark:text-mofu-dark-500 text-mofu-light-500 text-sm">
					{profile.name}님이 아직 포스트를 작성하지 않았습니다.
				</div>
			{/if}
		</div>
	{/if}
</div>
