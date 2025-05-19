<script lang="ts">
	// Use Skeleton Avatar for accessibility, but force circle with aspect-square and rounded-full
	// Use Megaphone icon for branding
	// Use Code icon for GitHub link
	// Use Twitter icon for Twitter link
	import { Megaphone, Code, Twitter } from 'lucide-svelte';
	import { GITHUB_URL } from '$lib/config';
</script>

<!-- Top separator only, no box -->
<hr class="hr border-surface-200-800 mb-6 w-[80%] mx-auto mt-16" />

<footer class="w-full p-4 bg-transparent">
	<!-- Two columns: About (left), Built using (right) -->
	<div class="flex flex-col md:flex-row md:justify-between md:items-stretch gap-8 md:gap-16 max-w-4xl mx-auto mb-6">
		<!-- About section: parallel structure -->
		<div class="flex-1 flex flex-col h-[184px] items-center md:items-start text-center md:text-left">
			<div class="preset-typo-title text-base md:text-lg font-semibold mb-2 py-4">About</div>
			<div class="flex flex-col justify-between h-full w-full">
				<div class="flex items-center gap-4 mb-2 justify-center md:justify-start">
					<!-- GitHub Icon Link -->
					<a href={GITHUB_URL} target="_blank" rel="noopener" aria-label="Project GitHub">
						<Code size={28} class="hover:text-primary-500 transition-colors" />
					</a>
					<!-- Twitter Icon Link -->
					<a href="https://x.com/fairtal3" target="_blank" rel="noopener" aria-label="Fairtale on Twitter">
						<Twitter size={28} class="hover:text-primary-500 transition-colors" />
					</a>
					<!-- Avatar, slightly zoomed/cropped -->
					<div class="h-10 w-10 aspect-square rounded-full overflow-hidden border border-surface-200-800 shadow flex-shrink-0">
						<img src="/img/team/fairtale.jpg" alt="Fairtale" class="w-full h-full object-cover object-center" style="transform: scale(1.05);" />
					</div>
					<!-- Built by text -->
					<span class="preset-typo-body-2">built by <a href={GITHUB_URL} target="_blank" rel="noopener" class="text-primary-500 hover:underline">@Fairtale</a></span>
				</div>
				<div>
					<span class="preset-typo-caption text-surface-500-400 mt-2">&copy; {new Date().getFullYear()} Reputator. All rights reserved.</span>
				</div>
			</div>
		</div>

		<!-- Built using section: parallel structure -->
		<div class="flex-1 flex flex-col h-[184px] items-center md:items-end text-center md:text-right">
			<div class="preset-typo-title text-base md:text-lg font-semibold mb-2 py-4">Built using</div>
			<div class="flex flex-col gap-2 items-center md:items-end h-full w-full">
				<a href="https://juno.build" target="_blank" rel="noopener" title="Juno">
					<img src="/img/partners/juno_badge_built_on_01.svg" alt="Built with Juno" class="w-56 max-w-lg h-auto block md:ml-auto" />
				</a>
				<a href="https://internetcomputer.org" target="_blank" rel="noopener" title="Internet Computer">
					<img src="/img/partners/icp_on_chain.png" alt="Internet Computer" class="w-56 max-w-lg h-auto block md:ml-auto" />
				</a>
			</div>
		</div>
	</div>
</footer>
