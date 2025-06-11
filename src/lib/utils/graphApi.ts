import { getSatelliteExtendedActor } from '@junobuild/core';
import { idlFactory } from '../../declarations/satellite/satellite.factory.did.js';

// Graph data types (will be generated automatically after backend is built)
export interface GraphNode {
	ulid: string;
	label: string;
	reputation?: number;
	avatar_url?: string | null;
}

export interface GraphEdge {
	source: string;
	target: string;
	weight: number;
	vote_value: number;
	is_bidirectional: boolean;
	source_count: number;
	target_count?: number | null;
	tag_ulid?: string | null;
}

export interface GraphData {
	nodes: GraphNode[];
	edges: GraphEdge[];
}

/**
 * Fetches graph data from the satellite backend
 * 
 * @param ulid - The ULID to query (tag or user ULID, or empty string for all)
 * @param queryType - Type of query: "tag", "user", or "all"
 * @returns Promise resolving to graph data with nodes and edges
 */
export async function fetchGraphData(ulid: string, queryType: 'tag' | 'user' | 'all'): Promise<GraphData> {
	console.log(`[fetchGraphData] Starting request - ULID: "${ulid}", Type: "${queryType}"`);
	
	try {
		// HOW: Use the same pattern as other satellite function calls in the codebase
		const { get_graph_data } = await getSatelliteExtendedActor<any>({
			idlFactory
		});

		console.log(`[fetchGraphData] Calling backend get_graph_data("${ulid}", "${queryType}")`);
		const result = await get_graph_data(ulid, queryType);
		console.log(`[fetchGraphData] Backend response:`, result);
		
		if ('Err' in result) {
			console.error(`[fetchGraphData] Backend error:`, result.Err);
			throw new Error(result.Err);
		}
		
		const graphData = result.Ok;
		console.log(`[fetchGraphData] Success - Nodes: ${graphData.nodes.length}, Edges: ${graphData.edges.length}`);
		return graphData;
	} catch (error) {
		console.error('[fetchGraphData] Failed to fetch graph data:', error);
		throw error;
	}
}

/**
 * HOW: Convenience function for fetching tag-specific graph data
 * - Uses the tag ULID to query votes within that tag context
 * - Useful for tag dashboard visualizations
 */
export function fetchTagGraphData(tagUlid: string): Promise<GraphData> {
	return fetchGraphData(tagUlid, 'tag');
}

/**
 * HOW: Convenience function for fetching user-specific graph data
 * - Uses user ULID to query all votes involving that user
 * - Shows both votes from and to the user
 */
export function fetchUserGraphData(userUlid: string): Promise<GraphData> {
	return fetchGraphData(userUlid, 'user');
}

/**
 * HOW: Convenience function for fetching all graph data
 * - Returns complete reputation network
 * - Useful for global dashboard views
 */
export function fetchAllGraphData(): Promise<GraphData> {
	return fetchGraphData('', 'all');
} 