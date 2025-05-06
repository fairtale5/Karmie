use ic_cdk; // Import ic_cdk
use junobuild_satellite::{list_docs, set_doc_store, get_doc, list_docs_store}; // Import junobuild_satellite functions
use junobuild_satellite::SetDoc; // why is this not in the previous import?
use junobuild_shared::types::list::{ListMatcher, ListParams}; // Import junobuild_shared types
use std::collections::HashMap; // Import std::collections::HashMap
use junobuild_utils::{encode_doc_data, decode_doc_data}; // Import junobuild_utils functions
use crate::logger; // Import our logger from the utils module 
use crate::utils::time::calculate_months_between; // Import time calculations
use crate::processors::document_keys::{create_reputation_key, format_reputation_key, format_tag_key};
use crate::processors::document_queries::query_doc_by_key;

// Import our data structures
use crate::utils::structs::{
    Tag, VoteData, Reputation, ReputationData, VoteWeight,
    AuthorInfo, TagData
};

// Import tag calculations
use crate::core::get_active_users_count;

/// Retrieves a user's cached reputation data for a specific tag.
///
/// Queries the reputations collection for a document with description 
/// formatted as: "owner=userKey;tag=tagKey;"
///
/// The function sanitizes all keys to remove special characters before building
/// the query pattern, ensuring consistent document retrieval.
///
/// # Arguments
/// * `user_key` - The user's document key
/// * `tag_key` - The tag's document key
///
/// # Returns
/// * `Result<Option<ReputationData>, String>` - The user's reputation data or None if not found, or an error message
pub async fn get_user_reputation_data(user_key: &str, tag_key: &str) -> Result<Option<ReputationData>, String> {
    // Create reputation key format: usr_{user_ulid}_tag_{tag_ulid}_
    let reputation_key = match format_reputation_key(user_key, tag_key) {
        Ok(key) => key,
        Err(e) => {
            logger!("error", "[get_user_reputation_data] Failed to format reputation key: user={}, tag={}, error={}", 
                user_key, tag_key, e);
            return Err(format!("Failed to format reputation key: {}", e));
        }
    };
    
    // Get the document directly by exact key match, which is more efficient
    // than querying by description field
    let reputation_doc = junobuild_satellite::get_doc(
        String::from("reputations"),
        reputation_key.clone()
    );
    
    // Return None if no document found
    if reputation_doc.is_none() {
        logger!("debug", "[get_user_reputation_data] No reputation document found for user={}, tag={}", 
            user_key, tag_key);
        return Ok(None);
    }
    
    // Unwrap the document (safe because we checked for None above)
    let doc = reputation_doc.unwrap();
    
    // Decode the reputation data from binary format
    let reputation_data = match decode_doc_data::<ReputationData>(&doc.data) {
        Ok(data) => data,
        Err(e) => {
            logger!("error", "[get_user_reputation_data] Failed to deserialize reputation data: key={}, error={}", 
                reputation_key, e);
            return Err(format!("Failed to deserialize reputation data: {}", e));
        }
    };
    
    logger!("debug", "[get_user_reputation_data] Retrieved reputation data for user={}, tag={}", 
        user_key, tag_key);
    
    Ok(Some(reputation_data))
}

/// Gets a slim version of user reputation data optimized for vote processing
///
/// This retrieves only the essential reputation data needed for vote processing:
/// - Current effective reputation
/// - Vote weight
/// - Voting power status
///
/// Optimized to use key-based queries for memory efficiency.
///
/// # Arguments
/// * `user_key` - The user's document key
/// * `tag_key` - The tag's document key
///
/// # Returns
/// * `Result<Option<AuthorInfo>, String>` - Slim reputation data or None if not found
pub async fn get_user_reputation_slim(user_key: &str, tag_key: &str) -> Result<Option<AuthorInfo>, String> {
    // Get active users count to check if we're in bootstrap phase
    let active_users = get_active_users_count(tag_key).await?;
    
    // Get tag to check threshold
    let tag = get_tag_doc(tag_key).await?;
    let in_bootstrap_phase = active_users < tag.data.min_users_for_threshold;
    
    // Create reputation key format: usr_{user_ulid}_tag_{tag_ulid}_
    let reputation_key = match format_reputation_key(user_key, tag_key) {
        Ok(key) => key,
        Err(e) => {
            logger!("error", "[get_user_reputation_slim] Failed to format reputation key: user={}, tag={}, error={}", 
                user_key, tag_key, e);
            return Err(format!("Failed to format reputation key: {}", e));
        }
    };
    
    logger!("debug", "[get_user_reputation_slim] Looking up reputation document with key: {}", reputation_key);
    
    // Get the document directly by exact key match
    let reputation_doc = junobuild_satellite::get_doc(
        String::from("reputations"),
        reputation_key.clone()
    );
    
    // Return None if no document found
    if reputation_doc.is_none() {
        logger!("debug", "[get_user_reputation_slim] No reputation document found for user={}, tag={}", 
            user_key, tag_key);
        return Ok(None);
    }
    
    // Unwrap the document (safe because we checked for None above)
    let doc = reputation_doc.unwrap();
    
    // Try to decode
    match decode_doc_data::<ReputationData>(&doc.data) {
        Ok(data) => {
            // During bootstrap phase, give everyone voting power for the purpose of processing votes
            let votes_active = if in_bootstrap_phase {
                // In bootstrap phase, all users have active votes
                logger!("info", "[get_user_reputation_slim] BOOTSTRAP PHASE: user={} has voting power in bootstrap phase (active_users={} < min_threshold={})",
                    user_key, active_users, tag.data.min_users_for_threshold);
                true
            } else {
                // Normal phase - only users with sufficient reputation have active votes
                data.has_voting_power
            };
            
            // Return the slim author info
            Ok(Some(AuthorInfo {
                effective_reputation: data.reputation_total_effective,
                vote_weight: data.vote_weight.clone(),
                votes_active,
            }))
        },
        Err(e) => {
            logger!("error", "[get_user_reputation_slim] Failed to decode reputation data: key={}, error={}", 
                reputation_key, e);
            Err(format!("Failed to decode reputation data: {}", e))
        }
    }
}

/// Gets the base weight for a vote from the author's reputation
/// 
/// This is a helper function that calculates the base weight for a vote
/// based on the author's reputation information. It's used in the
/// calculate_and_store_vote_weight function.
/// 
/// # Arguments
/// * `author_info` - The author's reputation information
/// 
/// # Returns
/// * `f64` - The base weight for votes by this author
fn calculate_vote_base_weight(author_info: &AuthorInfo) -> f64 {
    // Start with a base weight of 1.0
    // In the future, this could be adjusted based on the author's reputation
    1.0
}

/// Calculates and stores the weight of a vote, based on the voter's reputation.
///
/// This function performs database queries using the description field in the following format:
/// 
/// 1. Votes collection format: owner=userKey;target=targetUserKey;tag=tagKey;
///    - We query for documents with: owner=user_key;tag=tag_key;
///    - This finds all votes cast by this user in this specific tag
/// 
/// 2. Reputations collection format: owner=userKey;tag=tagKey;
///    - We query for documents with: owner=user_key;tag=tag_key;
///    - This finds the reputation document for this user in this tag
/// 
/// The queries use the Juno ListMatcher's exact string matching capability:
/// 
///    owner=userKey;tag=tagKey;
/// 
/// This approach:
///    - Sanitizes keys to avoid regex special character issues
///    - Creates consistent description formats
///    - Uses Juno's built-in string matching for efficient queries
/// 
/// # Concept
/// Instead of calculating vote weights dynamically (which could lead to reputation inflation),
/// we store a single normalized weight value that can be applied to any vote:
/// - Each user gets 100% total voting power
/// - Their vote weight = 1/total_weighted_votes
/// - This ensures sum of all weighted votes = 100%
/// - Time multipliers can then be applied without inflating total influence
/// 
/// # Example
/// If a user has 10 votes across different time periods:
/// 
/// Time period of vote                        multiplier
/// Period 1 (1 month,  1.5x):    3 votes   *     1.5      = 4.5
/// Period 2 (2 months, 1.2x):    4 votes   *     1.2      = 4.8
/// Period 3 (3 months, 1.1x):    3 votes   *     1.1      = 3.3
/// Total weighted votes = 12.6
/// 
/// Individual vote weight = 1/12.6 ≈ 0.0794 (7.94%)
/// 
/// When applied (if all votes came from the same user, just to prove the point):
/// Time period of vote                        vote weight
/// Period 1 (1 month,  1.5x):    3 votes   *    0.0794    *   1.5 = 0.3573 (35.73% total)
/// Period 2 (2 months, 1.2x):    4 votes   *    0.0794    *   1.2 = 0.3811 (38.11% total)
/// Period 3 (3 months, 1.1x):    3 votes   *    0.0794    *   1.1 = 0.2620 (26.20% total)
///                                                       Total influence = 100.00%
/// Note: The vote weight is the same for each vote, 
/// but the total influence is distributed based on the time period of the vote.
/// For the amounts to add up to 100% is by design, we need to use a lot of decimal places.
/// 
/// # Process
/// 1. Get all votes by this user in the specified tag
/// 2. For each vote:
///    - Get time-based multiplier based on vote age
///    - Add (1.0 * multiplier) to total weighted votes
/// 3. Calculate vote weight as 1/total_weighted_votes
/// 4. Store this weight in user's reputation document
/// 
/// # Benefits
/// - Single stored value reduces queries and computation
/// - Supports time-based vote weighting without inflation
/// - Part of bot prevention (new accounts must earn reputation)
/// - Ensures user's total influence stays at 100%
/// 
/// For detailed explanation and examples, see: /docs/core/development/test-calculations.md
///
pub async fn calculate_and_store_vote_weight(user_key: &str, tag_key: &str) -> Result<f64, String> {
    // Overview: This function calculates a normalized vote weight for a user in a specific tag and 
    // stores it in their reputation document. The process involves:
    //
    // 1. Query Format: We use a consistent description format for documents:
    //    - Votes collection: owner=userKey;tag=tagKey;target=targetUserKey;
    //          -> to query all votes by a user in a tag: `owner=userKey;tag=tagKey;`
    //          -> to query all votes to a user in a tag: `tag=tagKey;target=userKey;`
    //    - Reputations collection: owner=userKey;tag=tagKey;
    //
    // 2. Process Steps:
    //    - Step 1: Get tag configuration for the specified tag
    //    - Step 2: Query votes by this user in the specified tag
    //    - Step 3: Calculate total weighted votes by applying time multipliers
    //    - Step 4: Calculate individual vote weight (1/total_weighted_votes)
    //    - Step 5: Query existing reputation document for this user-tag pair
    //    - Step 6: Prepare reputation data (existing or new)
    //    - Step 8: Create complete reputation document
    //    - Step 9: Store the document with proper version handling
    
    logger!("info", "[calculate_and_store_vote_weight] START calculating vote weight for user={}, tag={}", user_key, tag_key);
    
    // Step 1: Get Tag Configuration for the specified tag
    // ----------------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 1: Getting tag configuration for tag={}", tag_key);
    let _tag = get_tag_doc(tag_key).await?;
    logger!("debug", "[calculate_and_store_vote_weight] Successfully retrieved tag: {}", tag_key);

    // Step 2: Get User's Votes
    // -----------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 2: Querying votes by user={} in tag={}", user_key, tag_key);
    
    // Create the key pattern to find votes by this user in this tag
    let vote_key_pattern = format!("usr_{}_tag_{}_", user_key, tag_key);
    logger!("info", "[calculate_and_store_vote_weight] Using key pattern: {}", vote_key_pattern);
    
    // Use our general-purpose query helper with the formatted key pattern
    let user_votes_result = query_doc_by_key(
        "votes",
        &vote_key_pattern
    )?;
    
    // Use the results directly - no need for additional filtering
    let user_votes_for_tag = user_votes_result.items;

    // Get the count of votes
    let user_votes_count = user_votes_for_tag.len();
    
    logger!("debug", "[calculate_and_store_vote_weight] Found {} votes by user", user_votes_count);
    // Add INFO level log for number of votes found
    logger!("info", "[calculate_and_store_vote_weight] Found {} votes by user={} in tag={}", user_votes_count, user_key, tag_key);

    // Step 3: Calculate Total Weighted Votes
    // ------------------------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 3: Calculating total weighted votes");
    let mut total_weighted_votes = 0.0;
    for (_, doc) in &user_votes_for_tag {
        // Get time-based multiplier for this vote using the document's created_at timestamp
        // We don't need to decode the document data since we only use the timestamp
        let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;
        
        // Add to total: base value (1.0) * time multiplier
        total_weighted_votes += 1.0 * time_multiplier;
    }
    logger!("debug", "[calculate_and_store_vote_weight] Total weighted votes: {}", total_weighted_votes);
    // Add INFO level log for total weighted votes
    logger!("info", "[calculate_and_store_vote_weight] Total weighted votes for user={}: {}", user_key, total_weighted_votes);

    // Step 4: Calculate Individual Vote Weight
    // -------------------------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 4: Calculating individual vote weight");
    let vote_weight = if total_weighted_votes > 0.0 {
        match VoteWeight::new(1.0 / total_weighted_votes) {
            Ok(weight) => weight,
            Err(e) => {
                logger!("error", "[calculate_and_store_vote_weight] Error creating vote weight: user={}, tag={}, error={}",
                    user_key, tag_key, e);
                return Err(format!("Invalid vote weight calculated: {}", e));
            }
        }
    } else {
        // If user has no votes yet, give them a base weight of 1.0 instead of 0.0
        // This allows new users to have some initial influence with their first votes
        logger!("info", "[calculate_and_store_vote_weight] User={} has no votes in tag={}, setting initial vote_weight=1.0",
            user_key, tag_key);
        match VoteWeight::new(1.0) {
            Ok(weight) => weight,
            Err(e) => {
                logger!("error", "[calculate_and_store_vote_weight] Error creating initial vote weight: user={}, tag={}, error={}",
                    user_key, tag_key, e);
                return Err(format!("Invalid vote weight calculated: {}", e));
            }
        }
    };
    logger!("debug", "[calculate_and_store_vote_weight] Calculated vote weight: {}", vote_weight.value());
    // Add INFO level log for calculated vote weight
    logger!("info", "[calculate_and_store_vote_weight] RESULT: Vote weight for user={} in tag={}: {}", user_key, tag_key, vote_weight.value());

    // Step 5: Query existing reputation document for this user-tag pair
    // We need to find if a reputation document already exists for this user in this tag
    logger!("debug", "[calculate_and_store_vote_weight] Step 5: Querying reputation document for user={}, tag={}", user_key, tag_key);
    
    // Create reputation key format: usr_{user_ulid}_tag_{tag_ulid}_
    let reputation_key = match format_reputation_key(user_key, tag_key) {
        Ok(key) => key,
        Err(e) => {
            logger!("error", "[calculate_and_store_vote_weight] Failed to format reputation key: user={}, tag={}, error={}", 
                user_key, tag_key, e);
            return Err(format!("Failed to format reputation key: {}", e));
        }
    };
    
    logger!("debug", "[calculate_and_store_vote_weight] Looking up reputation document with key: {}", reputation_key);
    
    // Execute direct key lookup (more efficient than list_docs)
    let reputation_doc = match junobuild_satellite::get_doc_store(
        ic_cdk::id(),  // Use canister ID for system-level operations
        String::from("reputations"),
        reputation_key.clone() // Remove the & reference, clone the string
    ) {
        Ok(doc) => doc,
        Err(e) => {
            logger!("error", "[calculate_and_store_vote_weight] Error retrieving reputation document: key={}, error={}", 
                reputation_key, e);
            return Err(format!("Error retrieving reputation document: {}", e));
        }
    };
    
    // Step 6: Prepare Reputation Data
    // ----------------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 6: Preparing reputation data");
    
    // Process existing document or create new data
    // The get_doc_store returns None if document doesn't exist
    let (doc_key, mut existing_data, version) = if let Some(doc) = reputation_doc {
        // Document exists, try to decode
        logger!("debug", "[calculate_and_store_vote_weight] Found existing reputation document: key='{}', version={:?}", 
            reputation_key, doc.version);
        
        // Log description if available
        if let Some(desc) = &doc.description {
            logger!("debug", "[calculate_and_store_vote_weight] Document description: '{}'", 
                desc);
        }
        
        // Decode document
        match decode_doc_data::<ReputationData>(&doc.data) {
            Ok(rep_data) => {
                logger!("debug", "[calculate_and_store_vote_weight] Successfully decoded document with key='{}', previous vote_weight={}",
                    reputation_key, rep_data.vote_weight.value());
                (reputation_key, Some(rep_data), doc.version)
            },
            Err(e) => {
                // Failed to decode but document exists
                let err_msg = format!("Failed to deserialize reputation data: {}", e);
                logger!("error", "{}", err_msg);
                // We'll create new data but keep the document key and version
                (reputation_key, None, doc.version)
            }
        }
    } else {
        // No document exists, we'll create everything from scratch
        logger!("debug", "[calculate_and_store_vote_weight] No existing reputation document found for user={} for tag={}", 
            user_key, tag_key);
        
        // Generate a new document key using our random key generator
        let new_key = create_reputation_key(user_key, tag_key).await?;
        logger!("debug", "[calculate_and_store_vote_weight] Generated key={} for new reputation document", new_key);
        
        (new_key, None, None)
    };
    
    // Create or update the reputation data
    let reputation_data = if let Some(ref mut existing) = existing_data {
        // Log current vote weight before any changes
        logger!("debug", "[calculate_and_store_vote_weight] Using existing reputation data with vote_weight={}",
            existing.vote_weight.value());
        // Update existing data with new vote_weight
        existing.vote_weight = vote_weight.clone();
        existing.last_calculation = ic_cdk::api::time();
        
        // Return a clone of the existing data (converts &mut ReputationData to ReputationData)
        existing.clone()
    } else {
        // Create new data with all default values
        ReputationData {
            user_key: user_key.to_string(),
            tag_key: tag_key.to_string(),
            reputation_basis: 0.0,
            reputation_rewards: 0.0,
            reputation_total_effective: 0.0,
            last_calculation: ic_cdk::api::time(),
            vote_weight: vote_weight.clone(),
            has_voting_power: false,
        }
    };

    // Step 8: Create Complete Document
    // -----------------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 8: Preparing data for reputation document");
    
    // No need for a document description, as we're using key-based queries instead
    // The key contains all the necessary information: usr_{userId}_tag_{tagId}_
    let description = String::new(); // Empty description - we use the key for queries now
    
    // Don't increment the version here, use the original version directly
    // The version will be automatically incremented by Juno when the document is stored
    
    let reputation = Reputation {
        key: doc_key.clone(),
        description,
        owner: ic_cdk::id(),  // Use canister's Principal ID as owner
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        version: version.unwrap_or(0), // Use the original version, not incremented
        data: reputation_data,
    };

    // Step 9: Store Document
    // -------------------
    logger!("debug", "[calculate_and_store_vote_weight] Step 9: Storing reputation document");
    match encode_doc_data(&reputation.data) {
        Ok(encoded_data) => {
            // Determine if this is a new document or an update based on version
            if let Some(v) = version {
                logger!("debug", "[calculate_and_store_vote_weight] Updating existing document with current version: {}", v);
                
                // For existing documents, include the version
                let doc = SetDoc {
                    data: encoded_data,
                    description: Some(reputation.description),
                    version, // Include version for updates to prevent concurrent conflicts
                };

                // Make explicit double-check of version before storing
                logger!("debug", "[calculate_and_store_vote_weight] Attempting to update document: key={}, version={:?}", 
                    reputation.key, doc.version);

                match set_doc_store(
                    ic_cdk::id(),  // Use canister's Principal ID as caller
                    String::from("reputations"),
                    reputation.key.clone(),
                    doc,
                ) {
                    Ok(_) => {
                        logger!("info", "[calculate_and_store_vote_weight] SUCCESS: Updated reputation document with key={}, version={:?}, vote_weight={}",
                            reputation.key, version, vote_weight.value());
                        Ok(vote_weight.value())
                    },
                    Err(e) => {
                        // Enhanced error logging for version conflicts
                        logger!("error", "[calculate_and_store_vote_weight] ERROR: Failed to store reputation document: key={}, attempted_version={:?}, error={}",
                            reputation.key, version, e);
                        
                        // Double-check version with direct document retrieval
                        if let Some(current_doc) = get_doc(String::from("reputations"), reputation.key.clone()) {
                            logger!("error", "[calculate_and_store_vote_weight] VERIFICATION: Current document in database has version={:?}, we attempted to use version={:?}",
                                current_doc.version, version);
                        } else {
                            logger!("error", "[calculate_and_store_vote_weight] VERIFICATION: Document with key={} not found during verification",
                                reputation.key);
                        }
                        
                        Err(format!("Failed to store reputation: {}", e))
                    }
                }
            } else {
                // For new documents, use SetDoc with version: Some(0)
                logger!("debug", "[calculate_and_store_vote_weight] Creating new document with version=0");
                
                let doc = SetDoc {
                    data: encoded_data,
                    description: Some(reputation.description),
                    version: Some(0), // Use Some(0) for new documents
                };

                logger!("debug", "[calculate_and_store_vote_weight] Storing new document with key={}", 
                    reputation.key);

                match set_doc_store(
                    ic_cdk::id(),  // Use canister's Principal ID as caller
                    String::from("reputations"),
                    reputation.key.clone(),
                    doc,
                ) {
                    Ok(_) => {
                        logger!("info", "[calculate_and_store_vote_weight] SUCCESS: Created reputation document with key={}, vote_weight={}",
                            reputation.key, vote_weight.value());
                        Ok(vote_weight.value())
                    },
                    Err(e) => {
                        // Enhanced error logging for new document creation
                        logger!("error", "[calculate_and_store_vote_weight] ERROR: Failed to create reputation document: key={}, error={}",
                            reputation.key, e);
                        
                        Err(format!("Failed to create reputation: {}", e))
                    }
                }
            }
        },
        Err(e) => {
            logger!("error", "[calculate_and_store_vote_weight] Error serializing reputation data: user={}, tag={}, error={}",
                user_key, tag_key, e);
            Err(format!("Failed to serialize reputation data: {}", e))
        }
    }
}

/// Calculates a user's reputation in a specific tag
/// 
/// This function implements the complete reputation calculation process for a user in a specific tag.
/// The calculation follows these steps:
/// 
/// 1. **Query Votes**
///    - Queries all votes where the user is the target
/// 
/// 2. **Author Index Creation**
///    - Creates an index of unique authors to avoid duplicate queries
///    - For each author, we will need:
///      - Their current effective reputation
///      - Their vote weight in this tag
///      - Whether their votes are active or not
/// 
/// 3. **Basis Reputation Calculation**
///    - Iterates through all received votes
///    - For each vote:
///      - Skips row if the author's votes are not active
///      - Applies time-based multiplier from tag rules
///      - Multiplies by author's weight and reputation (from the index created in step 2)
///      - Adds to total (positive votes = +1, negative = -1)
///    - Result stored as total_basis_reputation
///    - If no active authors are found, basis reputation is set to 0.0 but calculation continues
/// 
/// 4. **Trust Status Check**
///    - Compares total_basis_reputation against tag's minimum threshold
///    - User is considered "trusted" if his total_basis_reputation is above threshold
///    - If the user is trusted/untrusted, we will need to store the fact that he is trusted in the user's reputation document for this tag
/// 
/// 5. **Voting Rewards Calculation**
///    - Retrieve all votes where author is the user being calculated
///    - Format is now: "owner=user_key;tag=tag_key;"
///    - Get voting reward value from tag's configuration (tag.vote_reward)
///    - For each vote made by user:
///      - Calculate reward = tag.vote_reward * time multiplier
///    - Sum all rewards to get total_voting_rewards_reputation
///    - This step always executes even if no basis reputation was calculated
/// 
/// 6. **Final Reputation Calculation**
///    - If user is trusted OR community is in bootstrap phase:
///      - effective_reputation = total_basis_reputation + total_voting_rewards_reputation
///    - Otherwise:
///      - effective_reputation = total_basis_reputation
/// 
/// 7. **Storage**
///    - Stores all calculated values in reputations collection:
///      - total_basis_reputation
///      - total_voting_rewards_reputation
///      - last_known_effective_reputation
///      - trust status (from step 4)
/// 
/// # Arguments
/// * `user_key` - The key of the user whose reputation is being calculated
/// * `tag_key` - The key of the tag to calculate reputation for
/// 
/// # Returns
/// * `Result<ReputationData, String>` - The calculated reputation data or an error message
/// 
/// # Notes
/// - Only considers votes from authors with non-zero effective reputation
/// - Vote weights are calculated using time-based multipliers
/// - Voting rewards are only included for trusted users or in bootstrap phase
/// - All calculations are tag-specific and don't affect other tags
/// - Trust status is stored to determine if user's votes are active
/// - Voting rewards are always calculated even if user has received no votes
pub async fn calculate_user_reputation(user_key: &str, tag_key: &str) -> Result<ReputationData, String> {
    // Add start log message
    logger!("info", "[calculate_user_reputation] START calculating reputation for user={}, tag={}", user_key, tag_key);
    
    // Get the tag once at the start - we'll reuse this for all calculations
    let tag = get_tag_doc(tag_key).await?;

    // Step 1: Query Votes
    // ----------------------
    // Query votes where user is the target within the specific tag
    // Use key-based query: tag_{tag_key}_tar_{user_key}_
    let vote_key_pattern = format!("tag_{}_tar_{}_", tag_key, user_key);
    
    // Log the search pattern being used
    logger!("info", "[calculate_user_reputation] Searching for votes targeting user with key pattern: {}", vote_key_pattern);
    
    // Execute the votes query using our key-based query helper
    let vote_items_result = query_doc_by_key(
        "votes",
        &vote_key_pattern
    )?;
    
    // Use the results directly - no need to filter
    let vote_items = vote_items_result.items;

    // Add vote count info log
    logger!("info", "[calculate_user_reputation] VOTES TARGETING USER: Found {} votes where user={} is the target in tag={}",
        vote_items.len(), user_key, tag_key);

    // Convert raw vote documents into VoteData structs
    // This step:
    // 1. Iterates through each vote document from Juno storage
    // 2. Uses decode_doc_data to convert binary data into VoteData
    // 3. Handles any binary decoding errors
    let mut vote_data_list: Vec<VoteData> = Vec::new();
    for (_, doc) in &vote_items {
        match decode_doc_data::<VoteData>(&doc.data) {
            Ok(vote_data) => {
                vote_data_list.push(vote_data);
            }
            Err(e) => {
                logger!("warn", "Error decoding vote data: {}", e);
                continue;
            }
        }
    }

    // Add vote count info log
    logger!("info", "[calculate_user_reputation] Found {} votes targeting user={} in tag={}", vote_items.len(), user_key, tag_key);

    // Add detailed vote and reputation calculation log
    logger!("info", "[calculate_user_reputation] Found {} votes for user={} in tag={} (base reward per vote: {})",
        vote_data_list.len(),
        user_key,
        tag_key,
        tag.data.vote_reward
    );

    // Step 2: Author Index Creation
    // ----------------------------
    // Create an index of unique authors to avoid duplicate queries.
    // We only process authors from our vote list - if an author isn't in our votes,
    // we don't need their information. For each author in our votes:
    // - Get their current effective reputation
    // - Get their vote weight in this tag
    // - Get their trust status (if they are trusted or not)
    // - Store all this information for use in basis reputation calculation
    let mut author_index: HashMap<String, AuthorInfo> = HashMap::new();

    // Add info about author activity checking
    logger!("info", "[calculate_user_reputation] Checking {} unique authors who voted on user={}", vote_items.len(), user_key);
    
    // Process each vote to get author information
    for (_, doc) in &vote_items {
        let vote_data: VoteData = decode_doc_data(&doc.data)
            .map_err(|e| format!("Failed to deserialize vote: {}", e))?;

        // Skip if we already have this author's information
        if author_index.contains_key(&vote_data.user_key) {
            continue;
        }

        // Get author's reputation data
        match get_user_reputation_slim(&vote_data.user_key, tag_key).await {
            Ok(Some(author_info)) => {
                // Skip if author's votes are not active
                if !author_info.votes_active {
                    // Add  detailed info about why author is inactive
                    logger!("info", "[calculate_user_reputation] Author={} is inactive in tag={}: reputation={}, has_voting_power={}",
                        vote_data.user_key, tag_key, author_info.effective_reputation, author_info.votes_active);
                    continue;
                }
                
                // Add info about active author - do this BEFORE inserting into HashMap
                logger!("info", "[calculate_user_reputation] Active author={} in tag={}: reputation={}, vote_weight={}",
                    vote_data.user_key, tag_key, author_info.effective_reputation, author_info.vote_weight.value());

                // Store author information AFTER logging
                author_index.insert(
                    vote_data.user_key.clone(),
                    author_info, // Use the AuthorInfo directly
                );
            }
            Ok(None) => {
                logger!("warn", "No reputation data found for author in calculate_user_reputation: author={}, tag={}",
                    vote_data.user_key, tag_key);
            }
            Err(e) => {
                logger!("error", "[calculate_user_reputation] Error getting author reputation: author={}, tag={}, error={}",
                    vote_data.user_key, tag_key, e);
                continue;
            }
        }
    }

    // If we have no valid authors (all had inactive votes), set basis reputation to 0
    // but continue with voting rewards calculation
    let total_basis_reputation = if author_index.is_empty() {
        logger!("info", "[calculate_user_reputation] No votes from active authors found for user={} in tag={}, setting basis reputation to 0 but continuing with rewards",
            user_key, tag_key);
        0.0
    } else {
        // Step 3: Basis Reputation Calculation
        // -----------------------------------
        // Calculate total basis reputation from all received votes
        // For each vote, calculate its contribution by multiplying:
        // - Base value (+1 for positive, -1 for negative)
        // - Author's effective reputation
        // - Author's vote weight
        // - Time-based multiplier from tag rules
        // Then sum all vote contributions to get total_basis_reputation
        let mut basis_rep = 0.0;

        // Iterate through all received votes
        for (_, doc) in &vote_items {
            let vote_data: VoteData = decode_doc_data(&doc.data)
                .map_err(|e| format!("Failed to deserialize vote: {}", e))?;

            // Get author's information from our index
            let author_info = match author_index.get(&vote_data.user_key) {
                Some(info) => info,
                None => {
                    logger!("warn", "[calculate_user_reputation - Iterate Votes] Warning: Author info not found in index to calculate vote, author={}",
                        vote_data.user_key);
                    continue;
                }
            };

            // Get time-based multiplier for this vote using the document's created_at timestamp
            let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;

            // Calculate this vote's contribution:
            // 1. Base value: Use the vote's value directly
            let base_value = vote_data.value;

            // 2. Multiply by author's effective reputation
            let with_reputation = base_value * author_info.effective_reputation;

            // 3. Multiply by author's vote weight
            let with_weight = with_reputation * author_info.vote_weight.value();

            // 4. Finally apply time multiplier
            let final_contribution = with_weight * time_multiplier;

            // Add to total
            basis_rep += final_contribution;
        }
        
        basis_rep
    };

    // Add info about basis reputation total
    logger!("info", "[calculate_user_reputation] Calculated total_basis_reputation={} for user={} in tag={}", total_basis_reputation, user_key, tag_key);

    // Step 4: Trust Status Check
    // -------------------------
    // Compare total_basis_reputation against tag's minimum threshold
    // to determine if user has voting power
    let meets_threshold = total_basis_reputation >= tag.data.reputation_threshold;
    
    // Get active users count for bootstrap phase check
    let active_users = get_active_users_count(tag_key).await?;
    let in_bootstrap_phase = active_users < tag.data.min_users_for_threshold;
    
    // Determine voting power based on threshold or bootstrap phase
    let has_voting_power = meets_threshold || in_bootstrap_phase;
    
    // Add detailed info about voting power determination
    if meets_threshold {
        logger!("info", "[calculate_user_reputation] User meets threshold: user={}, reputation={}, threshold={}",
            user_key, total_basis_reputation, tag.data.reputation_threshold);
    } else if in_bootstrap_phase {
        logger!("info", "[calculate_user_reputation] Bootstrap phase active: active_users={} < min_threshold={}, voting rewards active",
            active_users, tag.data.min_users_for_threshold);
    } else {
        logger!("info", "[calculate_user_reputation] User lacks threshold: user={}, reputation={}, threshold={}",
            user_key, total_basis_reputation, tag.data.reputation_threshold);
    }

    // Step 5: Voting Rewards Calculation
    // --------------------------------
    // Retrieve all votes where author is the user being calculated
    // Get voting reward value from tag's configuration (tag.vote_reward)
    // For each vote made by user:
    // - Calculate reward = tag.vote_reward * time multiplier
    // Sum all rewards to get total_voting_rewards_reputation
    // Query votes where this user is the author
    // Use key-based query pattern: usr_{user_key}_tag_{tag_key}_
    let vote_key_pattern = format!("usr_{}_tag_{}_", user_key, tag_key);
    
    // Log the query pattern
    logger!("info", "[calculate_user_reputation] Searching for votes cast by user with key pattern: {}", vote_key_pattern);

    // Query the database for votes cast by the user using our helper
    let user_votes_result = query_doc_by_key(
        "votes",
        &vote_key_pattern
    )?;
    
    // Log the results of the query
    logger!("info", "[calculate_user_reputation] USER VOTES: Found {} votes cast by user={} in tag={}",
        user_votes_result.items.len(), user_key, tag_key);

    // Calculate rewards for each vote cast by the user
    let mut total_voting_rewards_reputation = 0.0;
    
    // If we're in bootstrap phase, rewards are always given regardless of voting power
    // Otherwise, rewards are only given if user meets threshold or already has rewards from bootstrap
    
    // First determine if user should receive voting rewards
    let should_receive_rewards = in_bootstrap_phase || meets_threshold;
    
    if should_receive_rewards {
        for (_, doc) in user_votes_result.items {
            // We only need the timestamp, not the vote data itself
            // Get time-based multiplier for this vote using the document's created_at timestamp
            let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;
            
            let reward = tag.data.vote_reward * time_multiplier;
            total_voting_rewards_reputation += reward;
            
            // Add detailed log for each vote reward calculation
            logger!("info", "[calculate_user_reputation] VOTE_REWARD: author={}, voteR={} (base_reward={} * time_multiplier={}), created_at={}",
                user_key, reward, tag.data.vote_reward, time_multiplier, doc.created_at);
        }
        
        // Log if rewards were given due to bootstrap phase
        if in_bootstrap_phase && !meets_threshold && total_voting_rewards_reputation > 0.0 {
            logger!("info", "[calculate_user_reputation] BOOTSTRAP REWARDS: user={} receives voteR={} during bootstrap phase",
                user_key, total_voting_rewards_reputation);
        }
    } else {
        logger!("info", "[calculate_user_reputation] NO REWARDS: user={} does not receive voting rewards (bootstrap={}, meets_threshold={})",
            user_key, in_bootstrap_phase, meets_threshold);
    }

    // Log info about the reputation calculation results 
    logger!("info", "[calculate_user_reputation] SUMMARY: user={}, tag={}, basisR={}, voteR={}, active_users={}",
        user_key, tag_key, total_basis_reputation, total_voting_rewards_reputation, active_users);

    // Step 6: Final Reputation Calculation
    // ----------------------------------
    // Calculate final effective reputation
    logger!("info", "[calculate_user_reputation] Found {} active users in tag={} (min_users_for_threshold={})",
        active_users, tag_key, tag.data.min_users_for_threshold);

    // Always include voting rewards in effective reputation
    let effective_reputation = total_basis_reputation + total_voting_rewards_reputation;
    
    // Log appropriate message based on voting power status
    if meets_threshold {
        logger!("info", "[calculate_user_reputation] TRUSTED: user={} has voting power by meeting threshold in tag={}",
            user_key, tag_key);
    } else if in_bootstrap_phase {
        logger!("info", "[calculate_user_reputation] BOOTSTRAP: tag={} is in bootstrap phase ({} < {} users), user has voting power",
            tag_key, active_users, tag.data.min_users_for_threshold);
    } else {
        logger!("info", "[calculate_user_reputation] UNTRUSTED: user={} lacks voting power but still gets voting rewards",
            user_key);
    }
    
    logger!("info", "[calculate_user_reputation] Effective reputation: basisR={} + voteR={} = totalR={}",
        total_basis_reputation, total_voting_rewards_reputation, effective_reputation);

    // Step 7: Storage
    // --------------
    // This step handles storing the calculated reputation data in the database.
    // The process follows these steps:
    // 1. Create the document description using DocumentDescription helper, we will use that for the query.
    // 2. Query the database 'reputations' for if a document already exists with this description and store it in a variabla for the document and another for the data.
    // 3. If the document doesn't exist, generate a unique document key using nanoid()
    // 4. Now generate the document in the format we need for storage.
    // 5. Store with proper version handling
    logger!("debug", "[calculate_user_reputation] Step 7: Storing reputation data for user={}, tag={}", user_key, tag_key);

    // Step 7.1: Query for existing document using key-based query
    // --------------------------------------------------------
    // Create the reputation key format: usr_{user_ulid}_tag_{tag_ulid}_
    let reputation_key = match format_reputation_key(user_key, tag_key) {
        Ok(key) => key,
        Err(e) => {
            logger!("error", "[calculate_user_reputation] Failed to format reputation key: user={}, tag={}, error={}", 
                user_key, tag_key, e);
            return Err(format!("Failed to format reputation key: {}", e));
        }
    };
    
    logger!("debug", "[calculate_user_reputation] Looking up reputation document with key: {}", reputation_key);
    
    // Query for the document using direct key lookup
    let existing_doc = junobuild_satellite::get_doc(
        String::from("reputations"),
        reputation_key.clone()
    );

    // Step 7.2: Prepare reputation data and document details
    // ---------------------------------------------------
    // Here we handle three cases:
    // 1. Found document and successfully decoded it - use its key, version, and vote_weight
    // 2. Found document but failed to decode - use its key and version, but default vote_weight
    // 3. No document found - generate a new key and use default vote_weight

    // First, check if document exists and try to decode it
    let (doc_key, existing_data, version) = if let Some(doc) = existing_doc {
        // Found a document, try to decode it
        match decode_doc_data::<ReputationData>(&doc.data) {
            Ok(decoded_data) => {
                // Case 1: Successfully decoded existing document
                logger!("info", "[calculate_user_reputation] Using existing vote_weight={} from reputation document for user={} in tag={}",
                    decoded_data.vote_weight.value(), user_key, tag_key);
                (reputation_key.clone(), Some(decoded_data), doc.version)
            },
            Err(e) => {
                // Case 2: Document exists but failed to decode
                logger!("error", "[calculate_user_reputation] Failed to decode existing reputation data, using default vote_weight=0.0: error={}",
                    e);
                (reputation_key.clone(), None, doc.version)
            }
        }
    } else {
        // Case 3: No document found: create a new one
        logger!("info", "[calculate_user_reputation] No existing document found for user={} in tag={}, creating new",
            user_key, tag_key);
        // Generate a new random document key
        let new_key = match create_reputation_key(user_key, tag_key).await {
            Ok(key) => key,
            Err(e) => {
                logger!("error", "[calculate_user_reputation] Failed to create reputation key: user={}, tag={}, error={}", 
                    user_key, tag_key, e);
                return Err(format!("Failed to create reputation key: {}", e));
            }
        };
        
        logger!("debug", "[calculate_user_reputation] Generated new key={} for reputation document", new_key);
        
        (new_key, None, None)
    };

    // Create the reputation data with all calculated values
    let reputation_data = ReputationData {
        user_key: user_key.to_string(),
        tag_key: tag_key.to_string(),
        reputation_basis: total_basis_reputation,
        reputation_rewards: total_voting_rewards_reputation,
        reputation_total_effective: effective_reputation,
        last_calculation: ic_cdk::api::time(),
        vote_weight: existing_data.as_ref().map(|data| data.vote_weight.clone())
                    .unwrap_or_else(|| VoteWeight::new(0.0).unwrap()),
        has_voting_power,
    };

    // Step 7.3: Store document
    // ----------------------
    match encode_doc_data(&reputation_data) {
        Ok(encoded_data) => {
            // Create different SetDoc struct based on whether this is a new document or existing one
            if let Some(existing) = existing_data {
                // For existing documents, include the version field
                let doc = SetDoc {
                    data: encoded_data,
                    description: Some(reputation_key),
                    version, // Include version for existing documents
                };

                // Add log to show what we're about to store
                logger!("info", "[calculate_user_reputation] UPDATING: key={}, basisR={}, voteR={}, totalR={}, vote_weight={}",
                    doc_key, reputation_data.reputation_basis, 
                    reputation_data.reputation_rewards,
                    reputation_data.reputation_total_effective,
                    existing.vote_weight.value());

                // Store the document with version for updates
                match set_doc_store(
                    ic_cdk::id(),
                    String::from("reputations"),
                    doc_key.clone(),
                    doc,
                ) {
                    Ok(_) => {
                        logger!("info", "[calculate_user_reputation] SUCCESS: updated reputation for user={} in tag={}: basisR={}, voteR={}, totalR={}",
                            user_key, 
                            tag_key, 
                            reputation_data.reputation_basis, 
                            reputation_data.reputation_rewards, 
                            reputation_data.reputation_total_effective
                            );
                        Ok(reputation_data)
                    },
                    Err(e) => {
                        logger!("error", "[calculate_user_reputation] Failed to update reputation document: key={}, error={}",
                            doc_key, e);
                        Err(format!("Failed to update reputation: {}", e))
                    }
                }
            } else {
                // For new documents, use SetDoc with version: Some(0)
                logger!("debug", "[calculate_user_reputation] Creating new document with version=0");
                
                let doc = SetDoc {
                    data: encoded_data,
                    description: Some(reputation_key),
                    version: Some(0), // Use Some(0) for new documents
                };

                // Add log to show what we're about to store
                logger!("info", "[calculate_user_reputation] CREATING: key={}, basisR={}, voteR={}, totalR={}, vote_weight={}",
                    doc_key, reputation_data.reputation_basis, 
                    reputation_data.reputation_rewards,
                    reputation_data.reputation_total_effective,
                    existing_data.as_ref().map(|data| data.vote_weight.value())
                    .unwrap_or_else(|| 0.0)
                    );

                // Store the document with version: Some(0) for new documents
                match set_doc_store(
                    ic_cdk::id(),
                    String::from("reputations"),
                    doc_key.clone(),
                    doc,
                ) {
                    Ok(_) => {
                        logger!("info", "[calculate_user_reputation] SUCCESS: created reputation for user={} in tag={}: basisR={}, voteR={}, totalR={}",
                            user_key, 
                            tag_key, 
                            reputation_data.reputation_basis, 
                            reputation_data.reputation_rewards, 
                            reputation_data.reputation_total_effective
                            );
                        Ok(reputation_data)
                    },
                    Err(e) => {
                        logger!("error", "[calculate_user_reputation] Failed to create reputation document: key={}, error={}",
                            doc_key, e);
                        Err(format!("Failed to create reputation: {}", e))
                    }
                }
            }
        },
        Err(e) => {
            logger!("error", "[calculate_user_reputation] Failed to encode reputation data: {}",
                e);
            Err(format!("Failed to encode reputation data: {}", e))
        }
    }
}

/// Gets the multiplier for a vote based on its age and the tag's configuration
/// 
/// This function calculates the time-based decay multiplier for votes based on their age.
/// The multiplier is used to give more weight to recent votes and less to older ones,
/// helping to maintain a dynamic and relevant reputation system.
/// 
/// The multiplier is determined by the tag's time_periods configuration, which defines
/// different time ranges and their corresponding multipliers. For example:
/// 
/// ```rust
/// time_periods: [
///     { months: 1, multiplier: 1.5 },    // First month: 150% weight
///     { months: 2, multiplier: 1.2 },    // Months 2-3: 120% weight
///     { months: 3, multiplier: 1.1 },    // Months 4-6: 110% weight
///     { months: 6, multiplier: 1.0 },    // Months 7-12: 100% weight
///     { months: 12, multiplier: 0.95 },  // Months 13-24: 95% weight
///     { months: 12, multiplier: 0.75 },  // Months 25-36: 75% weight
///     { months: 12, multiplier: 0.55 },  // Months 37-48: 55% weight
///     { months: 999, multiplier: 0.25 }  // Months 49+: 25% weight
/// ]
/// ```
/// 
/// # Arguments
/// * `vote_timestamp_ns` - The creation timestamp of the vote in nanoseconds
/// * `tag_key` - The tag key to get time period configuration from
///
/// # Returns
/// * `Result<f64, String>` - The calculated multiplier value or an error
///
/// # Example
/// If tag has periods:
/// - Period 1: 2629800000000000ns (1 month), multiplier 1.5
/// - Period 2: 5259600000000000ns (2 months), multiplier 1.2
/// - Period 3: 7889400000000000ns (3 months), multiplier 1.1
///
/// And the vote is 40 days old:
/// - It's not in Period 1 (exceeds 30 days)
/// - It is in Period 2 (less than 60 days)
/// - Returns multiplier of 1.2
pub async fn get_period_multiplier(vote_timestamp_ns: u64, tag_key: &str) -> Result<f64, String> {
    // Get tag settings to access configured time periods
    let tag = get_tag_doc(tag_key).await?;
    
    // Calculate months difference between vote and now
    let months_ago = calculate_months_between(vote_timestamp_ns, ic_cdk::api::time())?;
    
    // Find the appropriate time period in the tag configuration
    let mut accumulated_months = 0;
    for time_period in &tag.data.time_periods {
        accumulated_months += time_period.months;
        if months_ago <= accumulated_months {
            return Ok(time_period.multiplier);
        }
    }
    
    // If no matching period found, use the last period's multiplier
    // This handles votes older than any defined period
    if let Some(last_period) = tag.data.time_periods.last() {
        Ok(last_period.multiplier)
    } else {
        // Fallback if no periods defined (shouldn't happen due to validation)
        Ok(1.0)
    }
}

/// Gets a tag by its ulid
/// 
/// This function retrieves a tag document by its ULID using query_doc_by_key.
/// 
/// # Arguments
/// * `tag_doc_ulid` - The ULID of the tag to retrieve
/// 
/// # Returns
/// * `Result<Tag, String>` - The tag document or an error message
async fn get_tag_doc(tag_doc_ulid: &str) -> Result<Tag, String> {
    // Query for the tag using the tag ULID
    logger!("debug", "[get_tag_doc] Looking up tag with ULID: {}", tag_doc_ulid);
    
    // Use query_doc_by_key with tag pattern
    let tag_key_pattern = format!("tag_{}_", tag_doc_ulid);
    logger!("debug", "[get_tag_doc] Using key pattern: {}", tag_key_pattern);
    
    let tag_results = query_doc_by_key(
        "tags",
        &tag_key_pattern
    )?;
    
    // Check if we found any matching tags
    if tag_results.items.is_empty() {
        let err_msg = format!("Tag not found: {}", tag_doc_ulid);
        logger!("error", "[get_tag_doc] {}", err_msg);
        return Err(err_msg);
    }
    
    // Get the first matching tag (there should only be one)
    let (doc_key, tag_doc) = tag_results.items.first()
        .ok_or_else(|| format!("Tag not found: {}", tag_doc_ulid))?;

    // Decode the tag data into TagData
    let tag_data: TagData = decode_doc_data(&tag_doc.data)
        .map_err(|e| {
            logger!("error", "[get_tag_doc] Failed to deserialize tag data: key={}, error={}",
                tag_doc_ulid, e);
            format!("Failed to deserialize tag: {}", e)
        })?;
        
    // Construct a full Tag with both metadata and data
    let tag = Tag {
        key: doc_key.clone(), // Use the key from the query result
        description: tag_doc.description.clone().unwrap_or_default(),
        owner: tag_doc.owner,
        created_at: tag_doc.created_at,
        updated_at: tag_doc.updated_at,
        version: tag_doc.version.unwrap_or(0),  // Use original version
        data: tag_data,
    };

    Ok(tag)
}

/// Updates a user's reputation when they receive a vote
///
/// When a user receives a vote in a tag, this function is called to update their
/// reputation document. If no reputation document exists, one will be created.
///
/// # Arguments
/// * `target_key` - The key of the user receiving the vote (vote target)
/// * `tag_key` - The tag the vote is assigned to
/// * `vote_value` - The value of the vote (often +1 or -1)
/// * `vote_weight` - The weight of the vote (from author's voting power)
///
/// # Returns
/// * `Result<(), String>` - Success or an error message
pub async fn update_reputation_on_vote(
    target_key: &str,
    tag_key: &str,
    vote_value: f64,
    vote_weight: f64,
) -> Result<(), String> {
    // Add start log message
    logger!("info", "[update_reputation_on_vote] START updating reputation for target={}, tag={}, vote_value={}, vote_weight={}", target_key, tag_key, vote_value, vote_weight);
    
    // Step 1: Get or create reputation document for target user in this tag using key-based query
    logger!("info", "[update_reputation_on_vote] START updating reputation for target={}, tag={}, vote_value={}, vote_weight={}", target_key, tag_key, vote_value, vote_weight);
    
    // Create reputation key format to query for the document: usr_{user_ulid}_tag_{tag_ulid}_
    let reputation_key = match format_reputation_key(target_key, tag_key) {
        Ok(key) => key,
        Err(e) => {
            logger!("error", "[update_reputation_on_vote] Failed to format reputation key: user={}, tag={}, error={}", 
                target_key, tag_key, e);
            return Err(format!("Failed to format reputation key: {}", e));
        }
    };
    
    logger!("debug", "[update_reputation_on_vote] Looking up reputation document with key: {}", reputation_key);
    
    // Query for the user's reputation document using direct key lookup
    let existing_doc = junobuild_satellite::get_doc(
        String::from("reputations"),
        reputation_key.clone()
    );
    
    // Step 2: Process the query results and prepare document data
    // --------------------------------------------------------
    
    // Step 2.1: Check if a reputation document exists
    // get_doc returns the document only, not a key-value pair
    let (rep_key, mut reputation_data, version) = if let Some(doc) = existing_doc {
        // Decode the existing reputation data
        let rep_data: ReputationData = decode_doc_data(&doc.data)
            .map_err(|e| {
                let err_msg = format!("Failed to deserialize reputation data: {}", e);
                logger!("error", "{}", err_msg);
                err_msg
            })?;
        
        // Return the key, data, and original version
        (reputation_key.clone(), rep_data, doc.version)
    } else {
        // Step 2.3: If no document exists, create new data with default values
        let default_weight = VoteWeight::new(0.0).unwrap_or_else(|_| {
            logger!("error", "[update_reputation_on_vote] Failed to create default vote weight, using 0.0");
            VoteWeight::new(0.0).unwrap() // This should never fail for 0.0
        });
        
        let rep_data = ReputationData {
            user_key: target_key.to_string(),
            tag_key: tag_key.to_string(),
            reputation_basis: 0.0,
            reputation_rewards: 0.0,
            reputation_total_effective: 0.0,
            last_calculation: ic_cdk::api::time(),
            vote_weight: default_weight,
            has_voting_power: false,
        };
        
        // For new documents, we don't set a version
        (reputation_key.clone(), rep_data, None)
    };

    // Step 3: Update reputation based on vote
    // The basis reputation is directly affected by votes
    let contribution = vote_value * vote_weight;
    reputation_data.reputation_basis += contribution;
    reputation_data.last_calculation = ic_cdk::api::time();

    // Add info about reputation update
    logger!("info", "[update_reputation_on_vote] Adding vote contribution: target={}, tag={}, contribution={} (vote_value={} * vote_weight={}), new_total_basis={}",
        target_key, 
        tag_key, 
        contribution, 
        vote_value, 
        vote_weight, 
        reputation_data.reputation_basis
        );

    // Step 4: Create updated reputation document
    // No need for document description, we use the key for queries now
    let description = String::new(); // Empty description - using keys for queries
    
    // Step 5: Store updated reputation
    let doc = SetDoc {
        data: encode_doc_data(&reputation_data).map_err(|e| {
            let err_msg = format!("Failed to serialize reputation data: {}", e);
            logger!("error", "{}", err_msg);
            err_msg
        })?,
        description: Some(description),
        version, // Use the original version, Juno will increment it automatically
    };

    // Important: Clone rep_key when passing it to set_doc_store to preserve it for error messages
    match set_doc_store(
        ic_cdk::id(),
        String::from("reputations"),
        rep_key.clone(),
        doc,
    ) {
        Ok(_) => {
            logger!("info", "[update_reputation_on_vote] RESULT: Successfully updated reputation for target={} in tag={}: new_total_basis={}",
                target_key, 
                tag_key, 
                reputation_data.reputation_basis
                );
            Ok(())
        },
        Err(e) => {
            let err_msg = format!("Failed to store reputation: {}", e);
            logger!("error", "{}", err_msg);
            Err(err_msg)
        }
    }
}