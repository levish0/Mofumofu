<div class="min-h-screen text-gray-100">
	<div class="space-y-8">
		<!-- Profile Privacy Section -->
		<div class="space-y-4">
			<h2 class="text-xl font-semibold">Profile Privacy</h2>
			<p class="text-sm text-gray-400">Control who can see your profile and content.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Private Account</p>
						<p class="text-sm text-gray-400">Require approval for new followers</p>
					</div>
					<input type="checkbox" class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Hide Online Status</p>
						<p class="text-sm text-gray-400">Don't show when you're online</p>
					</div>
					<input type="checkbox" class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Show in Search</p>
						<p class="text-sm text-gray-400">Allow others to find your profile in search</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Show Follower Count</p>
						<p class="text-sm text-gray-400">Display follower/following counts on profile</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
			</div>
		</div>

		<!-- Content Privacy Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Content Privacy</h2>
			<p class="text-sm text-gray-400">Manage privacy settings for your posts and interactions.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Default Post Visibility</p>
						<p class="text-sm text-gray-400">Who can see your posts by default</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>Public</option>
						<option>Followers Only</option>
						<option>Close Friends</option>
						<option>Private</option>
					</select>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Who Can Comment</p>
						<p class="text-sm text-gray-400">Control who can comment on your posts</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>Everyone</option>
						<option>Followers Only</option>
						<option>Close Friends</option>
						<option>Nobody</option>
					</select>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Who Can Mention You</p>
						<p class="text-sm text-gray-400">Control who can mention you in posts</p>
					</div>
					<select class="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm">
						<option>Everyone</option>
						<option>Followers Only</option>
						<option>Close Friends</option>
						<option>Nobody</option>
					</select>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Hide Likes</p>
						<p class="text-sm text-gray-400">Hide like counts on your posts</p>
					</div>
					<input type="checkbox" class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
			</div>
		</div>

		<!-- Data & Analytics Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Data & Analytics</h2>
			<p class="text-sm text-gray-400">Control how your data is used and shared.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Analytics Tracking</p>
						<p class="text-sm text-gray-400">Allow anonymous usage analytics</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Personalized Ads</p>
						<p class="text-sm text-gray-400">Show ads based on your activity</p>
					</div>
					<input type="checkbox" class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Data Export</p>
						<p class="text-sm text-gray-400">Download a copy of your data</p>
					</div>
					<button class="rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium hover:bg-slate-800">
						Request Export
					</button>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Data Deletion</p>
						<p class="text-sm text-gray-400">Request deletion of specific data</p>
					</div>
					<button class="rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium hover:bg-slate-800">
						Manage Data
					</button>
				</div>
			</div>
		</div>

		<!-- Security Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Security</h2>
			<p class="text-sm text-gray-400">Protect your account with security features.</p>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Login Notifications</p>
						<p class="text-sm text-gray-400">Get notified of new login attempts</p>
					</div>
					<input type="checkbox" checked class="h-4 w-4 rounded border-slate-700 bg-slate-800" />
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Active Sessions</p>
						<p class="text-sm text-gray-400">View and manage active login sessions</p>
					</div>
					<button class="rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium hover:bg-slate-800">
						Manage Sessions
					</button>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Connected Apps</p>
						<p class="text-sm text-gray-400">Review apps with access to your account</p>
					</div>
					<button class="rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium hover:bg-slate-800">
						Review Apps
					</button>
				</div>
				<div class="flex items-center justify-between">
					<div>
						<p class="font-medium">Blocked Users</p>
						<p class="text-sm text-gray-400">Manage your blocked users list</p>
					</div>
					<button class="rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium hover:bg-slate-800">
						Manage Blocked
					</button>
				</div>
			</div>
		</div>
	</div>
</div>
