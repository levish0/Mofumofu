<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { createPost, updatePost, uploadThumbnail } from '$lib/api/post/postApi';
	import type { CreatePostRequest, UpdatePostRequest } from '$lib/api/post/types';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { Icon, PaperAirplane } from 'svelte-hero-icons';
	import { goto } from '$app/navigation';
	import { userStore } from '$lib/stores/user.svelte';
	import { toast } from 'svelte-sonner';

	import TitleInput from './TitleInput.svelte';
	import SlugInput from './SlugInput.svelte';
	import TagsInput from './TagsInput.svelte';
	import SummaryInput from './SummaryInput.svelte';
	import ThumbnailUpload from './ThumbnailUpload.svelte';
	import { ArrowLeft } from '@lucide/svelte';
	import * as m from '../../../../paraglide/messages';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';

	interface Props {
		title: string;
		content: string;
		tags: string;
		onPublished?: () => void;
		isEditMode?: boolean;
		editSlug?: string;
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
		editSlug,
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
		tags: '',
		thumbnail: null as string | null,
		thumbnailFile: null as Blob | null
	});
	let validationErrors = $state<Record<string, string>>({});

	function generateSlug(text: string): string {
		return (
			text
				.trim()
				// URL에 안전하지 않은 문자들만 제거
				.replace(/[\s\/\?#\[\]@!$&'()*+,;=]+/g, '-')
				// 연속된 하이픈을 하나로
				.replace(/-+/g, '-')
				// 앞뒤 하이픈 제거
				.replace(/^-+|-+$/g, '')
		);
	}

	function openDialog() {
		publishData = {
			title: title,
			slug: isEditMode && editSlug ? editSlug : generateSlug(title),
			content: content,
			summary: summary || '',
			tags: tags,
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
			// Clean up blob URL when removing thumbnail
			if (publishData.thumbnail && publishData.thumbnail.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}
			publishData.thumbnail = null;
			publishData.thumbnailFile = null;
		}
	}

	async function handlePublish() {
		// Clear previous errors
		validationErrors = {};

		// Validate all fields
		const schema = createPostSchema();
		const dataToValidate = {
			title: publishData.title.trim(),
			content: publishData.content.trim(),
			slug: publishData.slug.trim(),
			summary: publishData.summary.trim(),
			tags: publishData.tags
		};

		const result = v.safeParse(schema, dataToValidate);

		if (!result.success) {
			result.issues.forEach((issue) => {
				const path = issue.path?.[0]?.key as string;
				if (path) {
					validationErrors[path] = issue.message;
				}
			});
			return;
		}

		try {
			isLoading = true;
			let createdPostId: string | undefined; // <--FIX: Declare variable here

			if (isEditMode && editSlug) {
				// 수정 모드
				console.log(editPostId);
				const updateRequest: UpdatePostRequest = {
					post_id: editPostId!,
					title: publishData.title.trim(),
					content: publishData.content.trim(),
					summary: publishData.summary.trim() || null,
					hashtags: publishData.tags.trim()
						? publishData.tags
								.split(/[,\n]/)
								.map((tag) => tag.trim())
								.filter((tag) => tag.length > 0)
						: null,
					new_slug: publishData.slug.trim() !== editSlug ? publishData.slug.trim() : null
				};

				await updatePost(updateRequest);
			} else {
				// 새 포스트 생성
				const postRequest: CreatePostRequest = {
					title: publishData.title.trim(),
					content: publishData.content.trim(),
					slug: publishData.slug.trim(),
					summary: publishData.summary.trim() || null,
					hashtags: publishData.tags.trim()
						? publishData.tags
								.split(/[,\n]/)
								.map((tag) => tag.trim())
								.filter((tag) => tag.length > 0)
						: null
				};

				const createResponse = await createPost(postRequest);
				createdPostId = createResponse.post_id; // <--FIX: Assign value here
			}

			// Upload thumbnail if provided
			if (publishData.thumbnailFile) {
				try {
					// <--FIX: Use non-null assertion as it will be defined in create mode
					const postId = isEditMode ? editPostId! : createdPostId!;
					await uploadThumbnail({
						post_id: postId,
						file: new File([publishData.thumbnailFile], 'thumbnail.jpg', { type: publishData.thumbnailFile.type })
					});
				} catch (thumbnailError) {
					console.error('썸네일 업로드 실패:', thumbnailError);
					// 썸네일 업로드 실패는 전체 출간을 취소하지 않음
				}
			}

			// Clean up thumbnail blob URL after successful post creation
			if (publishData.thumbnail && publishData.thumbnail.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}

			isOpen = false;
			onPublished?.();

			toast.success(isEditMode ? '포스트가 성공적으로 수정되었습니다!' : m.publish_success_message());

			// Navigate to the published post
			const userHandle = userStore.user?.handle;
			if (userHandle) {
				const finalSlug =
					isEditMode && editSlug && publishData.slug.trim() === editSlug ? editSlug : publishData.slug.trim();
				const redirectUrl = `/@${userHandle}/post/${finalSlug}`;
				await goto(redirectUrl);
			} else {
				toast.error(m.publish_user_info_error());
			}
		} catch (error) {
			console.error(isEditMode ? '수정 실패:' : '출간 실패:', error);
			toast.error(isEditMode ? '포스트 수정에 실패했습니다. 다시 시도해주세요.' : m.publish_error_message());
		} finally {
			isLoading = false;
		}
	}

	// Check if there are any validation errors
	const hasErrors = $derived(Object.values(validationErrors).some((error) => error));
</script>

<Button
	onclick={openDialog}
	variant="ghost"
	class="dark:text-mofu-dark-950 dark:hover:bg-mofu bg-mofu flex items-center gap-2 rounded px-4 py-2 text-lg"
>
	<Icon src={PaperAirplane} class="h-5 w-5" solid />
	{isEditMode ? '수정하기' : m.publish_button()}
</Button>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content class="dark:bg-mofu-dark-900 bg-mofu-light-900 p-2 text-black sm:max-w-lg dark:text-white">
		<!-- Dialog main content with rounded-b-none -->
		<div class="rounded-t-lg rounded-b-none px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title>{isEditMode ? '포스트 수정' : m.publish_dialog_title()}</Dialog.Title>
				<Dialog.Description class="text-mofu-dark-300">
					{isEditMode ? '포스트 정보를 확인하고 수정하세요.' : m.publish_dialog_description()}
				</Dialog.Description>
			</Dialog.Header>

			<div class="hide-scrollbar max-h-[64vh] space-y-4 overflow-y-auto">
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

				<TagsInput
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

		<!-- Fixed button area -->
		<div class="dark:bg-mofu-dark-900 flex justify-end gap-2 rounded-b-lg px-2 py-2">
			<Button
				variant="ghost"
				onclick={() => (isOpen = false)}
				class="dark:text-mofu-dark-200 text-md flex items-center gap-2 rounded px-4 py-2"
			>
				<ArrowLeft class="h-5 w-5" />
				{m.common_cancel()}
			</Button>
			<Button
				onclick={handlePublish}
				disabled={isLoading || hasErrors}
				variant="ghost"
				class="dark:text-mofu-dark-950 dark:hover:bg-mofu bg-mofu text-md flex items-center gap-2 rounded px-4 py-2"
			>
				<Icon src={PaperAirplane} class="h-5 w-5" solid />
				{isLoading ? (isEditMode ? '수정 중...' : m.publish_loading()) : isEditMode ? '수정하기' : m.publish_button()}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

<LoadingOverlay
	isVisible={isLoading}
	message={isEditMode ? '포스트를 수정하고 있습니다...' : m.publish_loading_overlay()}
/>
