<div class="min-h-screen text-gray-100">
	<div class="space-y-8">
		<!-- Editor Preferences Section -->
		<div class="space-y-4">
			<h2 class="text-xl font-semibold">Editor Preferences</h2>
			<p class="text-sm text-gray-400">Customize your writing experience.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Default Editor Mode</p>
						<p class="text-sm text-gray-400">Choose your preferred writing mode</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>Rich Text</option>
						<option>Markdown</option>
						<option>Plain Text</option>
					</select>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Auto-save Drafts</p>
						<p class="text-sm text-gray-400">Automatically save drafts while writing</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Word Count Display</p>
						<p class="text-sm text-gray-400">Show word count while writing</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
			</div>
		</div>

		<!-- Publishing Options Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Publishing Options</h2>
			<p class="text-sm text-gray-400">Configure default publishing settings.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Default Visibility</p>
						<p class="text-sm text-gray-400">Default visibility for new posts</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>Public</option>
						<option>Followers Only</option>
						<option>Private</option>
					</select>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Allow Comments</p>
						<p class="text-sm text-gray-400">Enable comments on posts by default</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Content Warnings</p>
						<p class="text-sm text-gray-400">Prompt for content warnings when publishing</p>
					</div>
					<input type="checkbox" class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Cross-post to Social Media</p>
						<p class="text-sm text-gray-400">Automatically share posts to connected accounts</p>
					</div>
					<input type="checkbox" class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
			</div>
		</div>

		<!-- Content Management Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Content Management</h2>
			<p class="text-sm text-gray-400">Manage your content and media settings.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Draft Retention</p>
						<p class="text-sm text-gray-400">How long to keep unpublished drafts</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>30 days</option>
						<option>90 days</option>
						<option>1 year</option>
						<option>Forever</option>
					</select>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Media Upload Quality</p>
						<p class="text-sm text-gray-400">Default quality for uploaded images</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>High Quality</option>
						<option>Balanced</option>
						<option>Compressed</option>
					</select>
				</div>
			</div>
		</div>
	</div>
</div>
