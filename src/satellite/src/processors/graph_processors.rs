/*!
 * Graph Data Processors
 * 
 * This module handles graph visualization data generation for Sigma.js frontend.
 * It processes vote data into nodes (users) and edges (relationships) suitable
 * for interactive graph visualization.
 * 
 * # Features
 * - Single entry point with flexible query types
 * - Fetches votes using efficient key-based queries  
 * - Groups bidirectional relationships for cleaner visualization
 * - Fetches user data for node labels and avatars
 * - Supports tag-specific, user-specific, and dashboard views
 * 
 * # Graph Structure
 * - Nodes: Users with reputation-based sizing
 * - Edges: Vote relationships with smart visual encoding:
 *   - Straight edges = positive sentiment
 *   - Curved edges = negative sentiment  
 *   - Double-ended arrows = mutual relationships (same sentiment both ways)
 *   - Single arrows = one-way relationships or conflicting sentiments
 */

use junobuild_utils::decode_doc_data;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use candid::CandidType;

use crate::utils::structs::{VoteData, UserData, ReputationData};
use crate::processors::document_queries::query_doc_by_key;
use crate::logger;

/// Graph data structure for Sigma.js visualization
#[derive(Serialize, Deserialize, Clone, Debug, CandidType)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Graph node representing a user
#[derive(Serialize, Deserialize, Clone, Debug, CandidType)]
pub struct GraphNode {
    pub ulid: String,                  // user ULID
    pub label: String,                 // user handle
    pub avatar_url: Option<String>,    // user avatar URL (optional)
    pub reputation: Option<f64>,       // reputation score for sizing (optional)
}

/// Graph edge representing a vote relationship with smart visual encoding
#[derive(Serialize, Deserialize, Clone, Debug, CandidType)]
pub struct GraphEdge {
    pub source: String,                // primary voter ULID
    pub target: String,                // primary target ULID
    pub weight: f64,                   // combined vote count (for thickness)
    pub vote_value: i32,               // +1 or -1 (for curve: positive=straight, negative=curved)
    pub is_bidirectional: bool,        // true = double-ended arrow, false = single arrow
    pub source_count: u32,             // votes from source to target
    pub target_count: Option<u32>,     // votes from target to source (if bidirectional)
    pub tag_ulid: Option<String>,      // tag for dashboard coloring
}

/// Internal structure for analyzing vote relationships
#[derive(Clone, Debug)]
struct VoteRelationship {
    user1: String,
    user2: String,
    user1_to_user2: Option<(i32, u32)>, // (vote_value, count)
    user2_to_user1: Option<(i32, u32)>, // (vote_value, count)
    tag_ulid: String,
}

/// Main entry point for graph data generation
/// 
/// # Arguments
/// * `ulid` - The identifier (tag ULID, user ULID, or empty for "all")
/// * `query_type` - The query type: "tag", "user", or "all"
/// 
/// # Returns
/// * `Result<GraphData, String>` - Graph data ready for visualization
pub async fn get_graph_data(ulid: String, query_type: String) -> Result<GraphData, String> {
    logger!("debug", "[get_graph_data] Query: type={}, ulid={}", query_type, ulid);
    
    // Fetch votes based on query type
    let votes = fetch_votes_by_type(&ulid, &query_type).await?;
    logger!("info", "[get_graph_data] Found {} votes for type={}", votes.len(), query_type);
    
    // Process votes into smart edges
    let edges = process_votes_into_smart_edges(votes)?;
    
    // Build complete graph data
    let tag_ulid = if query_type == "tag" { Some(ulid) } else { None };
    let graph_data = build_graph_from_edges(edges, tag_ulid).await?;
    
    logger!("info", "[get_graph_data] Generated graph: {} nodes, {} edges", 
        graph_data.nodes.len(), graph_data.edges.len());
    
    Ok(graph_data)
}

/// Fetches votes based on query type using key patterns
async fn fetch_votes_by_type(ulid: &str, query_type: &str) -> Result<Vec<VoteData>, String> {
    let key_pattern = match query_type {
        "tag" => {
            if ulid.is_empty() {
                return Err("Tag ULID cannot be empty for tag query".to_string());
            }
            format!("tag_{}_", ulid)
        },
        "user" => {
            if ulid.is_empty() {
                return Err("User ULID cannot be empty for user query".to_string());
            }
            format!("_{}_", ulid)
        },
        "all" => {
            String::new() // Empty pattern fetches all documents
        },
        _ => {
            return Err(format!("Invalid query type: {}. Use 'tag', 'user', or 'all'", query_type));
        }
    };
    
    logger!("debug", "[fetch_votes_by_type] Using key pattern: '{}'", key_pattern);
    
    let votes_result = query_doc_by_key("votes", &key_pattern)
        .map_err(|e| format!("Failed to fetch votes: {}", e))?;
    
    extract_vote_data(votes_result.items.into_iter().map(|(_, doc)| doc).collect())
}

/// Extracts vote data from Juno documents
fn extract_vote_data(docs: Vec<junobuild_satellite::Doc>) -> Result<Vec<VoteData>, String> {
    let mut votes = Vec::new();
    
    for doc in docs {
        let vote_data: VoteData = decode_doc_data(&doc.data)
            .map_err(|e| format!("Failed to decode vote data: {}", e))?;
        votes.push(vote_data);
    }
    
    Ok(votes)
}

/// Processes votes with smart bidirectional grouping for cleaner visualization
fn process_votes_into_smart_edges(votes: Vec<VoteData>) -> Result<Vec<GraphEdge>, String> {
    // Step 1: Group votes by user pairs (unordered) and direction within each pair
    let mut pair_votes: HashMap<(String, String), HashMap<(String, String, i32), Vec<VoteData>>> = HashMap::new();
    
    for vote in votes {
        if vote.owner_ulid == vote.target_ulid {
            // Skip self-votes
            logger!("debug", "[process_votes_into_smart_edges] Skipping self-vote: {}", vote.owner_ulid);
            continue;
        }
        
        // Create ordered pair (smaller ULID first for consistent grouping)
        let user_pair = if vote.owner_ulid < vote.target_ulid {
            (vote.owner_ulid.clone(), vote.target_ulid.clone())
        } else {
            (vote.target_ulid.clone(), vote.owner_ulid.clone())
        };
        
        // Group by (source, target, sign) within each pair
        let direction_key = (vote.owner_ulid.clone(), vote.target_ulid.clone(), vote.value as i32);
        
        pair_votes
            .entry(user_pair)
            .or_insert_with(HashMap::new)
            .entry(direction_key)
            .or_insert_with(Vec::new)
            .push(vote);
    }
    
    // Step 2: Analyze each user pair for relationship patterns
    let relationships = analyze_vote_relationships(pair_votes)?;
    
    // Step 3: Convert relationships to smart edges
    convert_relationships_to_edges(relationships)
}

/// Analyzes vote relationships between user pairs
fn analyze_vote_relationships(
    pair_votes: HashMap<(String, String), HashMap<(String, String, i32), Vec<VoteData>>>
) -> Result<Vec<VoteRelationship>, String> {
    let mut relationships = Vec::new();
    
    for ((user1, user2), directions) in pair_votes {
        let mut user1_to_user2: Option<(i32, u32)> = None;
        let mut user2_to_user1: Option<(i32, u32)> = None;
        let mut tag_ulid = String::new();
        
        for ((source, target, vote_sign), vote_list) in directions {
            let vote_count = vote_list.len() as u32;
            let normalized_value = if vote_sign > 0 { 1 } else { -1 };
            
            // Set tag from first vote
            if tag_ulid.is_empty() {
                tag_ulid = vote_list[0].tag_ulid.clone();
            }
            
            if source == user1 && target == user2 {
                user1_to_user2 = Some((normalized_value, vote_count));
            } else if source == user2 && target == user1 {
                user2_to_user1 = Some((normalized_value, vote_count));
            }
        }
        
        relationships.push(VoteRelationship {
            user1: user1.clone(),
            user2: user2.clone(),
            user1_to_user2,
            user2_to_user1,
            tag_ulid,
        });
    }
    
    Ok(relationships)
}

/// Converts vote relationships to smart graph edges
fn convert_relationships_to_edges(relationships: Vec<VoteRelationship>) -> Result<Vec<GraphEdge>, String> {
    let mut edges = Vec::new();
    let relationship_count = relationships.len();
    
    for rel in relationships {
        match (rel.user1_to_user2, rel.user2_to_user1) {
            // Bidirectional with same sentiment - create single double-ended edge
            (Some((val1, count1)), Some((val2, count2))) if val1 == val2 => {
                edges.push(GraphEdge {
                    source: rel.user1.clone(),
                    target: rel.user2.clone(),
                    weight: (count1 + count2) as f64,
                    vote_value: val1, // Same sentiment
                    is_bidirectional: true,
                    source_count: count1,
                    target_count: Some(count2),
                    tag_ulid: Some(rel.tag_ulid.clone()),
                });
                
                logger!("debug", "[convert_relationships_to_edges] Mutual {} relationship: {}↔{} (weights: {}, {})", 
                    if val1 > 0 { "positive" } else { "negative" }, 
                    rel.user1, rel.user2, count1, count2);
            },
            
            // Bidirectional with different sentiments - create two separate edges  
            (Some((val1, count1)), Some((val2, count2))) => {
                edges.push(GraphEdge {
                    source: rel.user1.clone(),
                    target: rel.user2.clone(),
                    weight: count1 as f64,
                    vote_value: val1,
                    is_bidirectional: false,
                    source_count: count1,
                    target_count: None,
                    tag_ulid: Some(rel.tag_ulid.clone()),
                });
                
                edges.push(GraphEdge {
                    source: rel.user2.clone(),
                    target: rel.user1.clone(),
                    weight: count2 as f64,
                    vote_value: val2,
                    is_bidirectional: false,
                    source_count: count2,
                    target_count: None,
                    tag_ulid: Some(rel.tag_ulid.clone()),
                });
                
                logger!("debug", "[convert_relationships_to_edges] Conflicting relationship: {}→{} ({}) and {}→{} ({})", 
                    rel.user1, rel.user2, if val1 > 0 { "positive" } else { "negative" },
                    rel.user2, rel.user1, if val2 > 0 { "positive" } else { "negative" });
            },
            
            // One-way relationships - create single directional edges
            (Some((val1, count1)), None) => {
                edges.push(GraphEdge {
                    source: rel.user1.clone(),
                    target: rel.user2.clone(),
                    weight: count1 as f64,
                    vote_value: val1,
                    is_bidirectional: false,
                    source_count: count1,
                    target_count: None,
                    tag_ulid: Some(rel.tag_ulid.clone()),
                });
                
                logger!("debug", "[convert_relationships_to_edges] One-way relationship: {}→{} ({})", 
                    rel.user1, rel.user2, if val1 > 0 { "positive" } else { "negative" });
            },
            
            (None, Some((val2, count2))) => {
                edges.push(GraphEdge {
                    source: rel.user2.clone(),
                    target: rel.user1.clone(),
                    weight: count2 as f64,
                    vote_value: val2,
                    is_bidirectional: false,
                    source_count: count2,
                    target_count: None,
                    tag_ulid: Some(rel.tag_ulid.clone()),
                });
                
                logger!("debug", "[convert_relationships_to_edges] One-way relationship: {}→{} ({})", 
                    rel.user2, rel.user1, if val2 > 0 { "positive" } else { "negative" });
            },
            
            // No votes (shouldn't happen)
            (None, None) => {
                logger!("debug", "[convert_relationships_to_edges] No votes found for pair: {} - {}", 
                    rel.user1, rel.user2);
            }
        }
    }
    
    logger!("info", "[convert_relationships_to_edges] Created {} smart edges from {} relationships", 
        edges.len(), relationship_count);
    
    Ok(edges)
}

/// Builds complete graph data from processed edges
async fn build_graph_from_edges(
    edges: Vec<GraphEdge>, 
    single_tag: Option<String>
) -> Result<GraphData, String> {
    // Extract unique user ULIDs from edges
    let mut user_ulids = std::collections::HashSet::new();
    for edge in &edges {
        user_ulids.insert(edge.source.clone());
        user_ulids.insert(edge.target.clone());
    }
    
    logger!("debug", "[build_graph_from_edges] Found {} unique users in edge data", user_ulids.len());
    
    // Fetch user data for all participants
    let mut nodes = Vec::new();
    for user_ulid in user_ulids {
        match fetch_user_node_data(&user_ulid, single_tag.as_ref()).await {
            Ok(node) => nodes.push(node),
            Err(e) => {
                logger!("debug", "[build_graph_from_edges] Could not fetch user {}: {}", user_ulid, e);
                // Create placeholder node for missing users
                nodes.push(GraphNode {
                    ulid: user_ulid.clone(),
                    label: format!("User {}", &user_ulid[0..8]), // Show first 8 chars of ULID
                    avatar_url: Some(String::new()),
                    reputation: Some(1.0), // Default reputation
                });
            }
        }
    }
    
    logger!("info", "[build_graph_from_edges] Built graph: {} nodes, {} edges", nodes.len(), edges.len());
    
    Ok(GraphData { nodes, edges })
}

/// Fetches user data and creates graph node
async fn fetch_user_node_data(
    user_ulid: &str, 
    tag_ulid: Option<&String>
) -> Result<GraphNode, String> {
    // Fetch user document
    let user_result = query_doc_by_key("users", &format!("usr_{}_", user_ulid))
        .map_err(|e| format!("Failed to query user {}: {}", user_ulid, e))?;
    
    if user_result.items.is_empty() {
        return Err(format!("User {} not found", user_ulid));
    }
    
    let (_, user_doc) = &user_result.items[0];
    let user_data: UserData = decode_doc_data(&user_doc.data)
        .map_err(|e| format!("Failed to decode user data: {}", e))?;
    
    // Get reputation if tag is specified
    let reputation = if let Some(tag) = tag_ulid {
        fetch_user_reputation(user_ulid, tag).await.unwrap_or(1.0)
    } else {
        1.0 // Default reputation for cross-tag views
    };
    
    Ok(GraphNode {
        ulid: user_ulid.to_string(),
        label: user_data.user_handle,
        avatar_url: Some(user_data.avatar_url),
        reputation: Some(reputation),
    })
}

/// Fetches user reputation for a specific tag
async fn fetch_user_reputation(user_ulid: &str, tag_ulid: &str) -> Result<f64, String> {
    let reputation_key = format!("usr_{}_tag_{}_", user_ulid, tag_ulid);
    let reputation_result = query_doc_by_key("reputations", &reputation_key)
        .map_err(|e| format!("Failed to query reputation: {}", e))?;
    
    if reputation_result.items.is_empty() {
        return Ok(1.0); // Default reputation
    }
    
    let (_, reputation_doc) = &reputation_result.items[0];
    let reputation_data: ReputationData = decode_doc_data(&reputation_doc.data)
        .map_err(|e| format!("Failed to decode reputation data: {}", e))?;
    
    Ok(reputation_data.reputation_total_effective)
}

/// Generates consistent color for tag (simple hash-based approach)
fn generate_tag_color(tag_ulid: &str) -> String {
    // Simple hash to color mapping - replace with better algorithm if needed
    let hash = tag_ulid.chars().map(|c| c as u32).sum::<u32>();
    let colors = [
        "#ff6b6b", "#4ecdc4", "#45b7d1", "#96ceb4", "#feca57",
        "#ff9ff3", "#54a0ff", "#5f27cd", "#00d2d3", "#ff9f43",
        "#c44569", "#f8b500", "#38ada9", "#6c5ce7", "#a29bfe"
    ];
    colors[(hash as usize) % colors.len()].to_string()
} 