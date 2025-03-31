use junobuild_satellite::{list_docs, set_doc_store, get_doc, list_docs_store};
use junobuild_shared::types::list::{ListMatcher, ListParams};
use std::collections::HashMap;
use junobuild_utils::{encode_doc_data, decode_doc_data};
use junobuild_satellite::SetDoc;
use crate::utils::logging::{log_error, log_warn, log_info, log_debug};
use crate::utils::time::calculate_months_between;
use crate::utils::description_helpers::DocumentDescription;
use ic_cdk;

// Import our data structures
use crate::utils::structs::{
    Tag, VoteData, Reputation, ReputationData, VoteWeight,
    AuthorInfo, TagData
};

// Import tag calculations
use crate::utils::tag_calculations::get_active_users_count;

// Import id generator
use crate::utils::id_generator::generate_random_doc_key;

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
    // Create description to query by using the DocumentDescription helper
    let mut desc = DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
    // Query the reputations collection using the formatted description
    let results = list_docs(
        String::from("reputations"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    
    // Return None if no documents found, or the data if found
    if results.items.is_empty() {
        return Ok(None);
    }
    
    // Extract the first document (should be only one)
    let (_, doc) = &results.items[0];
    
    // Decode the reputation data from binary format
    let reputation_data: ReputationData = decode_doc_data(&doc.data)
        .map_err(|e| format!("Failed to deserialize reputation data: {}", e))?;
    
    Ok(Some(reputation_data))
}

/// Gets only the essential reputation data needed for calculations
/// 
/// This is an optimized version of get_user_reputation_data that returns only
/// the minimum data needed for reputation calculations. It's designed to be
/// efficient for use in calculate_user_reputation where we need to process
/// many authors' reputation data.
/// 
/// The function returns:
/// - author_key: For verification purposes
/// - effective_reputation: The score used in calculations
/// - has_voting_power: Whether the user's votes are active
/// - vote_weight: The weight of the user's votes
/// 
/// # Arguments
/// * `user_key` - The unique identifier of the user
/// * `tag_key` - The unique identifier of the tag
/// 
/// # Returns
/// * `Result<Option<AuthorInfo>, String>` - The essential reputation data or an error message
pub async fn get_user_reputation_slim(user_key: &str, tag_key: &str) -> Result<Option<AuthorInfo>, String> {
    // Call the main function to get the full data
    match get_user_reputation_data(user_key, tag_key).await {
        Ok(Some(rep_data)) => {
            // Extract only the necessary fields
            let author_info = AuthorInfo {
                effective_reputation: rep_data.last_known_effective_reputation,
                vote_weight: rep_data.vote_weight,
                votes_active: rep_data.has_voting_power,
            };
            
            Ok(Some(author_info))
        },
        Ok(None) => Ok(None),
        Err(e) => Err(e),
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
    // 1. Query Format: We use the new description format for queries:
    //    - Votes collection: owner=userKey;target=targetUserKey;tag=tagKey;
    //    - Reputations collection: owner=userKey;tag=tagKey;
    //
    // 2. Query Process:
    //    - First query finds all votes by this user in the specified tag
    //    - Second query finds any existing reputation document for this user-tag combination
    //    - Both queries use sanitized keys to avoid issues with special characters
    //
    // 3. Document Handling:
    //    - Keys are sanitized to remove special characters
    //    - This ensures consistent document retrieval regardless of key format
    
    log_info(&format!("[calculate_and_store_vote_weight] START calculating vote weight for user={}, tag={}", user_key, tag_key));
    
    // Step 1: Get Tag Configuration
    // ----------------------------
    log_debug(&format!("[calculate_and_store_vote_weight] Step 1: Getting tag configuration for tag={}", tag_key));
    let tag = get_tag(tag_key).await?;
    log_debug(&format!("[calculate_and_store_vote_weight] Successfully retrieved tag: {}", tag_key));

    // Step 2: Get User's Votes
    // -----------------------
    log_debug(&format!("[calculate_and_store_vote_weight] Step 2: Querying votes by user={} in tag={}", user_key, tag_key));
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = DocumentDescription::new();
    desc.add_owner(user_key);  // Only search for owner=user_key
    let description_filter = desc.build();
    
    // Add a debug log to show the exact search pattern being used
    log_info(&format!(
        "[calculate_and_store_vote_weight] Searching for votes with owner filter: \"{}\"",
        description_filter
    ));
    
    // Execute the votes query using the description filter
    let all_user_votes = list_docs(
        String::from("votes"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    
    // Filter the results to only include votes for the specified tag
    // Instead of trying to create a new ListResults, just filter the items directly
    let user_votes_for_tag: Vec<_> = all_user_votes.items.into_iter()
        .filter(|(_, doc)| {
            // Extract the tag from the description or the data
            if let Some(desc) = &doc.description {
                desc.contains(&format!("tag={};", tag_key))
            } else {
                // If no description, try to get tag from the data (this should never happen)
                match decode_doc_data::<VoteData>(&doc.data) {
                    Ok(vote_data) => vote_data.tag_key == tag_key,
                    Err(_) => false
                }
            }
        })
        .collect();
    
    // Get the count of votes
    let user_votes_count = user_votes_for_tag.len();
    
    log_debug(&format!("[calculate_and_store_vote_weight] Found {} votes by user", user_votes_count));
    // Add INFO level log for number of votes found
    log_info(&format!("[calculate_and_store_vote_weight] Found {} votes by user={} in tag={}", user_votes_count, user_key, tag_key));

    // Step 3: Calculate Total Weighted Votes
    // ------------------------------------
    log_debug(&format!("[calculate_and_store_vote_weight] Step 3: Calculating total weighted votes"));
    let mut total_weighted_votes = 0.0;
    for (_, doc) in &user_votes_for_tag {
        // Get time-based multiplier for this vote using the document's created_at timestamp
        // We don't need to decode the document data since we only use the timestamp
        let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;
        
        // Add to total: base value (1.0) * time multiplier
        total_weighted_votes += 1.0 * time_multiplier;
    }
    log_debug(&format!("[calculate_and_store_vote_weight] Total weighted votes: {}", total_weighted_votes));
    // Add INFO level log for total weighted votes
    log_info(&format!("[calculate_and_store_vote_weight] Total weighted votes for user={}: {}", user_key, total_weighted_votes));

    // Step 4: Calculate Individual Vote Weight
    // -------------------------------------
    log_debug(&format!("[calculate_and_store_vote_weight] Step 4: Calculating individual vote weight"));
    let vote_weight = if total_weighted_votes > 0.0 {
        match VoteWeight::new(1.0 / total_weighted_votes) {
            Ok(weight) => weight,
            Err(e) => {
                log_error(&format!(
                    "[calculate_and_store_vote_weight] Error creating vote weight: user={}, tag={}, error={}",
                    user_key, tag_key, e
                ));
                return Err(format!("Invalid vote weight calculated: {}", e));
            }
        }
    } else {
        // If user has no votes yet, give them a base weight of 1.0 instead of 0.0
        // This allows new users to have some initial influence with their first votes
        log_info(&format!(
            "[calculate_and_store_vote_weight] User={} has no votes in tag={}, setting initial vote_weight=1.0",
            user_key, tag_key
        ));
        match VoteWeight::new(1.0) {
            Ok(weight) => weight,
            Err(e) => {
                log_error(&format!(
                    "[calculate_and_store_vote_weight] Error creating initial vote weight: user={}, tag={}, error={}",
                    user_key, tag_key, e
                ));
                return Err(format!("Invalid vote weight calculated: {}", e));
            }
        }
    };
    log_debug(&format!("[calculate_and_store_vote_weight] Calculated vote weight: {}", vote_weight.value()));
    // Add INFO level log for calculated vote weight
    log_info(&format!("[calculate_and_store_vote_weight] RESULT: Vote weight for user={} in tag={}: {}", user_key, tag_key, vote_weight.value()));

    // Step 5: Query existing reputation document for this user-tag pair
    // -----------------------------------------------------------------
    // We need to find if a reputation document already exists for this user in this tag
    // We use the DocumentDescription helper to create a consistent search pattern
    log_debug(&format!("[calculate_and_store_vote_weight] Step 5: Querying reputation document for user={}, tag={}", user_key, tag_key));
    
    // Create properly formatted description for document lookup
    let mut desc = DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
    // Execute the reputation document query using the canister's ID (for system operations)
    let results = list_docs_store(
        ic_cdk::id(),  // Use canister ID for system-level operations
        String::from("reputations"),
        &ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    )?;  // Propagate any error using ?
    
    // Step 5c: Process query results
    // ------------------------------
    // Validate results - we expect at most one document per user-tag pair
    // The query should return exactly one document if it exists, or none if not
    // This is a validation check to ensure data integrity
    if results.items.len() > 1 {
        let err_msg = format!(
            "[calculate_and_store_vote_weight] ERROR: Found {} documents for user={} and tag={}. Expected only one.", 
            results.items.len(), user_key, tag_key
        );
        log_error(&err_msg);
        return Err(err_msg);
    }

    // Step 6: Prepare Reputation Data
    // ----------------------------
    log_debug(&format!("[calculate_and_store_vote_weight] Step 6: Preparing reputation data"));
    
    // Process existing document or create new data
    // Use a more streamlined approach to reduce redundant decoding
    let (doc_key, mut existing_data, version) = if let Some((key, doc)) = results.items.first() {
        // Document exists, try to decode
        log_debug(&format!(
            "[calculate_and_store_vote_weight] Found existing reputation document: key='{}', version={:?}", 
            key, doc.version
        ));
        
        // Log description if available
        if let Some(desc) = &doc.description {
            log_debug(&format!(
                "[calculate_and_store_vote_weight] Document description: '{}'", 
                desc
            ));
        }
        
        // Decode document
        match decode_doc_data::<ReputationData>(&doc.data) {
            Ok(rep_data) => {
                log_debug(&format!(
                    "[calculate_and_store_vote_weight] Successfully decoded document with key='{}', previous vote_weight={}",
                    key, rep_data.vote_weight.value()
                ));
                (key.clone(), Some(rep_data), doc.version)
            },
            Err(e) => {
                // Failed to decode but document exists
                let err_msg = format!("Failed to deserialize reputation data: {}", e);
                log_error(&err_msg);
                // We'll create new data but keep the document key and version
                (key.clone(), None, doc.version)
            }
        }
    } else {
        // No document exists, we'll create everything from scratch
        log_debug(&format!(
            "[calculate_and_store_vote_weight] No existing reputation document found for user={}, tag={}", 
            user_key, tag_key
        ));
        
        // Generate a new document key using our random key generator
        let new_key = generate_random_doc_key();
        log_debug(&format!("[calculate_and_store_vote_weight] Generated new key={} for reputation document", new_key));
        
        (new_key, None, None)
    };
    
    // Create or update the reputation data
    let mut reputation_data = if let Some(ref mut existing) = existing_data {
        // Log current vote weight before any changes
        log_debug(&format!(
            "[calculate_and_store_vote_weight] Using existing reputation data with vote_weight={}",
            existing.vote_weight.value()
        ));
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
            total_basis_reputation: 0.0,
            total_voting_rewards_reputation: 0.0,
            last_known_effective_reputation: 0.0,
            last_calculation: ic_cdk::api::time(),
            vote_weight: vote_weight.clone(),
            has_voting_power: false,
        }
    };

    // Step 8: Create Complete Document
    // -----------------------------
    log_debug(&format!("[calculate_and_store_vote_weight] Step 8: Creating complete reputation document"));
    
    // Create the description for the reputation document using proper format
    let mut desc = DocumentDescription::new();
    desc.add_owner(&reputation_data.user_key)
        .add_field("tag", &reputation_data.tag_key);
    let description = desc.build();
    
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
    log_debug(&format!("[calculate_and_store_vote_weight] Step 9: Storing reputation document"));
    match encode_doc_data(&reputation.data) {
        Ok(encoded_data) => {
            log_debug(&format!("[calculate_and_store_vote_weight] Using original version: {:?}", 
                version));
            
            let doc = SetDoc {
                data: encoded_data,
                description: Some(reputation.description),
                version, // Use the original version from the document, not incremented
            };

            // Make explicit double-check of version before storing
            log_debug(&format!("[calculate_and_store_vote_weight] Storing document with key={}, version={:?}", 
                reputation.key, doc.version));

            match set_doc_store(
                ic_cdk::id(),  // Use canister's Principal ID as caller
                String::from("reputations"),
                reputation.key.clone(),
                doc,
            ) {
                Ok(_) => {
                    log_info(&format!(
                        "[calculate_and_store_vote_weight] SUCCESS: {} reputation document with key={}, version={:?}, vote_weight={}",
                        if existing_data.is_some() { "Updated" } else { "Created" },
                        reputation.key, version, vote_weight.value()
                    ));
                    Ok(vote_weight.value())
                },
                Err(e) => {
                    // Enhanced error logging for version conflicts
                    log_error(&format!(
                        "[calculate_and_store_vote_weight] ERROR: Failed to store reputation document: key={}, attempted_version={:?}, error={}",
                        reputation.key, version, e
                    ));
                    
                    // Double-check version with direct document retrieval
                    if let Some(current_doc) = get_doc(String::from("reputations"), reputation.key.clone()) {
                        log_error(&format!(
                            "[calculate_and_store_vote_weight] VERIFICATION: Current document in database has version={:?}, we attempted to use version={:?}",
                            current_doc.version, version
                        ));
                    } else {
                        log_error(&format!(
                            "[calculate_and_store_vote_weight] VERIFICATION: Document with key={} not found during verification",
                            reputation.key
                        ));
                    }
                    
                    Err(format!("Failed to store reputation: {}", e))
                }
            }
        }
        Err(e) => {
            log_error(&format!(
                "[calculate_and_store_vote_weight] Error serializing reputation data: user={}, tag={}, error={}",
                user_key, tag_key, e
            ));
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
pub async fn calculate_user_reputation(user_key: &str, tag_key: &str) -> Result<ReputationData, String> {
    // Add start log message
    log_info(&format!("[calculate_user_reputation] START calculating reputation for user={}, tag={}", user_key, tag_key));
    
    // Get the tag once at the start - we'll reuse this for all calculations
    let tag = get_tag(tag_key).await?;

    // Step 1: Query Votes
    // ----------------------
    // Query all votes targeted the user under a specified tag
    // We use the description field to filter votes efficiently
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
    let votes = list_docs(
        String::from("votes"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Store vote items in a vector to avoid multiple moves
    let vote_items: Vec<_> = votes.items;

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
                log_warn(&format!(
                    "Error decoding vote data: {}",
                    e
                ));
                continue;
            }
        }
    }

    // Add vote count info log
    log_info(&format!(
        "[calculate_user_reputation] Found {} votes targeting user={} in tag={}", vote_items.len(), user_key, tag_key));

    // Add detailed vote and reputation calculation log
    log_info(&format!(
        "[calculate_user_reputation] Found {} votes for user={} in tag={} (base reward per vote: {})",
        vote_data_list.len(),
        user_key,
        tag_key,
        tag.data.vote_reward
    ));

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
    log_info(&format!("[calculate_user_reputation] Checking {} unique authors who voted on user={}", vote_items.len(), user_key));
    
    // Process each vote to get author information
    for (_, doc) in &vote_items {
        let vote_data: VoteData = decode_doc_data(&doc.data)
            .map_err(|e| format!("Failed to deserialize vote: {}", e))?;

        // Skip if we already have this author's information
        if author_index.contains_key(&vote_data.author_key) {
            continue;
        }

        // Get author's reputation data
        match get_user_reputation_slim(&vote_data.author_key, tag_key).await {
            Ok(Some(author_info)) => {
                // Skip if author's votes are not active
                if !author_info.votes_active {
                    log_warn(&format!(
                        "Skipping inactive author: author={}, tag={}",
                        vote_data.author_key, tag_key
                    ));
                    // Add more detailed info about why author is inactive
                    log_info(&format!(
                        "[calculate_user_reputation] Author={} is inactive in tag={}: reputation={}, has_voting_power={}",
                        vote_data.author_key, tag_key, author_info.effective_reputation, author_info.votes_active
                    ));
                    continue;
                }
                
                // Add info about active author - do this BEFORE inserting into HashMap
                log_info(&format!(
                    "[calculate_user_reputation] Active author={} in tag={}: reputation={}, vote_weight={}",
                    vote_data.author_key, tag_key, author_info.effective_reputation, author_info.vote_weight.value()
                ));

                // Store author information AFTER logging
                author_index.insert(
                    vote_data.author_key.clone(),
                    author_info, // Use the AuthorInfo directly
                );
            }
            Ok(None) => {
                log_warn(&format!(
                    "No reputation data found for author in calculate_user_reputation: author={}, tag={}",
                    vote_data.author_key, tag_key
                ));
            }
            Err(e) => {
                log_error(&format!(
                    "[calculate_user_reputation] Error getting author reputation: author={}, tag={}, error={}",
                    vote_data.author_key, tag_key, e
                ));
                continue;
            }
        }
    }

    // If we have no valid authors (all had inactive votes), return early
    if author_index.is_empty() {
        return Ok(ReputationData {
            user_key: user_key.to_string(),
            tag_key: tag_key.to_string(),
            total_basis_reputation: 0.0,
            total_voting_rewards_reputation: 0.0,
            last_known_effective_reputation: 0.0,
            last_calculation: ic_cdk::api::time(),
            vote_weight: VoteWeight::new(0.0)
                .map_err(|e| format!("Invalid vote weight calculated: {}", e))?,
            has_voting_power: false,
        });
    }

    // Step 3: Basis Reputation Calculation
    // -----------------------------------
    // Calculate total basis reputation from all received votes
    // For each vote, calculate its contribution by multiplying:
    // - Base value (+1 for positive, -1 for negative)
    // - Author's effective reputation
    // - Author's vote weight
    // - Time-based multiplier from tag rules
    // Then sum all vote contributions to get total_basis_reputation
    let mut total_basis_reputation = 0.0;

    // Iterate through all received votes
    for (_, doc) in &vote_items {
        let vote_data: VoteData = decode_doc_data(&doc.data)
            .map_err(|e| format!("Failed to deserialize vote: {}", e))?;

        // Get author's information from our index
        let author_info = match author_index.get(&vote_data.author_key) {
            Some(info) => info,
            None => {
                log_warn(&format!(
                    "[calculate_user_reputation - Iterate Votes] Warning: Author info not found in index to calculate vote, author={}",
                    vote_data.author_key
                ));
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
        total_basis_reputation += final_contribution;
    }

    // Add info about basis reputation total
    log_info(&format!("[calculate_user_reputation] Calculated total_basis_reputation={} for user={} in tag={}", total_basis_reputation, user_key, tag_key));

    // Step 4: Trust Status Check
    // -------------------------
    // Compare total_basis_reputation against tag's minimum threshold
    // to determine if user has voting power
    let has_voting_power = total_basis_reputation >= tag.data.reputation_threshold;

    // Step 5: Voting Rewards Calculation
    // --------------------------------
    // Retrieve all votes where author is the user being calculated
    // Get voting reward value from tag's configuration (tag.vote_reward)
    // For each vote made by user:
    // - Calculate reward = tag.vote_reward * time multiplier
    // Sum all rewards to get total_voting_rewards_reputation
    // Query votes where this user is the author
    // Format is now: "owner=user_key;tag=tag_key;"
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();

    let user_votes = list_docs(
        String::from("votes"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Calculate rewards for each vote cast by the user
    let mut total_voting_rewards_reputation = 0.0;
    for (_, doc) in user_votes.items {
        // We only need the timestamp, not the vote data itself
        // Get time-based multiplier for this vote using the document's created_at timestamp
        let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;
        
        let reward = tag.data.vote_reward * time_multiplier;
        total_voting_rewards_reputation += reward;
        
        // Add detailed log for each vote reward calculation
        log_info(&format!(
            "[calculate_user_reputation] VOTE_REWARD: author={}, voteR={} (base_reward={} * time_multiplier={}), created_at={}",
            user_key, reward, tag.data.vote_reward, time_multiplier, doc.created_at
        ));
    }

    // Add info about voting rewards total
    // Get active users count before using it in any logs
    let active_users = get_active_users_count(tag_key).await?;
    
    log_info(&format!(
        "[calculate_user_reputation] SUMMARY: user={}, tag={}, basisR={}, voteR={}, active_users={}",
        user_key, tag_key, total_basis_reputation, total_voting_rewards_reputation, active_users
    ));

    // Step 6: Final Reputation Calculation
    // ----------------------------------
    // Calculate final effective reputation based on trust status and bootstrap phase
    // Active users is already calculated above
    // Add detailed info about active users count
    log_info(&format!(
        "[calculate_user_reputation] Found {} active users in tag={} (min_users_for_threshold={})",
        active_users, tag_key, tag.data.min_users_for_threshold
    ));

    // FIXED: Always include voting rewards in effective reputation
    // This ensures users can gain reputation by voting even if their votes don't count for others
    let effective_reputation = total_basis_reputation + total_voting_rewards_reputation;
    
    // Log appropriate message based on voting power status
    if has_voting_power {
        log_info(&format!(
            "[calculate_user_reputation] TRUSTED: user={} has voting power in tag={}",
            user_key, tag_key
        ));
    } else if active_users < tag.data.min_users_for_threshold {
        log_info(&format!(
            "[calculate_user_reputation] BOOTSTRAP: tag={} is in bootstrap phase ({} < {} users)",
            tag_key, active_users, tag.data.min_users_for_threshold
        ));
    } else {
        log_info(&format!(
            "[calculate_user_reputation] UNTRUSTED: user={} lacks voting power but still gets voting rewards",
            user_key
        ));
    }
    
    log_info(&format!(
        "[calculate_user_reputation] Effective reputation: basisR={} + voteR={} = totalR={}",
        total_basis_reputation, total_voting_rewards_reputation, effective_reputation
    ));

    // Step 7: Storage
    // --------------
    // This step handles storing the calculated reputation data in the database.
    // The process follows these steps:
    // 1. Create the document description using DocumentDescription helper, we will use that for the query.
    // 2. Query the database 'reputations' for if a document already exists with this description and store it in a variabla for the document and another for the data.
    // 3. If the document doesn't exist, generate a unique document key using nanoid()
    // 4. Now generate the document in the format we need for storage.
    // 5. Store with proper version handling
    log_debug(&format!("[calculate_user_reputation] Step 7: Storing reputation data for user={}, tag={}", user_key, tag_key));

    // Step 7.1: Create description and query for existing document
    // --------------------------------------------------------
    let mut desc = DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
    log_debug(&format!("[calculate_user_reputation] Searching for existing reputation document with description: {}", description_filter));
    
    let existing_docs = list_docs(
        String::from("reputations"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter.clone()),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Step 7.2: Prepare reputation data and document details
    // ---------------------------------------------------
    // Here we handle three cases:
    // 1. Found document and successfully decoded it - use its key, version, and vote_weight
    // 2. Found document but failed to decode - use its key and version, but default vote_weight
    // 3. No document found - generate a new key and use default vote_weight

    // First, check if document exists and try to decode it
    let (doc_key, existing_data, version) = if let Some((existing_key, existing_doc)) = existing_docs.items.first() {
        // Found a document, try to decode it
        match decode_doc_data::<ReputationData>(&existing_doc.data) {
            Ok(decoded_data) => {
                // Case 1: Successfully decoded existing document
                log_info(&format!(
                    "[calculate_user_reputation] Using existing vote_weight={} from reputation document for user={} in tag={}",
                    decoded_data.vote_weight.value(), user_key, tag_key
                ));
                (existing_key.clone(), Some(decoded_data), existing_doc.version)
            },
            Err(e) => {
                // Case 2: Document exists but failed to decode
                log_error(&format!(
                    "[calculate_user_reputation] Failed to decode existing reputation data, using default vote_weight=0.0: error={}",
                    e
                ));
                (existing_key.clone(), None, existing_doc.version)
            }
        }
    } else {
        // Case 3: No document found
        log_info(&format!(
            "[calculate_user_reputation] No existing document found for user={} in tag={}, creating new",
            user_key, tag_key
        ));
        // Generate a new random document key instead of using canister ID
        (generate_random_doc_key(), None, None)
    };

    // Determine vote_weight to use based on existing data
    let vote_weight = match &existing_data {
        Some(data) => data.vote_weight.clone(),
        None => {
            // Use default weight of 0.0 if no existing document or decode failed
            VoteWeight::new(0.0)
                .map_err(|e| format!("Invalid vote weight: {}", e))?
        }
    };

    // Create the reputation data with all calculated values
    let reputation_data = ReputationData {
        user_key: user_key.to_string(),
        tag_key: tag_key.to_string(),
        total_basis_reputation,
        total_voting_rewards_reputation,
        last_known_effective_reputation: effective_reputation,
        last_calculation: ic_cdk::api::time(),
        vote_weight: vote_weight.clone(), // Use the existing vote_weight variable we determined earlier
        has_voting_power,
    };

    // Step 7.3: Store document
    // ----------------------
    match encode_doc_data(&reputation_data) {
        Ok(encoded_data) => {
            // Determine new version (increment if exists)
            let new_version = version.map(|v| v + 1);
            
            let doc = SetDoc {
                data: encoded_data,
                description: Some(description_filter),
                version: new_version,
            };

            // Add log to show what we're about to store
            log_info(&format!(
                "[calculate_user_reputation] {}: key={}, basisR={}, voteR={}, totalR={}, vote_weight={}",
                if existing_data.is_some() { "UPDATING" } else { "CREATING" },
                doc_key, reputation_data.total_basis_reputation, 
                reputation_data.total_voting_rewards_reputation,
                reputation_data.last_known_effective_reputation,
                vote_weight.value()
            ));

            match set_doc_store(
                ic_cdk::id(),
                String::from("reputations"),
                doc_key.clone(),
                doc,
            ) {
                Ok(_) => {
                    log_info(&format!(
                        "[calculate_user_reputation] SUCCESS: {} reputation for user={} in tag={}: basisR={}, voteR={}, totalR={}",
                        if existing_data.is_some() { "updated" } else { "created" },
                        user_key, tag_key, total_basis_reputation, total_voting_rewards_reputation, effective_reputation
                    ));
                    Ok(reputation_data)
                },
                Err(e) => {
                    log_error(&format!(
                        "[calculate_user_reputation] Failed to {} reputation document: key={}, error={}",
                        if existing_data.is_some() { "update" } else { "create" },
                        doc_key, e
                    ));
                    Err(format!("Failed to {} reputation: {}", 
                        if existing_data.is_some() { "update" } else { "create" }, e))
                }
            }
        },
        Err(e) => {
            log_error(&format!(
                "[calculate_user_reputation] Failed to encode reputation data: {}",
                e
            ));
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
    let tag = get_tag(tag_key).await?;
    
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

/// Gets a tag by its key
/// 
/// This function retrieves a tag document directly by its key using Juno's get_doc.
/// It's more efficient than querying by description since it uses direct key lookup.
/// 
/// # Arguments
/// * `tag_key` - The unique identifier of the tag to retrieve
/// 
/// # Returns
/// * `Result<Tag, String>` - The tag document or an error message
async fn get_tag(tag_key: &str) -> Result<Tag, String> {
    // Get the document from Juno
    let tag_doc = get_doc(
        String::from("tags"),      // Collection name first
        tag_key.to_string(),       // Document key second
    )
    .ok_or_else(|| {
        log_error(&format!(
            "[get_tag] Tag not found: tag={}",
            tag_key
        ));
        format!("Tag not found: {}", tag_key)
    })?;

    // Decode the tag data into TagData, not Tag
    let tag_data: TagData = decode_doc_data(&tag_doc.data)
        .map_err(|e| {
            log_error(&format!(
                "[get_tag] Failed to deserialize tag data: tag={}, error={}",
                tag_key, e
            ));
            format!("Failed to deserialize tag: {}", e)
        })?;
        
    // Construct a full Tag with both metadata and data
    let tag = Tag {
        key: tag_key.to_string(),
        description: tag_doc.description.unwrap_or_default(),
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
    log_info(&format!("[update_reputation_on_vote] START updating reputation for target={}, tag={}, vote_value={}, vote_weight={}", target_key, tag_key, vote_value, vote_weight));
    
    // Step 1: Get or create reputation document for target user in this tag
    // We create a description filter using the format: owner=targetKey;tag=tagKey;
    log_info(&format!("[update_reputation_on_vote] START updating reputation for target={}, tag={}, vote_value={}, vote_weight={}", target_key, tag_key, vote_value, vote_weight));
    
    // Create the exact description to search for using the DocumentDescription helper
    let mut desc = DocumentDescription::new();
    desc.add_owner(target_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
    // Query the reputations collection to find any existing document for this user in this tag
    let results = list_docs(
        String::from("reputations"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    
    // Step 2: Process the query results and prepare document data
    // --------------------------------------------------------
    
    // Step 2.1: Check if a reputation document exists
    // We look for a document with our description filter in the results
    let existing_doc = results.items.first();

    // Step 2.2: If document exists, decode its data and prepare for update
    // We'll increment the version number for the update
    let (rep_key, mut reputation_data, version) = if let Some((doc_key, doc)) = existing_doc {
        // Decode the existing reputation data
        let rep_data: ReputationData = decode_doc_data(&doc.data)
            .map_err(|e| {
                let err_msg = format!("Failed to deserialize reputation data: {}", e);
                log_error(&err_msg);
                err_msg
            })?;
        
        // Return the key, data, and original version (don't increment yet)
        (doc_key.clone(), rep_data, doc.version)
    } else {
        // Step 2.3: If no document exists, create new data with default values
        let default_weight = VoteWeight::new(0.0).unwrap_or_else(|_| {
            log_error("[update_reputation_on_vote] Failed to create default vote weight, using 0.0");
            VoteWeight::new(0.0).unwrap() // This should never fail for 0.0
        });
        
        let rep_data = ReputationData {
            user_key: target_key.to_string(),
            tag_key: tag_key.to_string(),
            total_basis_reputation: 0.0,
            total_voting_rewards_reputation: 0.0,
            last_known_effective_reputation: 0.0,
            last_calculation: ic_cdk::api::time(),
            vote_weight: default_weight,
            has_voting_power: false,
        };
        
        // For new documents, we don't set a version
        (target_key.to_string(), rep_data, None)
    };

    // Step 3: Update reputation based on vote
    // The basis reputation is directly affected by votes
    let contribution = vote_value * vote_weight;
    reputation_data.total_basis_reputation += contribution;
    reputation_data.last_calculation = ic_cdk::api::time();

    // Add info about reputation update
    log_info(&format!(
        "[update_reputation_on_vote] Adding vote contribution: target={}, tag={}, contribution={} (vote_value={} * vote_weight={}), new_total_basis={}",
        target_key, tag_key, contribution, vote_value, vote_weight, reputation_data.total_basis_reputation
    ));

    // Step 4: Create updated reputation document
    // Create the description using proper format
    let mut desc = DocumentDescription::new();
    desc.add_owner(&reputation_data.user_key)
        .add_field("tag", &reputation_data.tag_key);
    let description = desc.build();
    
    // Step 5: Store updated reputation
    let doc = SetDoc {
        data: encode_doc_data(&reputation_data).map_err(|e| {
            let err_msg = format!("Failed to serialize reputation data: {}", e);
            log_error(&err_msg);
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
            log_info(&format!(
                "[update_reputation_on_vote] RESULT: Successfully updated reputation for target={} in tag={}: new_total_basis={}",
                target_key, tag_key, reputation_data.total_basis_reputation
            ));
            Ok(())
        },
        Err(e) => {
            let err_msg = format!("Failed to store reputation: {}", e);
            log_error(&err_msg);
            Err(err_msg)
        }
    }
}