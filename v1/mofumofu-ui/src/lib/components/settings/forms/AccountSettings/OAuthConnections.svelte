<script lang="ts">
	import { Button } from '../../../ui/button';
	import { getOAuthConnections, unlinkOAuth } from '$lib/api/auth/authApi';
	import type { OAuthConnectionsResponse, OAuthProvider } from '$lib/api/auth/types';
	import { toast } from 'svelte-sonner';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { enhance } from '$app/forms';

	// settingsStore에서 데이터 가져오기
	const connections = $derived(settingsStore.account.oauthConnections as OAuthProvider[]);
	const isOAuthOnly = $derived(settingsStore.account.isOAuthOnly);
	let unlinkingProviders = $state<Set<OAuthProvider>>(new Set());

	const providerDisplayNames: Record<OAuthProvider, string> = {
		Google: 'Google',
		Github: 'GitHub'
	};

	const availableProviders: OAuthProvider[] = ['Google', 'Github'];
	const unconnectedProviders = $derived(availableProviders.filter((provider) => !connections.includes(provider)));

	async function loadConnections() {
		try {
			const response = await getOAuthConnections();
			settingsStore.updateAccountSilent({
				oauthConnections: response.connections,
				isOAuthOnly: response.is_oauth_only
			});
		} catch (error) {
			console.error('Failed to load OAuth connections:', error);
			toast.error('연결된 계정 정보를 불러오는데 실패했습니다.');
		}
	}

	async function handleUnlinkOAuth(provider: OAuthProvider) {
		if (connections.length === 1 && isOAuthOnly) {
			toast.error('마지막 OAuth 연결은 해제할 수 없습니다. 먼저 비밀번호를 설정해주세요.');
			return;
		}

		unlinkingProviders.add(provider);
		unlinkingProviders = new Set(unlinkingProviders);

		try {
			await unlinkOAuth(provider);
			await loadConnections(); // settingsStore를 업데이트
			toast.success(`${providerDisplayNames[provider]} 연결이 해제되었습니다.`);
		} catch (error) {
			console.error('Failed to unlink OAuth connection:', error);
			toast.error(`${providerDisplayNames[provider]} 연결 해제에 실패했습니다.`);
		} finally {
			unlinkingProviders.delete(provider);
			unlinkingProviders = new Set(unlinkingProviders);
		}
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">연결된 계정</h2>
	<div class="space-y-3">
		{#if connections.length === 0}
			<p class="text-mofu-light-400 dark:text-mofu-dark-400 text-sm">연결된 소셜 계정이 없습니다.</p>
		{:else}
			{#each connections as provider}
				<div
					class="border-mofu-light-700 dark:border-mofu-dark-700 bg-mofu-light-800 dark:bg-mofu-dark-800 flex items-center justify-between rounded-lg border p-4"
				>
					<div class="flex items-center space-x-3">
						<div
							class="bg-mofu-light-700 dark:bg-mofu-dark-700 flex h-10 w-10 items-center justify-center rounded-full"
						>
							{#if provider === 'Google'}
								<svg class="h-5 w-5" viewBox="0 0 24 24">
									<path
										fill="#4285F4"
										d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
									/>
									<path
										fill="#34A853"
										d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
									/>
									<path
										fill="#FBBC05"
										d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
									/>
									<path
										fill="#EA4335"
										d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
									/>
								</svg>
							{:else if provider === 'Github'}
								<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
									<path
										fill-rule="evenodd"
										d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
										clip-rule="evenodd"
									/>
								</svg>
							{/if}
						</div>
						<div>
							<p class="text-mofu-light-200 dark:text-mofu-dark-200 font-medium">{providerDisplayNames[provider]}</p>
							<p class="text-mofu-light-400 dark:text-mofu-dark-400 text-sm">연결됨</p>
						</div>
					</div>
					<Button
						variant="destructive"
						onclick={() => handleUnlinkOAuth(provider)}
						disabled={unlinkingProviders.has(provider) || (connections.length === 1 && isOAuthOnly)}
						class="h-8 px-3 text-sm"
					>
						{#if unlinkingProviders.has(provider)}
							<div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"></div>
						{:else}
							연결 해제
						{/if}
					</Button>
				</div>
			{/each}
		{/if}
		{#if isOAuthOnly && connections.length === 1}
			<p class="text-xs text-orange-400">마지막 소셜 계정 연결을 해제하려면 먼저 비밀번호를 설정해주세요.</p>
		{/if}

		{#if unconnectedProviders.length > 0}
			<div class="mt-6">
				<h3 class="mb-3 text-lg font-medium">계정 연결 추가</h3>
				<div class="space-y-2">
					{#each unconnectedProviders as provider}
						<form
							method="POST"
							action="?/{provider === 'Google' ? 'linkGoogle' : 'linkGithub'}"
							use:enhance
							class="w-full"
						>
							<button
								type="submit"
								class="border-mofu-light-700 dark:border-mofu-dark-700 bg-mofu-light-800 dark:bg-mofu-dark-800 hover:bg-mofu-light-700 dark:hover:bg-mofu-dark-700 flex w-full items-center justify-between rounded-lg border p-4 transition-colors"
							>
								<div class="flex items-center space-x-3">
									<div
										class="bg-mofu-light-700 dark:bg-mofu-dark-700 flex h-10 w-10 items-center justify-center rounded-full"
									>
										{#if provider === 'Google'}
											<svg class="h-5 w-5" viewBox="0 0 24 24">
												<path
													fill="#4285F4"
													d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
												/>
												<path
													fill="#34A853"
													d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
												/>
												<path
													fill="#FBBC05"
													d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
												/>
												<path
													fill="#EA4335"
													d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
												/>
											</svg>
										{:else if provider === 'Github'}
											<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
												<path
													fill-rule="evenodd"
													d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
													clip-rule="evenodd"
												/>
											</svg>
										{/if}
									</div>
									<div>
										<p class="text-mofu-light-200 dark:text-mofu-dark-200 font-medium">
											{providerDisplayNames[provider]}
										</p>
										<p class="text-mofu-light-400 dark:text-mofu-dark-400 text-sm">연결하기</p>
									</div>
								</div>
								<svg
									class="text-mofu-light-400 dark:text-mofu-dark-400 h-5 w-5"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
								</svg>
							</button>
						</form>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>
