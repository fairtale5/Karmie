<script lang="ts">
	/**
	 * Graph Test Page - Phase 1 Implementation
	 * 
	 * HOW: Demonstrates Sigma.js integration with existing theme system
	 * - Shows interactive reputation network visualization
	 * - Tests theme switching functionality with graph updates
	 * - Provides foundation for Phase 2 backend integration
	 * - Uses existing Skeleton UI components for consistency
	 */
	import { setPageMeta } from '$lib/stores/page';
	import SigmaGraph from '$lib/components/graph/SigmaGraph.svelte';
	import { themeStore } from '$lib/stores/theme';

	// HOW: Set page metadata for navigation and SEO
	setPageMeta({
		title: 'Graph Visualization',
		description: 'Interactive reputation network visualization using Sigma.js'
	});

	// HOW: Component state for interactive demos
	let graphHeight = '600px';
	let showControls = true;
	let autoLayout = true;

	// HOW: Demo configuration options
	const heightOptions = [
		{ label: 'Small (400px)', value: '400px' },
		{ label: 'Medium (600px)', value: '600px' },
		{ label: 'Large (800px)', value: '800px' },
		{ label: 'Full Height (100vh)', value: '100vh' }
	];

	// HOW: Toggle theme for testing theme integration
	function toggleTheme() {
		themeStore.toggle();
	}
</script>

<!-- 
HOW: Page layout using existing design system
- Container provides consistent spacing and responsive behavior
- Cards organize content into logical sections
- Typography scales with theme settings
-->
<div class="container mx-auto p-8 space-y-8">
	
	<!-- Page Header -->
	<header class="text-center space-y-4">
		<h1 class="h1">🌐 Graph Visualization Demo</h1>
		<p class="text-xl text-surface-600-300">
			Interactive reputation network powered by Sigma.js and Graphology
		</p>
		<div class="flex justify-center gap-4">
			<span class="badge variant-soft-primary">Phase 1: Dummy Data</span>
			<span class="badge variant-soft-secondary">Theme Integration</span>
			<span class="badge variant-soft-tertiary">ForceAtlas2 Layout</span>
		</div>
	</header>

	<!-- Graph Configuration Panel -->
	<section class="card p-6">
		<header class="card-header">
			<h2 class="h3">⚙️ Configuration</h2>
			<p class="text-surface-600-300">Adjust graph settings and test theme integration</p>
		</header>
		
		<div class="card-body grid grid-cols-1 md:grid-cols-3 gap-6">
			
			<!-- Height Control -->
			<div class="space-y-2">
				<label class="label" for="height-select">
					<span>Graph Height</span>
				</label>
				<select 
					id="height-select"
					class="select" 
					bind:value={graphHeight}
				>
					{#each heightOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>

			<!-- Controls Toggle -->
			<div class="space-y-2">
				<label class="label">
					<span>Display Options</span>
				</label>
				<div class="space-y-2">
					<label class="flex items-center space-x-2">
						<input 
							class="checkbox" 
							type="checkbox" 
							bind:checked={showControls}
						/>
						<span>Show Controls</span>
					</label>
					<label class="flex items-center space-x-2">
						<input 
							class="checkbox" 
							type="checkbox" 
							bind:checked={autoLayout}
						/>
						<span>Auto Layout</span>
					</label>
				</div>
			</div>

			<!-- Theme Test -->
			<div class="space-y-2">
				<label class="label">
					<span>Theme Testing</span>
				</label>
				<button 
					class="btn variant-filled-surface w-full"
					on:click={toggleTheme}
				>
					🌓 Toggle Theme
				</button>
				<p class="text-sm text-surface-500-400">
					Test how graph colors update with theme changes
				</p>
			</div>
		</div>
	</section>

	<!-- Main Graph Display -->
	<section class="card">
		<header class="card-header">
			<h2 class="h3">📊 Reputation Network</h2>
			<p class="text-surface-600-300">
				Interactive visualization of user relationships and topic contributions
			</p>
		</header>
		
		<div class="card-body p-0">
			<SigmaGraph 
				height={graphHeight}
				{showControls}
				{autoLayout}
			/>
		</div>
	</section>

	<!-- Graph Legend and Instructions -->
	<section class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		
		<!-- Legend -->
		<div class="card">
			<header class="card-header">
				<h3 class="h4">🗂️ Legend</h3>
			</header>
			<div class="card-body space-y-4">
				
				<!-- Node Types -->
				<div class="space-y-2">
					<h4 class="h5">Node Types</h4>
					<div class="grid grid-cols-2 gap-2 text-sm">
						<div class="flex items-center gap-2">
							<div class="w-4 h-4 rounded-full bg-primary-500"></div>
							<span>Users (size = reputation)</span>
						</div>
						<div class="flex items-center gap-2">
							<div class="w-4 h-4 rounded-full bg-secondary-500"></div>
							<span>Topics</span>
						</div>
					</div>
				</div>

				<!-- Edge Types -->
				<div class="space-y-2">
					<h4 class="h5">Edge Types</h4>
					<div class="grid grid-cols-1 gap-2 text-sm">
						<div class="flex items-center gap-2">
							<div class="w-8 h-1 bg-success-500"></div>
							<span>Upvotes</span>
						</div>
						<div class="flex items-center gap-2">
							<div class="w-8 h-1 bg-error-500"></div>
							<span>Downvotes</span>
						</div>
						<div class="flex items-center gap-2">
							<div class="w-8 h-1 bg-tertiary-500"></div>
							<span>Contributes to topic</span>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Instructions -->
		<div class="card">
			<header class="card-header">
				<h3 class="h4">🎮 Interactions</h3>
			</header>
			<div class="card-body space-y-3 text-sm">
				<div class="flex items-start gap-2">
					<span class="badge variant-soft">🖱️</span>
					<div>
						<strong>Click nodes/edges</strong> to select and view details in console
					</div>
				</div>
				<div class="flex items-start gap-2">
					<span class="badge variant-soft">🎯</span>
					<div>
						<strong>Drag nodes</strong> to manually position them
					</div>
				</div>
				<div class="flex items-start gap-2">
					<span class="badge variant-soft">🔍</span>
					<div>
						<strong>Mouse wheel</strong> to zoom in/out
					</div>
				</div>
				<div class="flex items-start gap-2">
					<span class="badge variant-soft">📱</span>
					<div>
						<strong>Drag background</strong> to pan around the graph
					</div>
				</div>
				<div class="flex items-start gap-2">
					<span class="badge variant-soft">⚡</span>
					<div>
						<strong>Apply Layout</strong> to run ForceAtlas2 positioning
					</div>
				</div>
				<div class="flex items-start gap-2">
					<span class="badge variant-soft">🎯</span>
					<div>
						<strong>Center View</strong> to reset camera position
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Technical Details -->
	<section class="card">
		<header class="card-header">
			<h3 class="h4">🔧 Technical Implementation</h3>
		</header>
		<div class="card-body">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 text-sm">
				
				<div class="space-y-3">
					<h4 class="h6 text-primary-500">Phase 1: Foundation</h4>
					<ul class="list space-y-1">
						<li>✅ Sigma.js v2 + Graphology integration</li>
						<li>✅ ForceAtlas2 layout algorithm</li>
						<li>✅ Dark/Light theme switching</li>
						<li>✅ Dummy reputation data</li>
						<li>✅ Interactive controls</li>
						<li>✅ Responsive design</li>
					</ul>
				</div>

				<div class="space-y-3">
					<h4 class="h6 text-secondary-500">Phase 2: Backend Integration</h4>
					<ul class="list space-y-1">
						<li>🔄 Real reputation data from backend</li>
						<li>🔄 Custom API endpoint for graph data</li>
						<li>🔄 Dynamic graph updates</li>
						<li>🔄 User filtering and search</li>
						<li>🔄 Performance optimization</li>
						<li>🔄 Export/import functionality</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="card variant-ghost-warning">
		<header class="card-header">
			<h3 class="h4">🚀 Next Steps for Phase 2</h3>
		</header>
		<div class="card-body">
			<p class="mb-4">
				Phase 1 is complete! The graph visualization is working with theme integration. 
				Phase 2 will connect this to your backend reputation system.
			</p>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
				<div>
					<h5 class="font-semibold mb-2">Backend Tasks:</h5>
					<ul class="list">
						<li>Create `/api/graph/reputation` endpoint</li>
						<li>Process vote data into graph format</li>
						<li>Add filtering by time/topic/user</li>
						<li>Implement graph caching</li>
					</ul>
				</div>
				<div>
					<h5 class="font-semibold mb-2">Frontend Enhancements:</h5>
					<ul class="list">
						<li>Add data loading states</li>
						<li>Implement search functionality</li>
						<li>Add graph export options</li>
						<li>Performance optimizations</li>
					</ul>
				</div>
			</div>
		</div>
	</section>
</div> 