<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>Terms of Service - Mofumofu</title>
	<meta name="description" content="Mofumofu's terms of service and user agreement" />
</svelte:head>

<div class="bg-mofu-dark-900 text-mofu-dark-100 min-h-screen">
	<div class="mx-auto max-w-4xl px-4 py-12">
		<div class="prose prose-invert max-w-none">
			<h1 class="mb-8 text-4xl font-bold">Terms of Service</h1>

			<p class="text-mofu-dark-300 mb-8 text-lg">
				Last updated: {new Date().toLocaleDateString()}
			</p>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">1. Acceptance of Terms</h2>
				<p class="mb-4">
					By accessing and using Mofumofu ("the Service"), you accept and agree to be bound by the terms and provision
					of this agreement. If you do not agree to abide by the above, please do not use this service.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">2. Use License</h2>
				<p class="mb-4">
					Permission is granted to temporarily access Mofumofu for personal, non-commercial transitory viewing only.
					This is the grant of a license, not a transfer of title, and under this license you may not:
				</p>
				<ul class="mb-4 list-disc space-y-2 pl-6">
					<li>Modify or copy the materials</li>
					<li>Use the materials for any commercial purpose or for any public display</li>
					<li>Attempt to reverse engineer any software contained on the website</li>
					<li>Remove any copyright or other proprietary notations from the materials</li>
				</ul>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">3. User Accounts</h2>
				<p class="mb-4">
					When you create an account with us, you must provide information that is accurate, complete, and current at
					all times. You are responsible for safeguarding the password and for all activities that occur under your
					account.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">4. Content Policy</h2>
				<p class="mb-4">
					Our Service allows you to post, link, store, share and otherwise make available certain information, text,
					graphics, videos, or other material ("Content"). You are responsible for Content that you post to the Service,
					including its legality, reliability, and appropriateness.
				</p>
				<p class="mb-4">You agree not to post Content that:</p>
				<ul class="mb-4 list-disc space-y-2 pl-6">
					<li>
						Is unlawful, harmful, threatening, abusive, harassing, defamatory, vulgar, obscene, or otherwise
						objectionable
					</li>
					<li>Infringes any patent, trademark, trade secret, copyright, or other intellectual property rights</li>
					<li>
						Contains software viruses or any other computer code designed to interrupt, destroy, or limit functionality
					</li>
					<li>
						Impersonates any person or entity or falsely states or misrepresents your affiliation with a person or
						entity
					</li>
				</ul>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">5. Intellectual Property Rights</h2>
				<p class="mb-4">
					The Service and its original content, features, and functionality are and will remain the exclusive property
					of Mofumofu and its licensors. The Service is protected by copyright, trademark, and other laws.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">6. Termination</h2>
				<p class="mb-4">
					We may terminate or suspend your account and bar access to the Service immediately, without prior notice or
					liability, under our sole discretion, for any reason whatsoever, including without limitation if you breach
					the Terms.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">7. Disclaimer</h2>
				<p class="mb-4">
					The information on this website is provided on an "as is" basis. To the fullest extent permitted by law, this
					Company:
				</p>
				<ul class="mb-4 list-disc space-y-2 pl-6">
					<li>Excludes all representations and warranties relating to this website and its contents</li>
					<li>Does not warrant that the functions contained in this website will be uninterrupted or error-free</li>
					<li>Excludes all liability for damages arising out of or in connection with your use of this website</li>
				</ul>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">8. Limitation of Liability</h2>
				<p class="mb-4">
					In no event shall Mofumofu, nor its directors, employees, partners, agents, suppliers, or affiliates, be
					liable for any indirect, incidental, special, consequential, or punitive damages, including without
					limitation, loss of profits, data, use, goodwill, or other intangible losses.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">9. Governing Law</h2>
				<p class="mb-4">
					These Terms shall be interpreted and governed by the laws of the jurisdiction in which we operate, without
					regard to its conflict of law provisions.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">10. Changes to Terms</h2>
				<p class="mb-4">
					We reserve the right, at our sole discretion, to modify or replace these Terms at any time. If a revision is
					material, we will try to provide at least 30 days notice prior to any new terms taking effect.
				</p>
			</section>

			<section class="mb-8">
				<h2 class="mb-4 text-2xl font-semibold">11. Contact Information</h2>
				<p class="mb-4">
					If you have any questions about these Terms of Service, please contact us at levish@mofumofu.com
				</p>
			</section>
		</div>
	</div>
</div>
