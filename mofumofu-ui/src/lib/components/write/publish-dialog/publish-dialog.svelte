<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { createPost, updatePost, uploadPostImage } from '$lib/api/posts/postsApi';
	import type { CreatePostRequest, UpdatePostRequest } from '$lib/api/types';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { Icon, PaperAirplane } from 'svelte-hero-icons';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { toast } from 'svelte-sonner';
	import { ArrowLeft } from '@lucide/svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';

	import TitleInput from './title-input.svelte';
	import SlugInput from './slug-input.svelte';
	import PublishTagsInput from './tags-input.svelte';
	import SummaryInput from './summary-input.svelte';
	import ThumbnailUpload from './thumbnail-upload.svelte';

	interface Props {
		title: string;
		content: string;
		tags: string[];
		onPublished?: () => void;
		isEditMode?: boolean;
		editPostId?: string;
		summary?: string;
		existingThumbnail?: string | null;
	}

	let {
		title,
		content,
		tags,
		onPublished,
		isEditMode = false,
		editPostId,
		summary,
		existingThumbnail
	}: Props = $props();

	let isOpen = $state(false);
	let isLoading = $state(false);
	let publishData = $state({
		title: '',
		slug: '',
		content: '',
		summary: '',
		tags: [] as string[],
		thumbnail: null as string | null,
		thumbnailFile: null as Blob | null
	});
	let validationErrors = $state<Record<string, string>>({});

	function generateSlug(text: string): string {
		return text
			.trim()
			.replace(/[\s\/\?#\[\]@!$&'()*+,;=]+/g, '-')
			.replace(/-+/g, '-')
			.replace(/^-+|-+$/g, '');
	}

	function openDialog() {
		publishData = {
			title,
			slug: isEditMode ? generateSlug(title) : generateSlug(title),
			content,
			summary: summary || '',
			tags: [...tags],
			thumbnail: isEditMode ? existingThumbnail || null : null,
			thumbnailFile: null
		};
		validationErrors = {};
		isOpen = true;
	}

	function handleValidationChange(field: string) {
		return (error?: string) => {
			validationErrors[field] = error || '';
		};
	}

	function updateField<K extends keyof typeof publishData>(field: K) {
		return (value: (typeof publishData)[K]) => {
			publishData[field] = value;
		};
	}

	function handleThumbnailUpdate(data: { thumbnailFile: Blob; thumbnail: string } | null) {
		if (data) {
			publishData.thumbnail = data.thumbnail;
			publishData.thumbnailFile = data.thumbnailFile;
		} else {
			if (publishData.thumbnail?.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}
			publishData.thumbnail = null;
			publishData.thumbnailFile = null;
		}
	}

	async function handlePublish() {
		validationErrors = {};

		const schema = createPostSchema();
		const tagsString = publishData.tags.join(',');
		const result = v.safeParse(schema, {
			title: publishData.title.trim(),
			content: publishData.content.trim(),
			slug: publishData.slug.trim(),
			summary: publishData.summary.trim(),
			tags: tagsString
		});

		if (!result.success) {
			result.issues.forEach((issue) => {
				const path = issue.path?.[0]?.key as string;
				if (path) validationErrors[path] = issue.message;
			});
			return;
		}

		try {
			isLoading = true;

			// Upload thumbnail first if a new file was selected
			let thumbnailUrl: string | null = null;
			if (publishData.thumbnailFile) {
				try {
					const file = new File([publishData.thumbnailFile], 'thumbnail.jpg', {
						type: publishData.thumbnailFile.type
					});
					const uploadResult = await uploadPostImage(file);
					thumbnailUrl = uploadResult.image_url;
				} catch (err) {
					console.error('Thumbnail upload failed:', err);
					// Continue without thumbnail
				}
			} else if (publishData.thumbnail && !publishData.thumbnail.startsWith('blob:')) {
				// Keep existing thumbnail URL
				thumbnailUrl = publishData.thumbnail;
			}

			const hashtags = publishData.tags.length > 0 ? publishData.tags : null;
			let finalSlug = publishData.slug.trim();

			if (isEditMode && editPostId) {
				const updateRequest: UpdatePostRequest = {
					title: publishData.title.trim(),
					content: publishData.content.trim(),
					slug: finalSlug,
					summary: publishData.summary.trim() || null,
					hashtags,
					thumbnail_image: thumbnailUrl
				};
				await updatePost(editPostId, updateRequest);
			} else {
				const postRequest: CreatePostRequest = {
					title: publishData.title.trim(),
					content: publishData.content.trim(),
					slug: finalSlug,
					summary: publishData.summary.trim() || null,
					hashtags,
					thumbnail_image: thumbnailUrl,
					publish: true
				};
				await createPost(postRequest);
			}

			// Clean up blob URL
			if (publishData.thumbnail?.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}

			isOpen = false;
			onPublished?.();

			toast.success(isEditMode ? 'Post updated successfully!' : 'Post published successfully!');

			const userHandle = page.data.user?.handle;
			if (userHandle) {
				await goto(`/@${userHandle}/post/${finalSlug}`);
			}
		} catch (error) {
			console.error(isEditMode ? 'Update failed:' : 'Publish failed:', error);
			toast.error(
				isEditMode
					? 'Failed to update post. Please try again.'
					: 'Failed to publish post. Please try again.'
			);
		} finally {
			isLoading = false;
		}
	}

	const hasErrors = $derived(Object.values(validationErrors).some((e) => e));
</script>

<Button
	onclick={openDialog}
	variant="ghost"
	class="flex items-center gap-2 bg-primary px-4 py-2 text-lg text-primary-foreground hover:bg-primary/90"
>
	<Icon src={PaperAirplane} class="h-5 w-5" solid />
	{isEditMode ? 'Update' : 'Publish'}
</Button>

{#if isLoading}
	<div
		class="pointer-events-auto fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm"
		role="dialog"
		aria-modal="true"
	>
		<div class="flex flex-col items-center gap-4">
			<Spinner class="size-12 text-white" />
			<p class="text-lg font-medium text-white">
				{isEditMode ? 'Updating post...' : 'Publishing post...'}
			</p>
		</div>
	</div>
{/if}

<Dialog.Root
	bind:open={isOpen}
	onOpenChange={(open) => {
		if (isLoading) return;
		isOpen = open;
	}}
>
	<Dialog.Content class="sm:max-w-lg">
		<div class="px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title>{isEditMode ? 'Update Post' : 'Publish Post'}</Dialog.Title>
				<Dialog.Description class="text-muted-foreground">
					{isEditMode
						? 'Review and update your post details.'
						: 'Review your post details before publishing.'}
				</Dialog.Description>
			</Dialog.Header>

			<div class="max-h-[64vh] space-y-4 overflow-y-auto">
				<ThumbnailUpload thumbnail={publishData.thumbnail} onUpdate={handleThumbnailUpdate} />
				<TitleInput
					value={publishData.title}
					onUpdate={updateField('title')}
					onValidationChange={handleValidationChange('title')}
				/>
				<SlugInput
					value={publishData.slug}
					onUpdate={updateField('slug')}
					onValidationChange={handleValidationChange('slug')}
				/>
				<PublishTagsInput
					value={publishData.tags}
					onUpdate={updateField('tags')}
					onValidationChange={handleValidationChange('tags')}
				/>
				<SummaryInput
					value={publishData.summary}
					onUpdate={updateField('summary')}
					onValidationChange={handleValidationChange('summary')}
				/>
			</div>
		</div>

		<Dialog.Footer class="px-2 py-2">
			<Button variant="ghost" onclick={() => (isOpen = false)} class="flex items-center gap-2">
				<ArrowLeft class="h-5 w-5" />
				Cancel
			</Button>
			<Button
				onclick={handlePublish}
				disabled={isLoading || hasErrors}
				class="flex items-center gap-2"
			>
				<Icon src={PaperAirplane} class="h-5 w-5" solid />
				{#if isLoading}
					{isEditMode ? 'Updating...' : 'Publishing...'}
				{:else}
					{isEditMode ? 'Update' : 'Publish'}
				{/if}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
