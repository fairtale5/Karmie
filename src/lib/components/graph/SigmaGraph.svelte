<script lang="ts">
	/**
	 * SigmaGraph.svelte - Interactive graph visualization component
	 * 
	 * HOW: Integrates Sigma.js with SvelteKit and the existing theme system
	 * - Renders graph using WebGL for performance with large datasets
	 * - Applies ForceAtlas2 layout algorithm for natural node positioning
	 * - Responds to theme changes by updating node/edge colors
	 * - Provides interactivity: hover, click, drag, zoom
	 * - Integrates with Skeleton UI design system
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import Sigma from 'sigma';
	import Graph from 'graphology';
	import FA2Layout from 'graphology-layout-forceatlas2';
	import { createDummyGraph } from './graphData';
	import { themeStore } from '$lib/stores/theme';

	// Component props for customization
	export let width: string = '100%';
	export let height: string = '600px';
	export let showControls: boolean = true;
	export let graphData: any = null; // External graph data from backend
	export let loading: boolean = false; // Loading state

	// Component state
	let containerElement: HTMLDivElement;
	let sigmaInstance: Sigma | null = null;
	let graph: Graph;
	let layoutRunning = false;
	let selectedNode: string | null = null;

	// HOW: Reactive theme integration - updates colors when theme changes
	$: currentTheme = $themeStore;
	
	// HOW: Reactive statement to update theme when store changes
	$: if (sigmaInstance && currentTheme) {
		updateGraphTheme();
		updateLabelColors();
	}
	
	/**
	 * HOW: Updates label colors to match current theme
	 * - Uses your theme's surface colors for proper contrast
	 * - Automatically switches between light/dark mode colors
	 */
	function updateLabelColors() {
		if (!sigmaInstance) return;
		
		// HOW: Get theme-appropriate text color using your CSS variables
		const style = getComputedStyle(document.documentElement);
		const isDarkMode = document.documentElement.getAttribute('data-mode') === 'dark';
		
		// HOW: Use hex colors that work reliably with Sigma.js
		const textColor = isDarkMode 
			? '#f5f5f5'  // Light text for dark mode
			: '#1a1a1a'; // Dark text for light mode
		
		// HOW: Update Sigma.js label renderer settings
		sigmaInstance.setSetting('labelColor', { color: textColor });
		sigmaInstance.refresh();
	}
	$: if (sigmaInstance && browser) {
		updateGraphTheme();
	}

	/**
	 * HOW: Updates graph colors when theme changes
	 * - Preserves node positions and uses current data source
	 * - Updates colors based on theme
	 * - Triggers Sigma.js re-render to show updated colors
	 */
	function updateGraphTheme() {
		if (!graph || !sigmaInstance) return;

		// HOW: Preserve current node positions during theme change
		const oldPositions: { [key: string]: { x: number; y: number } } = {};
		graph.forEachNode((node, attributes) => {
			oldPositions[node] = { x: attributes.x, y: attributes.y };
		});
		
		// HOW: Create new graph with current theme colors using same data source
		const newGraph = createGraphFromData(graphData);
		
		// HOW: Clear and rebuild graph with preserved positions
		graph.clear();
		newGraph.forEachNode((node, attributes) => {
			// Preserve layout positions if they exist
			if (oldPositions[node]) {
				attributes.x = oldPositions[node].x;
				attributes.y = oldPositions[node].y;
			}
			graph.addNode(node, attributes);
		});
		
		newGraph.forEachEdge((edge, attributes, source, target) => {
			graph.addEdge(source, target, attributes);
		});

		// HOW: Trigger Sigma.js re-render with new colors
		sigmaInstance.refresh();
		console.log('Graph colors updated for theme change');
	}

	/**
	 * HOW: Applies ForceAtlas2 layout algorithm to position nodes naturally
	 * - Uses physics simulation to position nodes based on connections
	 * - Runs for specified iterations to reach stable layout
	 * - Updates component state to show layout progress
	 * - Called automatically on initial load and manually via button
	 */
	function runLayout() {
		if (!graph || layoutRunning) return;
		
		layoutRunning = true;
		
		// HOW: Apply ForceAtlas2 algorithm with balanced settings
		FA2Layout.assign(graph, {
			iterations: 150,
			settings: {
				gravity: 1,
				scalingRatio: 10,
				strongGravityMode: false,
				adjustSizes: true,
				edgeWeightInfluence: 1,
				barnesHutOptimize: false // For smaller graphs, exact computation is fine
			}
		});
		
		layoutRunning = false;
		if (sigmaInstance) {
			sigmaInstance.refresh();
		}
	}

	/**
	 * HOW: Centers the graph view and resets zoom level
	 * - Uses Sigma.js built-in reset to show all nodes optimally
	 * - Animates camera to show entire graph with proper centering
	 * - Provides consistent view regardless of layout changes
	 */
	function centerGraph() {
		if (!sigmaInstance) return;
		
		// HOW: Use Sigma.js camera reset to center on all nodes
		const camera = sigmaInstance.getCamera();
		camera.animatedReset({ duration: 500 });
	}

	/**
	 * HOW: Handles node click interactions
	 * - Updates selected node state for highlighting
	 * - Could trigger sidebar panels, modals, or navigation
	 * - Provides foundation for deeper graph exploration
	 */
	function handleNodeClick(event: { node: string }) {
		selectedNode = selectedNode === event.node ? null : event.node;
		console.log('Node clicked:', {
			node: event.node,
			attributes: graph.getNodeAttributes(event.node)
		});
	}

	/**
	 * HOW: Handles edge click interactions  
	 * - Shows relationship details
	 * - Could highlight related nodes or show edge properties
	 * - Enables exploration of connections between entities
	 */
	function handleEdgeClick(event: { edge: string }) {
		const edgeData = graph.getEdgeAttributes(event.edge);
		console.log('Edge clicked:', {
			edge: event.edge,
			edgeCategory: edgeData.edgeCategory,
			label: edgeData.label,
			weight: edgeData.weight,
			source: graph.source(event.edge),
			target: graph.target(event.edge)
		});
	}

	/**
	 * HOW: Creates graph from external data or dummy data
	 * - Converts backend GraphData to Graphology format
	 * - Implements smart edge rendering: curved=negative, straight=positive, double-ended=mutual
	 * - Uses theme colors and proper node sizing
	 */
	function createGraphFromData(data: any = null): Graph {
		const graph = new Graph({ type: 'directed' });
		
		// Use external data if provided, otherwise fall back to dummy data
		const sourceData = data || createDummyGraph();
		
		if (data) {
			// Convert backend data to Sigma.js format
			data.nodes.forEach((node: any) => {
				graph.addNode(node.ulid, {
					label: node.label,
					// Initial random positions - ForceAtlas2 will improve these based on connections
					x: (Math.random() - 0.5) * 100,
					y: (Math.random() - 0.5) * 100,
					size: Math.max(8, node.reputation * 8), // Size based on reputation
					color: getNodeColor(node.reputation),
					type: 'circle',
					nodeCategory: 'user',
					reputation: node.reputation,
					avatar_url: node.avatar_url
				});
			});
			
			data.edges.forEach((edge: any) => {
				const edgeId = `${edge.source}-${edge.target}-${edge.vote_value}`;
				
				graph.addEdge(edge.source, edge.target, {
					id: edgeId,
					weight: edge.weight,
					color: getEdgeColor(edge.vote_value),
					// Use valid Sigma.js edge types: 'arrow' for all directed edges
					type: 'arrow',
					size: Math.max(1, edge.weight / 2), // Thickness based on vote count
					vote_value: edge.vote_value,
					vote_count: edge.source_count,
					is_bidirectional: edge.is_bidirectional,
					// Visual indicators in the label for now
					label: `${edge.vote_value > 0 ? '+' : '-'}${edge.source_count}${edge.target_count ? `↔${edge.target_count}` : ''}`
				});
			});
		} else {
			// Use dummy data (existing createDummyGraph logic)
			return createDummyGraph();
		}
		
		return graph;
	}
	
	/**
	 * HOW: Gets node color based on reputation level
	 */
	function getNodeColor(reputation: number): string {
		const colors = getThemeColors();
		if (reputation >= 80) {
			return colors.nodeColors.strong;
		} else if (reputation >= 60) {
			return colors.nodeColors.medium;
		} else {
			return colors.nodeColors.light;
		}
	}
	
	/**
	 * HOW: Gets edge color based on vote value (positive/negative)
	 */
	function getEdgeColor(voteValue: number): string {
		const colors = getThemeColors();
		return voteValue > 0 ? colors.edgeColors.success : colors.edgeColors.error;
	}
	
	/**
	 * HOW: Gets themed colors (moved from graphData.ts)
	 */
	function getThemeColors() {
		const isDarkMode = typeof window !== 'undefined' 
			? document.documentElement.getAttribute('data-mode') === 'dark'
			: false;
		
		if (isDarkMode) {
			return {
				nodeColors: {
					light: '#9d8df1',
					medium: '#8b7cf6', 
					strong: '#7c6aef'
				},
				edgeColors: {
					success: '#4ade80',
					error: '#f87171'
				}
			};
		} else {
			return {
				nodeColors: {
					light: '#a78bfa',
					medium: '#8b5cf6',
					strong: '#7c3aed'
				},
				edgeColors: {
					success: '#22c55e',
					error: '#ef4444'
				}
			};
		}
	}

	/**
	 * HOW: Main component initialization
	 * - Creates graph data and Sigma.js instance
	 * - Sets up event listeners for interaction
	 * - Applies initial layout and theme
	 * - Only runs in browser environment
	 */
	onMount(() => {
		if (!browser || !containerElement) return;

		// HOW: Create graph with external data or dummy data
		graph = createGraphFromData(graphData);

		// HOW: Initialize Sigma.js renderer with theme-aware settings
		sigmaInstance = new Sigma(graph, containerElement, {
			// Rendering settings optimized for reputation graphs
			renderEdgeLabels: false, // Keep clean for better performance
			renderLabels: true,      // Show node labels
			defaultEdgeType: 'arrow',
			defaultNodeType: 'circle',
			minCameraRatio: 0.1,
			maxCameraRatio: 10,
			// HOW: Use hex color for initial label setup
			labelColor: { color: '#1a1a1a' },
			labelSize: 12,
			labelWeight: 'normal',
		});

		// HOW: Set up interaction event listeners
		sigmaInstance.on('clickNode', handleNodeClick);
		sigmaInstance.on('clickEdge', handleEdgeClick);
		
		// HOW: Apply initial layout automatically (proper pipeline)
		// ForceAtlas2 calculates x,y coordinates based on graph structure
		setTimeout(() => {
			runLayout();
			updateGraphTheme();
			// Center after layout completes
			setTimeout(centerGraph, 100);
		}, 100); // Small delay for DOM stability
	});

	// HOW: Watch for changes in graphData and recreate graph
	$: if (graphData && sigmaInstance) {
		// Preserve positions
		const oldPositions: { [key: string]: { x: number; y: number } } = {};
		graph.forEachNode((node, attributes) => {
			oldPositions[node] = { x: attributes.x, y: attributes.y };
		});

		// Recreate graph with new data
		graph = createGraphFromData(graphData);
		
		// Restore positions if nodes still exist
		graph.forEachNode((node, attributes) => {
			if (oldPositions[node]) {
				attributes.x = oldPositions[node].x;
				attributes.y = oldPositions[node].y;
			}
		});

		// Update Sigma instance
		sigmaInstance.setGraph(graph);
		sigmaInstance.refresh();
	}

	/**
	 * HOW: Cleanup when component is destroyed
	 * - Prevents memory leaks by properly disposing Sigma.js
	 * - Removes event listeners and releases WebGL context
	 * - Essential for SPA navigation and hot reloading
	 */
	onDestroy(() => {
		if (sigmaInstance) {
			sigmaInstance.kill();
			sigmaInstance = null;
		}
	});
</script>

<!-- 
Graph Container and Controls
HOW: Provides responsive container with optional control panel
- Container uses CSS Grid for flexible layout management
- Controls provide user interaction for graph manipulation
- Theme-aware styling integrates with Skeleton UI system
-->
<div class="graph-wrapper" style="height: {height};">
	{#if showControls}
		<div class="graph-controls">
			<div class="btn-group variant-ghost-surface">
				<button 
					class="btn btn-sm"
					class:variant-filled-primary={layoutRunning}
					disabled={layoutRunning}
					on:click={runLayout}
				>
					{layoutRunning ? 'Laying out...' : 'Re-apply Layout'}
				</button>
				<button 
					class="btn btn-sm"
					on:click={centerGraph}
				>
					Center View
				</button>
			</div>
			
			{#if selectedNode}
				<div class="selected-info">
					<span class="badge variant-filled-secondary">
						Selected: {graph?.getNodeAttribute(selectedNode, 'label')}
					</span>
				</div>
			{/if}
		</div>
	{/if}

	<!-- HOW: Main graph container with theme-aware styling -->
	<div 
		class="graph-container"
		style="width: {width}; height: {height};"
		bind:this={containerElement}
	/>
</div>

<style>
	/**
	 * HOW: Component styles that integrate with theme system
	 * - Uses CSS custom properties from Skeleton UI theme
	 * - Provides responsive layout for different screen sizes
	 * - Maintains visual consistency with rest of application
	 */
	.graph-wrapper {
		position: relative;
		display: flex;
		flex-direction: column;
		background: var(--color-surface-50);
		border: 1px solid var(--color-surface-300);
		border-radius: var(--radius-container);
		overflow: hidden;
	}

	/* HOW: Dark mode support using existing theme system */
	:global([data-mode="dark"]) .graph-wrapper {
		background: var(--color-surface-900);
		border-color: var(--color-surface-700);
	}

	.graph-controls {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: var(--color-surface-100);
		border-bottom: 1px solid var(--color-surface-300);
		gap: 1rem;
	}

	:global([data-mode="dark"]) .graph-controls {
		background: var(--color-surface-800);
		border-color: var(--color-surface-600);
	}

	.graph-container {
		flex: 1;
		position: relative;
		background: var(--color-surface-50);
	}

	:global([data-mode="dark"]) .graph-container {
		background: var(--color-surface-900);
	}

	.selected-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.btn-group {
		display: flex;
		gap: 0.5rem;
	}

	/* HOW: Responsive adjustments for mobile devices */
	@media (max-width: 640px) {
		.graph-controls {
			flex-direction: column;
			gap: 0.5rem;
		}
		
		.btn-group {
			justify-content: center;
		}
	}
</style> 