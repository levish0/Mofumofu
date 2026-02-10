<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { getDrafts, deleteDraft } from '$lib/api/drafts';
	import type { DraftResponse } from '$lib/api/types';
	import { Icon, DocumentText, Trash, PencilSquare } from 'svelte-hero-icons';
	import PillButton from '$lib/components/pill-button/pill-button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { toast } from 'svelte-sonner';

	const user = $derived(page.data.user);

	let drafts = $state<DraftResponse[]>([]);
	let loading = $state(true);
	let deleteTarget = $state<string | null>(null);
	let isDeleting = $state(false);

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	async function loadDrafts() {
		loading = true;
		try {
			const res = await getDrafts();
			drafts = res.data;
		} catch {
			toast.error('Failed to load drafts.');
		} finally {
			loading = false;
		}
	}

	async function confirmDelete() {
		if (!deleteTarget) return;
		isDeleting = true;
		try {
			await deleteDraft(deleteTarget);
			drafts = drafts.filter((d) => d.id !== deleteTarget);
			deleteTarget = null;
			toast.success('Draft deleted.');
		} catch {
			toast.error('Failed to delete draft.');
		} finally {
			isDeleting = false;
		}
	}

	$effect(() => {
		if (!user) {
			goto('/account/signin');
			return;
		}
		loadDrafts();
	});
</script>

<svelte:head>
	<title>My Drafts - Mofumofu</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if user}
	<div class="mx-auto max-w-4xl px-4 py-6">
		<div class="mb-6 flex items-center justify-between">
			<div>
				<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Drafts</h1>
				{#if !loading}
					<p class="mt-1 text-sm text-mofu-light-300 dark:text-mofu-dark-300">
						{drafts.length} draft{drafts.length !== 1 ? 's' : ''}
					</p>
				{/if}
			</div>
			<PillButton href="/write">New Post</PillButton>
		</div>

		{#if loading}
			<div class="space-y-3">
				{#each Array(4) as _}
					<div class="animate-pulse rounded-xl bg-mofu-light-800 p-5 dark:bg-mofu-dark-800">
						<div class="mb-3 h-5 w-48 rounded bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
						<div class="h-4 w-full rounded bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
						<div class="mt-2 h-3 w-24 rounded bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
					</div>
				{/each}
			</div>
		{:else if drafts.length === 0}
			<div class="py-20 text-center">
				<Icon
					src={DocumentText}
					class="mx-auto mb-4 h-12 w-12 text-mofu-light-400 dark:text-mofu-dark-400"
				/>
				<p class="mb-2 text-lg font-medium text-gray-900 dark:text-white">No drafts yet</p>
				<p class="mb-6 text-mofu-light-300 dark:text-mofu-dark-300">
					Drafts are automatically saved when you write a new post.
				</p>
				<PillButton href="/write">Start Writing</PillButton>
			</div>
		{:else}
			<div class="space-y-3">
				{#each drafts as draft (draft.id)}
					<div
						class="group flex items-start justify-between rounded-xl bg-mofu-light-800 p-5 transition-colors hover:bg-mofu-light-700 dark:bg-mofu-dark-800 dark:hover:bg-mofu-dark-700"
					>
						<a href="/write?draft={draft.id}" class="min-w-0 flex-1">
							<h3 class="truncate text-lg font-bold text-gray-900 dark:text-white">
								{draft.title || 'Untitled'}
							</h3>
							{#if draft.content}
								<p class="mt-1 line-clamp-2 text-sm text-mofu-light-200 dark:text-mofu-dark-200">
									{draft.content.slice(0, 200)}
								</p>
							{/if}
							<p class="mt-2 text-xs text-mofu-light-300 dark:text-mofu-dark-300">
								Updated {formatDate(draft.updated_at)}
							</p>
						</a>
						<div
							class="ml-4 flex shrink-0 items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100"
						>
							<Button variant="ghost" size="icon" href="/write?draft={draft.id}" class="h-8 w-8">
								<Icon src={PencilSquare} class="h-4 w-4" solid />
							</Button>
							<Button
								variant="ghost"
								size="icon"
								onclick={() => (deleteTarget = draft.id)}
								class="h-8 w-8 text-rose-500 hover:text-rose-600"
							>
								<Icon src={Trash} class="h-4 w-4" solid />
							</Button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Delete confirmation dialog -->
	<Dialog.Root
		open={!!deleteTarget}
		onOpenChange={(open) => {
			if (!open) deleteTarget = null;
		}}
	>
		<Dialog.Content class="sm:max-w-sm">
			<Dialog.Header>
				<Dialog.Title>Delete Draft</Dialog.Title>
				<Dialog.Description>
					This action cannot be undone. Are you sure you want to delete this draft?
				</Dialog.Description>
			</Dialog.Header>
			<Dialog.Footer>
				<Button variant="ghost" onclick={() => (deleteTarget = null)}>Cancel</Button>
				<Button variant="destructive" onclick={confirmDelete} disabled={isDeleting}>
					{isDeleting ? 'Deleting...' : 'Delete'}
				</Button>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>
{/if}
