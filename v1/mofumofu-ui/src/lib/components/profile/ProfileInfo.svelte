<script lang="ts">
	import type { UserInfoResponse } from '$lib/api/user/types';
	import { Briefcase, CalendarDays, Icon, Link, MapPin } from 'svelte-hero-icons';
	import * as m from '../../../paraglide/messages';
	import Autolinker from 'autolinker';
	import escapeHtml from 'escape-html';

	type Props = {
		profile: UserInfoResponse;
		followerCount?: number;
		followingCount?: number;
	};

	const { profile, followerCount = 0, followingCount = 0 }: Props = $props();

	// 가입 날짜 포맷팅 함수
	function formatJoinDate(createdAt?: string): string {
		if (!createdAt) return 'Joined August 2017'; // 기본값

		const date = new Date(createdAt);
		const options: Intl.DateTimeFormatOptions = {
			year: 'numeric',
			month: 'long'
		};
		return `Joined ${date.toLocaleDateString('en-US', options)}`;
	}

	function linkifyBio(text: string): string {
		// 1. First escape HTML
		const escapedText = escapeHtml(text);

		// 2. Convert line breaks to <br> tags
		const textWithBreaks = escapedText.replace(/\n/g, '<br>');

		// Then apply autolinker
		return Autolinker.link(textWithBreaks, {
			urls: true,
			email: true,
			phone: false,
			mention: false,
			hashtag: false,
			newWindow: true,
			className: 'text-mofu hover:opacity-70 underline transition-colors'
		});
	}
</script>

<!-- Profile Content -->
<div class="px-2 pt-14 pb-4">
	<!-- Profile Info -->
	<div class="space-y-2">
		<div>
			<h1 class="text-xl font-bold text-gray-900 dark:text-white">
				{profile.name}
			</h1>
			<p class="dark:text-mofu-dark-300 text-sm">
				@{profile.handle}
			</p>
		</div>

		<!-- Bio -->
		{#if profile.bio}
			<p class="dark:text-mofu-dark-200 text-md text-gray-700">
				{@html linkifyBio(profile.bio)}
			</p>
		{/if}

		<!-- Location and Join Date -->
		<div class="dark:text-mofu-dark-300 flex items-center gap-4 text-sm">
			{#if profile.location}
				<div class="flex items-center gap-1">
					<Icon src={MapPin} class="h-4 w-4" />
					<span>{profile.location}</span>
				</div>
			{/if}
			{#if profile.website}
				<div class="flex items-center gap-1">
					<Icon src={Link} class="h-4 w-4" />
					<a
						href={profile.website}
						target="_blank"
						rel="noopener noreferrer"
						class="text-mofu underline transition-colors hover:opacity-70"
					>
						{profile.website.replace(/^https?:\/\//, '')}
					</a>
				</div>
			{/if}
			<!--
			<div class="flex items-center gap-1">
				<Icon src={Briefcase} class="h-4 w-4" />
				<span>mofumofu</span>
			</div>
			-->
			<div class="flex items-center gap-1">
				<Icon src={CalendarDays} class="h-4 w-4" />
				<span>{formatJoinDate(profile.created_at)}</span>
			</div>
		</div>

		<!-- Stats -->
		<div class="flex items-center space-x-4 text-sm">
			<div>
				<span class="font-bold text-gray-900 dark:text-white">{followingCount.toLocaleString()}</span>
				<span class="dark:text-mofu-dark-300">{m.profile_following()}</span>
			</div>
			<div>
				<span class="font-bold text-gray-900 dark:text-white">{followerCount.toLocaleString()}</span>
				<span class="dark:text-mofu-dark-300">{m.profile_followers()}</span>
			</div>
		</div>
	</div>
</div>
