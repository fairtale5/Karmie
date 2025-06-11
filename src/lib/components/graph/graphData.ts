/**
 * Dummy graph data generator for Phase 1 testing
 * 
 * HOW: Creates sample reputation network data with users, topics, and vote relationships
 * - Generates nodes representing users with reputation scores
 * - Creates edges representing voting relationships 
 * - Uses color coding for different node types and vote types
 * - Provides realistic data structure for Sigma.js consumption
 */
import Graph from 'graphology';

export interface GraphNode {
	id: string;
	label: string;
	x: number; // Required - starts with random, ForceAtlas2 improves positioning
	y: number; // Required - starts with random, ForceAtlas2 improves positioning
	size: number;
	color: string;
	type: 'circle'; // Using supported Sigma.js node type
	nodeCategory: 'user' | 'topic'; // Custom property for our logic
	reputation?: number;
	description?: string;
}

export interface GraphEdge {
	id: string;
	source: string;
	target: string;
	weight: number;
	color: string;
	type: 'arrow' | 'line'; // Using supported Sigma.js edge types
	edgeCategory: 'upvote' | 'downvote' | 'contributes_to'; // Custom property for our logic
	label?: string;
}

/**
 * HOW: Generates themed colors using hex equivalents of your repu-crimson theme
 * - Uses secondary colors for nodes (dots) and tertiary for edges (lines) 
 * - Converts your OKLCH theme colors to hex values for Sigma.js compatibility
 * - Provides both light and dark mode appropriate colors
 * - Returns colors compatible with Sigma.js rendering engine
 */
function getThemeColors() {
	// HOW: Debug what CSS custom properties return
	if (typeof window !== 'undefined') {
		const style = getComputedStyle(document.documentElement);
		console.log('CSS Property Debug:', {
			secondary300: style.getPropertyValue('--color-secondary-300').trim(),
			secondary500: style.getPropertyValue('--color-secondary-500').trim(),
			secondary700: style.getPropertyValue('--color-secondary-700').trim()
		});
	}
	
	// HOW: Detect current theme mode for proper color selection
	const isDarkMode = typeof window !== 'undefined' 
		? document.documentElement.getAttribute('data-mode') === 'dark'
		: false;
	
	if (isDarkMode) {
		// Dark mode: Use brighter variants of your theme colors for visibility
		return {
			nodeColors: {
				light: '#9d8df1',    // Brighter secondary-300 equivalent 
				medium: '#8b7cf6',   // Brighter secondary-500 equivalent
				strong: '#7c6aef'    // Brighter secondary-700 equivalent
			},
			edgeColors: {
				light: '#f472b6',    // Brighter tertiary-300 equivalent
				medium: '#ec4899',   // Brighter tertiary-500 equivalent  
				strong: '#db2777',   // Brighter tertiary-700 equivalent
				success: '#4ade80',  // Success green for upvotes
				error: '#f87171'     // Error red for downvotes
			}
		};
	} else {
		// Light mode: Use your repu-crimson theme colors converted to hex
		return {
			nodeColors: {
				light: '#a78bfa',    // secondary-300 hex equivalent
				medium: '#8b5cf6',   // secondary-500 hex equivalent  
				strong: '#7c3aed'    // secondary-700 hex equivalent
			},
			edgeColors: {
				light: '#f9a8d4',    // tertiary-300 hex equivalent
				medium: '#ec4899',   // tertiary-500 hex equivalent
				strong: '#be185d',   // tertiary-700 hex equivalent
				success: '#22c55e',  // Success green for upvotes
				error: '#ef4444'     // Error red for downvotes
			}
		};
	}
}

/**
 * HOW: Creates a populated Graphology instance with dummy reputation data
 * - Users represented as nodes with reputation scores determining size
 * - Topics represented as larger nodes that users contribute to
 * - Edges show voting relationships and topic contributions
 * - Random positioning for natural ForceAtlas2 layout starting point
 */
export function createDummyGraph(): Graph {
	const graph = new Graph({ type: 'directed' });
	const colors = getThemeColors();

	// HOW: Define sample users with varying reputation levels
	const users = [
		{ id: 'alice', label: 'Alice Chen', reputation: 95, desc: 'Senior Developer & Open Source Contributor' },
		{ id: 'bob', label: 'Bob Martinez', reputation: 78, desc: 'Data Scientist & ML Engineer' },
		{ id: 'carol', label: 'Carol Kim', reputation: 62, desc: 'Frontend Specialist & UX Advocate' },
		{ id: 'david', label: 'David Singh', reputation: 41, desc: 'Junior Developer & Quick Learner' },
		{ id: 'eva', label: 'Eva Thompson', reputation: 87, desc: 'DevOps Engineer & Security Expert' },
		{ id: 'frank', label: 'Frank Liu', reputation: 33, desc: 'New Graduate & Eager Contributor' },
		{ id: 'grace', label: 'Grace Johnson', reputation: 71, desc: 'Product Manager & Technical Writer' },
		{ id: 'henry', label: 'Henry Brown', reputation: 89, desc: 'Architect & System Designer' }
	];

	// HOW: No topic nodes needed - your system only has users and votes between them

	// HOW: Add user nodes with reputation-based sizing and coloring using secondary colors
	// Note: Initial random positions required for Sigma.js rendering, ForceAtlas2 improves them
	users.forEach(user => {
		const nodeSize = Math.max(8, user.reputation / 8); // Size scales with reputation
		
		// HOW: Choose secondary color shade based on reputation level
		let nodeColor;
		if (user.reputation >= 80) {
			nodeColor = colors.nodeColors.strong; // High reputation = strong secondary
		} else if (user.reputation >= 60) {
			nodeColor = colors.nodeColors.medium; // Medium reputation = medium secondary
		} else {
			nodeColor = colors.nodeColors.light;  // Lower reputation = light secondary
		}
		
		graph.addNode(user.id, {
			label: user.label,
			// Initial random positions - ForceAtlas2 will improve these based on connections
			x: (Math.random() - 0.5) * 100,
			y: (Math.random() - 0.5) * 100,
			size: nodeSize,
			color: nodeColor,
			type: 'circle', // Using supported Sigma.js node type
			nodeCategory: 'user', // Custom property for our logic
			reputation: user.reputation,
			description: user.desc
		});
	});

	// HOW: Only user nodes - no topics needed in your reputation system

	// HOW: Create realistic voting relationships between users (matching your votes schema)
	const voteRelationships = [
		// Positive votes (value: +1)
		{ from: 'alice', to: 'bob', type: 'upvote', value: 1 },
		{ from: 'alice', to: 'eva', type: 'upvote', value: 1 },
		{ from: 'bob', to: 'alice', type: 'upvote', value: 1 },
		{ from: 'carol', to: 'alice', type: 'upvote', value: 1 },
		{ from: 'david', to: 'alice', type: 'upvote', value: 1 },
		{ from: 'david', to: 'grace', type: 'upvote', value: 1 },
		{ from: 'eva', to: 'henry', type: 'upvote', value: 1 },
		{ from: 'frank', to: 'david', type: 'upvote', value: 1 },
		{ from: 'grace', to: 'carol', type: 'upvote', value: 1 },
		{ from: 'henry', to: 'eva', type: 'upvote', value: 1 },
		{ from: 'bob', to: 'carol', type: 'upvote', value: 1 },
		{ from: 'eva', to: 'alice', type: 'upvote', value: 1 },
		{ from: 'grace', to: 'henry', type: 'upvote', value: 1 },
		{ from: 'carol', to: 'david', type: 'upvote', value: 1 },
		
		// Negative votes (value: -1) 
		{ from: 'henry', to: 'frank', type: 'downvote', value: -1 },
		{ from: 'frank', to: 'henry', type: 'downvote', value: -1 },
		{ from: 'alice', to: 'frank', type: 'downvote', value: -1 }
	];

	// HOW: Add voting edges using success/error colors (matching your votes schema)
	voteRelationships.forEach(rel => {
		const edgeId = `${rel.from}-${rel.type}-${rel.to}`;
		
		// HOW: Use success for upvotes, error for downvotes
		let edgeColor;
		if (rel.type === 'upvote') {
			edgeColor = colors.edgeColors.success; // Success green for upvotes
		} else {
			edgeColor = colors.edgeColors.error;   // Error red for downvotes  
		}
		
		graph.addEdge(rel.from, rel.to, {
			id: edgeId,
			weight: 1.0, // Default weight as per your schema
			color: edgeColor,
			type: 'arrow', // Using supported Sigma.js edge type
			edgeCategory: rel.type as 'upvote' | 'downvote', // Custom property for our logic
			voteValue: rel.value, // Store the actual vote value (+1 or -1)
			label: `${rel.type} (${rel.value > 0 ? '+' : ''}${rel.value})`
		});
	});

	// HOW: No contribution relationships - your system only has votes between users

	return graph;
}

/**
 * HOW: Export interface for the complete graph structure
 * Used by Sigma.js component for proper typing and data validation
 */
export interface GraphData {
	nodes: GraphNode[];
	edges: GraphEdge[];
}

/**
 * HOW: Converts Graphology instance to plain object format
 * Useful for serialization, debugging, or alternative graph libraries
 */
export function exportGraphData(graph: Graph): GraphData {
	const nodes: GraphNode[] = [];
	const edges: GraphEdge[] = [];

	// Extract nodes with all attributes
	graph.forEachNode((node, attributes) => {
		nodes.push({
			id: node,
			...attributes
		} as GraphNode);
	});

	// Extract edges with all attributes  
	graph.forEachEdge((edge, attributes, source, target) => {
		edges.push({
			id: edge,
			source,
			target,
			...attributes
		} as GraphEdge);
	});

	return { nodes, edges };
}

// HOW: Example of handling duplicate/bidirectional votes in reputation systems

/**
 * HOW: Utility to process raw vote data and handle duplicates
 * - Merges multiple votes between same users in same direction
 * - Preserves bidirectional relationships as separate edges
 * - Returns clean data ready for graph visualization
 */
export function processVoteData(rawVotes: Array<{from: string, to: string, value: number}>): Array<{from: string, to: string, value: number, count: number}> {
	const voteMap = new Map<string, {value: number, count: number}>();
	
	rawVotes.forEach(vote => {
		const key = `${vote.from}->${vote.to}`;
		
		if (voteMap.has(key)) {
			const existing = voteMap.get(key)!;
			existing.value += vote.value; // Sum vote values
			existing.count += 1; // Count occurrences  
		} else {
			voteMap.set(key, { value: vote.value, count: 1 });
		}
	});
	
	return Array.from(voteMap.entries()).map(([key, data]) => {
		const [from, to] = key.split('->');
		return { from, to, value: data.value, count: data.count };
	});
}

/**
 * EXAMPLE: How different vote scenarios are handled
 * 
 * Raw Backend Data:
 * [
 *   {from: 'A', to: 'B', value: 1},  // A upvotes B
 *   {from: 'A', to: 'B', value: 1},  // A upvotes B again (duplicate)
 *   {from: 'B', to: 'A', value: 1},  // B upvotes A (bidirectional)
 *   {from: 'A', to: 'B', value: -1}, // A downvotes B (conflicting)
 * ]
 * 
 * After processVoteData():
 * [
 *   {from: 'A', to: 'B', value: 1, count: 3},  // Net +1 from 3 votes
 *   {from: 'B', to: 'A', value: 1, count: 1}   // Separate bidirectional
 * ]
 * 
 * Visual Result:
 * - ONE arrow A → B (green, weight=1, shows "3 votes") 
 * - ONE arrow B → A (green, weight=1, shows "1 vote")
 */ 