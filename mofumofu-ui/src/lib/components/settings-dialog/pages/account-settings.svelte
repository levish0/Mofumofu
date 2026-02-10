<script lang="ts">
	import { page } from '$app/state';
	import { invalidateAll } from '$app/navigation';
	import { isApiError } from '$lib/api';
	import { updateMyProfile, uploadProfileImage, deleteProfileImage } from '$lib/api/user';
	import { changeEmail, changePassword } from '$lib/api/auth';
	import { validateField, emailSchema, passwordSchema } from '$lib/schemas/auth';
	import { FlatInput, FLAT_INPUT_CLASS } from '$lib/components/flat-input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import * as Password from '$lib/components/ui/password';
	import * as ImageCropper from '$lib/components/ui/image-cropper';
	import { toast } from 'svelte-sonner';

	const user = $derived(page.data.user!);

	// --- Profile Image ---
	let imageUploading = $state(false);
	let cropperSrc = $state('');

	$effect(() => {
		cropperSrc = user.profile_image ?? '';
	});

	async function handleCropped(url: string) {
		imageUploading = true;
		try {
			const file = await ImageCropper.getFileFromUrl(url, 'profile.png');
			if (file.size > 4 * 1024 * 1024) {
				toast.error('Image must be under 4MB.');
				return;
			}
			await uploadProfileImage(file);
			await invalidateAll();
			toast.success('Profile image updated.');
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Upload failed.');
			else toast.error('Upload failed.');
		} finally {
			imageUploading = false;
		}
	}

	async function handleImageDelete() {
		imageUploading = true;
		try {
			await deleteProfileImage();
			await invalidateAll();
			cropperSrc = '';
			toast.success('Profile image removed.');
		} catch {
			toast.error('Failed to remove image.');
		} finally {
			imageUploading = false;
		}
	}

	// --- Basic Info ---
	let displayName = $state('');
	let bio = $state('');
	let profileSaving = $state(false);

	$effect(() => {
		displayName = user.display_name;
		bio = user.bio ?? '';
	});

	const profileChanged = $derived(displayName !== user.display_name || bio !== (user.bio ?? ''));

	async function handleProfileSave() {
		profileSaving = true;
		try {
			await updateMyProfile({
				display_name: displayName || null,
				bio: bio || null
			});
			await invalidateAll();
			toast.success('Profile updated.');
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Update failed.');
			else toast.error('Update failed.');
		} finally {
			profileSaving = false;
		}
	}

	// --- Email Change ---
	let newEmail = $state('');
	let emailPassword = $state('');
	let emailLoading = $state(false);
	let emailTouched = $state(false);
	let emailError = $derived(emailTouched ? validateField(emailSchema, newEmail) : null);

	async function handleEmailChange() {
		emailTouched = true;
		if (emailError) return;

		emailLoading = true;
		try {
			await changeEmail({ new_email: newEmail, password: emailPassword });
			toast.success('Verification email sent to your new address.');
			newEmail = '';
			emailPassword = '';
			emailTouched = false;
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Failed to change email.');
			else toast.error('Failed to change email.');
		} finally {
			emailLoading = false;
		}
	}

	// --- Password Change ---
	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let passwordLoading = $state(false);
	let newPasswordTouched = $state(false);
	let newPasswordError = $derived(
		newPasswordTouched ? validateField(passwordSchema, newPassword) : null
	);
	let confirmError = $derived(
		confirmPassword && newPassword !== confirmPassword ? 'Passwords do not match.' : null
	);

	async function handlePasswordChange() {
		newPasswordTouched = true;
		if (newPasswordError || confirmError || !currentPassword) return;

		passwordLoading = true;
		try {
			await changePassword({
				current_password: currentPassword,
				new_password: newPassword
			});
			toast.success('Password changed.');
			currentPassword = '';
			newPassword = '';
			confirmPassword = '';
			newPasswordTouched = false;
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Failed to change password.');
			else toast.error('Failed to change password.');
		} finally {
			passwordLoading = false;
		}
	}
</script>

<div class="mx-auto max-w-2xl space-y-8">
	<!-- Profile -->
	<section class="space-y-4">
		<h4 class="text-base font-semibold">Profile</h4>
		<div class="rounded-lg border p-6">
			<ImageCropper.Root
				bind:src={cropperSrc}
				onCropped={handleCropped}
				accept="image/png, image/jpeg, image/webp"
				onUnsupportedFile={() => toast.error('Unsupported file type.')}
			>
				<div class="flex items-start gap-6">
					<ImageCropper.UploadTrigger>
						<ImageCropper.Preview class="size-20 rounded-xl" />
					</ImageCropper.UploadTrigger>
					<div class="flex-1 space-y-4">
						<div>
							<p class="font-medium">Profile picture</p>
							<p class="text-sm text-muted-foreground">PNG, JPG, WebP up to 4MB</p>
						</div>
						<div class="flex gap-2">
							<ImageCropper.UploadTrigger>
								<Button
									variant="outline"
									size="sm"
									disabled={imageUploading}
									type="button"
									class="pointer-events-none"
								>
									Upload
								</Button>
							</ImageCropper.UploadTrigger>
							{#if user.profile_image}
								<Button
									variant="ghost"
									size="sm"
									class="text-red-600 hover:text-red-700"
									disabled={imageUploading}
									onclick={handleImageDelete}
								>
									Remove
								</Button>
							{/if}
						</div>
					</div>
				</div>
				<ImageCropper.Dialog>
					<ImageCropper.Cropper />
					<ImageCropper.Controls>
						<ImageCropper.Cancel />
						<ImageCropper.Crop />
					</ImageCropper.Controls>
				</ImageCropper.Dialog>
			</ImageCropper.Root>
		</div>
	</section>

	<!-- Basic Info -->
	<section class="space-y-4">
		<h4 class="text-base font-semibold">Basic Information</h4>
		<div class="rounded-lg border p-6">
			<div class="space-y-4">
				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label for="settings-handle">Handle</Label>
						<FlatInput id="settings-handle" value={user.handle} disabled />
						<p class="text-xs text-muted-foreground">Handle cannot be changed.</p>
					</div>
					<div class="space-y-2">
						<Label for="settings-display-name">Display Name</Label>
						<FlatInput
							id="settings-display-name"
							bind:value={displayName}
							disabled={profileSaving}
						/>
					</div>
				</div>
				<div class="space-y-2">
					<Label for="settings-bio">Bio</Label>
					<FlatInput
						id="settings-bio"
						placeholder="Tell us about yourself"
						bind:value={bio}
						disabled={profileSaving}
					/>
				</div>
				<Button onclick={handleProfileSave} disabled={!profileChanged || profileSaving}>
					{profileSaving ? 'Saving...' : 'Save'}
				</Button>
			</div>
		</div>
	</section>

	<!-- Email -->
	<section class="space-y-4">
		<h4 class="text-base font-semibold">Email</h4>
		<div class="rounded-lg border p-6">
			<div class="space-y-4">
				<p class="text-sm text-muted-foreground">
					Current email: <span class="font-medium">{user.email}</span>
				</p>
				<div class="space-y-2">
					<Label for="settings-new-email">New Email</Label>
					<FlatInput
						id="settings-new-email"
						type="email"
						bind:value={newEmail}
						disabled={emailLoading}
						onblur={() => (emailTouched = true)}
					/>
					{#if emailError}
						<p class="text-xs text-rose-400">{emailError}</p>
					{/if}
				</div>
				<div class="space-y-2">
					<Label for="settings-email-password">Password</Label>
					<Password.Root>
						<Password.Input
							id="settings-email-password"
							bind:value={emailPassword}
							autocomplete="current-password"
							class={FLAT_INPUT_CLASS}
							disabled={emailLoading}
						>
							<Password.ToggleVisibility />
						</Password.Input>
					</Password.Root>
				</div>
				<Button
					variant="outline"
					onclick={handleEmailChange}
					disabled={!newEmail || !emailPassword || emailLoading}
				>
					{emailLoading ? 'Sending...' : 'Send Verification'}
				</Button>
			</div>
		</div>
	</section>

	<!-- Password -->
	<section class="space-y-4">
		<h4 class="text-base font-semibold">Password</h4>
		<div class="rounded-lg border p-6">
			<div class="space-y-4">
				<div class="space-y-2">
					<Label for="settings-current-pw">Current Password</Label>
					<Password.Root>
						<Password.Input
							id="settings-current-pw"
							bind:value={currentPassword}
							autocomplete="current-password"
							class={FLAT_INPUT_CLASS}
							disabled={passwordLoading}
						>
							<Password.ToggleVisibility />
						</Password.Input>
					</Password.Root>
				</div>
				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label for="settings-new-pw">New Password</Label>
						<Password.Root>
							<Password.Input
								id="settings-new-pw"
								bind:value={newPassword}
								autocomplete="new-password"
								class={FLAT_INPUT_CLASS}
								disabled={passwordLoading}
								onblur={() => (newPasswordTouched = true)}
							>
								<Password.ToggleVisibility />
							</Password.Input>
							<Password.Strength />
						</Password.Root>
						{#if newPasswordError}
							<p class="text-xs text-rose-400">{newPasswordError}</p>
						{/if}
					</div>
					<div class="space-y-2">
						<Label for="settings-confirm-pw">Confirm Password</Label>
						<Password.Root>
							<Password.Input
								id="settings-confirm-pw"
								bind:value={confirmPassword}
								autocomplete="new-password"
								class={FLAT_INPUT_CLASS}
								disabled={passwordLoading}
							>
								<Password.ToggleVisibility />
							</Password.Input>
						</Password.Root>
						{#if confirmError}
							<p class="text-xs text-rose-400">{confirmError}</p>
						{/if}
					</div>
				</div>
				<Button
					variant="outline"
					onclick={handlePasswordChange}
					disabled={!currentPassword || !newPassword || !confirmPassword || passwordLoading}
				>
					{passwordLoading ? 'Changing...' : 'Change Password'}
				</Button>
			</div>
		</div>
	</section>

	<!-- Danger Zone -->
	<section class="space-y-4">
		<h4 class="text-base font-semibold text-red-600">Danger Zone</h4>
		<div
			class="rounded-lg border border-red-200 bg-red-50/50 p-6 dark:border-red-900/50 dark:bg-red-950/20"
		>
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<div>
					<p class="font-medium">Delete Account</p>
					<p class="text-sm text-muted-foreground">
						Once deleted, all your data will be permanently removed.
					</p>
				</div>
				<Button variant="destructive" size="sm">Delete Account</Button>
			</div>
		</div>
	</section>
</div>
