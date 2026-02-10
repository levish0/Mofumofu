<script lang="ts">
	import { page } from '$app/state';
</script>

<div class="flex min-h-screen flex-col items-center justify-center">
	<h1 class="text-[clamp(3rem,15vw,12rem)] leading-none font-light tracking-tight">
		{page.status}
	</h1>
	<p class="mt-4 text-sm tracking-widest uppercase text-muted-foreground">
		{page.error?.message ?? 'Something went wrong'}
	</p>
</div>
