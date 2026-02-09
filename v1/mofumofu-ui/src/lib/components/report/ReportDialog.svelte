<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { createReport } from '$lib/api/report/reportApi';
	import type { ReportReason, ReportTargetType } from '$lib/api/report/types';
	import { toast } from 'svelte-sonner';
	import { Flag } from '@lucide/svelte';

	interface Props {
		targetId: string;
		targetType: ReportTargetType;
		open: boolean;
		onOpenChange: (open: boolean) => void;
	}

	let { targetId, targetType, open, onOpenChange }: Props = $props();

	let selectedReasons = $state<ReportReason[]>([]);
	let description = $state('');
	let isSubmitting = $state(false);

	const reasons: { value: ReportReason; label: string; description: string }[] = [
		{
			value: 'Spam',
			label: '스팸',
			description: '반복적이거나 원치 않는 콘텐츠'
		},
		{
			value: 'Harassment',
			label: '괴롭힘',
			description: '타인을 괴롭히거나 위협하는 내용'
		},
		{
			value: 'InappropriateContent',
			label: '부적절한 콘텐츠',
			description: '음란물, 폭력적 내용 등 부적절한 콘텐츠'
		},
		{
			value: 'Copyright',
			label: '저작권 침해',
			description: '허가 없이 사용된 저작권 보호 콘텐츠'
		},
		{
			value: 'Other',
			label: '기타',
			description: '위에 해당하지 않는 다른 문제'
		}
	];

	const toggleReason = (reason: ReportReason) => {
		if (selectedReasons.includes(reason)) {
			selectedReasons = selectedReasons.filter((r) => r !== reason);
		} else {
			selectedReasons = [...selectedReasons, reason];
		}
	};

	const handleSubmit = async () => {
		if (selectedReasons.length === 0) {
			toast.error('신고 사유를 선택해주세요.');
			return;
		}

		try {
			isSubmitting = true;

			await createReport({
				target_id: targetId,
				target_type: targetType,
				reasons: selectedReasons,
				description: description.trim() || undefined
			});

			toast.success('신고가 접수되었습니다. 검토 후 적절한 조치를 취하겠습니다.');
			onOpenChange(false);

			// Reset form
			selectedReasons = [];
			description = '';
		} catch (error) {
			console.error('Failed to submit report:', error);
			toast.error('신고 접수에 실패했습니다. 다시 시도해주세요.');
		} finally {
			isSubmitting = false;
		}
	};

	const targetTypeLabel = $derived.by(() => {
		switch (targetType) {
			case 'Post':
				return '게시물';
			case 'Comment':
				return '댓글';
			case 'User':
				return '사용자';
			default:
				return '콘텐츠';
		}
	});
</script>

<Dialog.Root {open} {onOpenChange}>
	<Dialog.Content class="dark:bg-mofu-dark-800 sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title class="flex items-center gap-2">
				<Flag class="h-5 w-5 text-rose-500" />
				{targetTypeLabel} 신고하기
			</Dialog.Title>
			<Dialog.Description>
				이 {targetTypeLabel}이 커뮤니티 가이드라인을 위반한다고 생각하시면 신고해주세요. 로그인하지 않아도 익명으로
				신고할 수 있습니다.
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-4 py-4">
			<div>
				<h4 class="mb-3 text-sm font-medium">신고 사유를 선택하세요 (복수 선택 가능)</h4>
				<div class="space-y-2">
					{#each reasons as reason}
						<label
							class="hover:bg-mofu-light-900 dark:hover:bg-mofu-dark-700 flex cursor-pointer items-start gap-3 rounded-lg p-2 transition-colors"
						>
							<input
								type="checkbox"
								class="mt-1"
								checked={selectedReasons.includes(reason.value)}
								onchange={() => toggleReason(reason.value)}
							/>
							<div class="min-w-0 flex-1">
								<div class="text-sm font-medium">{reason.label}</div>
								<div class="text-mofu-light-500 dark:text-mofu-dark-400 mt-1 text-xs">
									{reason.description}
								</div>
							</div>
						</label>
					{/each}
				</div>
			</div>

			<div>
				<label for="description" class="mb-2 block text-sm font-medium"> 추가 설명 (선택사항) </label>
				<textarea
					id="description"
					bind:value={description}
					placeholder="추가로 설명하고 싶은 내용이 있다면 적어주세요..."
					class="border-mofu-light-300 dark:border-mofu-dark-600 focus:ring-mofu w-full resize-none rounded-lg border bg-transparent p-2 focus:border-transparent focus:ring-2 focus:outline-none"
					rows="3"
				></textarea>
			</div>
		</div>

		<Dialog.Footer class="gap-2">
			<Button variant="outline" onclick={() => onOpenChange(false)} disabled={isSubmitting}>취소</Button>
			<Button
				onclick={handleSubmit}
				disabled={isSubmitting || selectedReasons.length === 0}
				class="bg-red-600 text-white hover:bg-red-700"
			>
				{isSubmitting ? '신고 중...' : '신고하기'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
