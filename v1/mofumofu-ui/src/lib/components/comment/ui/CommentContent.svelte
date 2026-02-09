<script lang="ts">
	interface Props {
		content: string;
	}

	let { content }: Props = $props();
</script>

<div class="comment-content break-words whitespace-pre-wrap">
	{content}
</div>
