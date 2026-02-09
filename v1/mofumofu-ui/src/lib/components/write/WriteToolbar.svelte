<script lang="ts">
	import {
		Heading1,
		Heading2,
		Heading3,
		Heading4,
		Bold,
		Italic,
		Strikethrough,
		Quote,
		Link,
		Image,
		Code,
		Sigma,
		ChevronUp,
		ChevronDown
	} from '@lucide/svelte';
	import { Button } from '../ui/button';
	import { Switch } from '../ui/switch';
	import { uploadImage } from '$lib/api/post/postApi';
	import { toast } from 'svelte-sonner';
	import * as m from '../../../paraglide/messages';
	interface Props {
		onInsertText: (before: string, after?: string) => void;
		showStickyToolbar: boolean;
		onToggleHeader: () => void;
		isPreviewMode?: boolean;
		onTogglePreviewMode?: (isPreview: boolean) => void;
	}

	const {
		onInsertText,
		showStickyToolbar,
		onToggleHeader,
		isPreviewMode = false,
		onTogglePreviewMode
	}: Props = $props();

	async function handleImageUpload() {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = 'image/*';

		input.onchange = async (e) => {
			const file = (e.target as HTMLInputElement).files?.[0];
			if (!file) return;

			try {
				// 파일 크기 체크 (8MB 제한)
				const fileSizeMB = file.size / (1024 * 1024);
				if (fileSizeMB > 8) {
					toast.error(m.file_size_limit_exceeded({ size: fileSizeMB.toFixed(2) }));
					return;
				}

				toast.loading(m.write_image_uploading());

				// API에 직접 업로드 (서버에서 압축 처리)
				const response = await uploadImage({ file });

				toast.dismiss();
				toast.success(m.write_image_upload_success());

				const markdownImage = `![${file.name}](${response.public_url})`;
				onInsertText(markdownImage);
			} catch (error) {
				toast.dismiss();
				toast.error(m.write_image_upload_error());
				console.error('Image upload failed:', error);
			}
		};

		input.click();
	}
</script>

<div class="dark:bg-mofu-dark-900 bg-mofu-light-900 px-4 pb-4">
	<div class="flex flex-wrap items-center justify-between gap-2">
		<div class="flex flex-wrap items-center gap-2">
			<Button
				variant="ghost"
				onclick={() => onInsertText('# ')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
			>
				<Heading1 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('## ')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
			>
				<Heading2 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('### ')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
			>
				<Heading3 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('#### ')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
			>
				<Heading4 class="h-5 w-5" />
			</Button>

			<div class="dark:bg-mofu-dark-700 bg-mofu-light-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('**', '**')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_bold()}
			>
				<Bold class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('*', '*')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_italic()}
			>
				<Italic class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('~~', '~~')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_strikethrough()}
			>
				<Strikethrough class="h-5 w-5" />
			</Button>

			<div class="bg-mofu-dark-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('> ')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_quote()}
			>
				<Quote class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('[', '](url)')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_link()}
			>
				<Link class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={handleImageUpload}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_image_upload()}
			>
				<Image class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('$$\n', '\n$$')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_math()}
			>
				<Sigma class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('```', '\n```')}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_code()}
			>
				<Code class="h-5 w-5" />
			</Button>
		</div>

		<div class="ml-auto flex items-center gap-3">
			<!-- 모바일/태블릿 전용 에디터/프리뷰 모드 스위치 -->
			{#if onTogglePreviewMode}
				<div class="flex items-center gap-2 lg:hidden">
					<span class="dark:text-mofu-dark-400 text-mofu-light-400 text-sm">{m.write_editor_mobile_label()}</span>
					<Switch checked={isPreviewMode} onCheckedChange={onTogglePreviewMode} />
					<span class="dark:text-mofu-dark-400 text-mofu-light-400 text-sm">{m.write_preview_mobile_label()}</span>
				</div>
			{/if}

			<!-- 헤더 토글 버튼 -->
			<Button
				variant="ghost"
				onclick={onToggleHeader}
				class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 rounded p-2"
				title={showStickyToolbar ? m.write_toolbar_show_header() : m.write_toolbar_hide_header()}
			>
				{#if showStickyToolbar}
					<ChevronDown class="h-5 w-5" />
				{:else}
					<ChevronUp class="h-5 w-5" />
				{/if}
			</Button>
		</div>
	</div>
</div>
