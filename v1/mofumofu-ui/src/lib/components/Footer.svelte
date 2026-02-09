<!-- src/lib/components/Footer.svelte -->
<script lang="ts">
	import * as m from '../../paraglide/messages';
</script>

<footer
	class="dark:bg-mofu-dark-900 bg-mofu-light-900 border-mofu-light-700 dark:border-mofu-dark-700 relative border-t pt-20 pb-20"
>
	<div class="custom-width container mx-auto max-w-7xl px-4">
		<div class="grid grid-cols-2 gap-6 sm:grid-cols-3">
			<!-- About Section -->
			<div class="space-y-2">
				<h3 class="text-xl font-semibold text-zinc-900 dark:text-zinc-100">もふもふ。</h3>
				<p class="text-mofu-light-300 dark:text-mofu-dark-300 text-xs">
					Mofumofu is an open-source, minimalist blogging platform. Write essays, tutorials, or daily thoughts and focus
					on what matters most.
				</p>
				<p class="text-xs text-zinc-600 dark:text-zinc-400">
					<a href="/" class="text-zinc-700 hover:underline dark:text-zinc-300">By Levish</a>
				</p>
			</div>

			<!-- Legal -->
			<div class="space-y-2">
				<h3 class="text-md font-semibold text-zinc-900 dark:text-zinc-100">{m.footer_legal()}</h3>
				<ul class="space-y-1 text-sm">
					<li>
						<a href="/privacy" class="text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100">
							{m.footer_privacy_policy()}
						</a>
					</li>
					<li>
						<a href="/terms" class="text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100">
							{m.footer_terms_of_service()}
						</a>
					</li>
				</ul>
			</div>
		</div>
	</div>
</footer>
