<script lang="ts">
	import { settingsStore } from '$lib/stores/settings.svelte.js';

	const notifications = $derived(settingsStore.notifications);
</script>

<div class="min-h-screen text-gray-100">
	<div class="space-y-8">
		<!-- Push Notifications Section -->
		<div class="space-y-4">
			<h2 class="text-xl font-semibold">Push Notifications</h2>
			<p class="text-sm text-gray-400">Manage notifications sent to your devices.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">New Posts</p>
						<p class="text-sm text-gray-400">Notify when someone you follow posts</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.newPosts}
						onchange={(e) => settingsStore.updateNotifications({ newPosts: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Comments</p>
						<p class="text-sm text-gray-400">Notify when someone comments on your posts</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.comments}
						onchange={(e) => settingsStore.updateNotifications({ comments: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Likes</p>
						<p class="text-sm text-gray-400">Notify when someone likes your posts</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.likes}
						onchange={(e) => settingsStore.updateNotifications({ likes: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">New Followers</p>
						<p class="text-sm text-gray-400">Notify when someone follows you</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.newFollowers}
						onchange={(e) =>
							settingsStore.updateNotifications({ newFollowers: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Mentions</p>
						<p class="text-sm text-gray-400">Notify when someone mentions you</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.mentions}
						onchange={(e) => settingsStore.updateNotifications({ mentions: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
			</div>
		</div>

		<!-- Email Notifications Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Email Notifications</h2>
			<p class="text-sm text-gray-400">Configure email notification preferences.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Weekly Digest</p>
						<p class="text-sm text-gray-400">Weekly summary of activity and highlights</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.weeklyDigest}
						onchange={(e) =>
							settingsStore.updateNotifications({ weeklyDigest: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Security Alerts</p>
						<p class="text-sm text-gray-400">Important security updates and login notifications</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.securityAlerts}
						onchange={(e) =>
							settingsStore.updateNotifications({ securityAlerts: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Product Updates</p>
						<p class="text-sm text-gray-400">New features and product announcements</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.productUpdates}
						onchange={(e) =>
							settingsStore.updateNotifications({ productUpdates: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Marketing</p>
						<p class="text-sm text-gray-400">Tips, recommendations, and promotional content</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.marketing}
						onchange={(e) => settingsStore.updateNotifications({ marketing: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
			</div>
		</div>

		<!-- Do Not Disturb Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Do Not Disturb</h2>
			<p class="text-sm text-gray-400">Quiet hours and notification scheduling.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Enable Do Not Disturb</p>
						<p class="text-sm text-gray-400">Pause notifications during specified hours</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.doNotDisturbEnabled}
						onchange={(e) =>
							settingsStore.updateNotifications({ doNotDisturbEnabled: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Start Time</p>
						<p class="text-sm text-gray-400">When to start quiet hours</p>
					</div>
					<input
						type="time"
						value={notifications.doNotDisturbStart}
						onchange={(e) =>
							settingsStore.updateNotifications({ doNotDisturbStart: (e.target as HTMLInputElement).value })}
						class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">End Time</p>
						<p class="text-sm text-gray-400">When to end quiet hours</p>
					</div>
					<input
						type="time"
						value={notifications.doNotDisturbEnd}
						onchange={(e) =>
							settingsStore.updateNotifications({ doNotDisturbEnd: (e.target as HTMLInputElement).value })}
						class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm"
					/>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Weekend Mode</p>
						<p class="text-sm text-gray-400">Extend quiet hours on weekends</p>
					</div>
					<input
						type="checkbox"
						checked={notifications.weekendMode}
						onchange={(e) => settingsStore.updateNotifications({ weekendMode: (e.target as HTMLInputElement).checked })}
						class="h-4 w-4 rounded border-slate-700 bg-slate-800"
					/>
				</div>
			</div>
		</div>
	</div>
</div>
