use junobuild_satellite::{list_docs, set_doc_store, get_doc};
use junobuild_shared::types::list::{ListMatcher, ListParams};
use serde_json::Value;
use std::collections::HashMap;
use junobuild_utils::{encode_doc_data, decode_doc_data};
use junobuild_satellite::SetDoc;
use crate::utils::logging::{log_error, log_warn, log_info, log_debug};
use crate::utils::time::{calculate_months_between, get_period_for_timestamp};

// Import our data structures
use crate::utils::structs::{
    Tag, VoteData, Reputation, ReputationData, TimePeriod, VoteWeight,
    AuthorInfo
};

// Import tag calculations
use crate::utils::tag_calculations::get_active_users_count;

/// Gets a user's complete reputation data for a specific tag
/// 
/// This function queries the reputations collection to find a user's 
/// cached reputation data for a specific tag. The reputation document is identified 
/// by a description field that combines the user_key and tag_key in the format 
/// "[owner:{user_key}],[tag:{tag_key}]".
/// 
/// The function returns:
/// - None if no reputation document exists (user hasn't received any votes in this tag)
/// - Some(ReputationData) containing:
///   - total_basis_reputation: Reputation from received votes
///   - total_voting_rewards_reputation: Reputation from casting votes
///   - last_known_effective_reputation: Final reputation score
///   - last_calculation: Timestamp of last calculation
///   - vote_weight: User's vote weight in this tag
///   - has_voting_power: Whether user's votes are active
/// 
/// # Arguments
/// * `user_key` - The unique identifier of the user
/// * `tag_key` - The unique identifier of the tag
/// 
/// # Returns
/// * `Result<Option<ReputationData>, String>` - The user's complete reputation data or an error message
pub async fn get_user_reputation_data(user_key: &str, tag_key: &str) -> Result<Option<ReputationData>, String> {
    // Query the reputations collection using a description filter
    // The description field format is "[owner:{user_key}],[tag:{tag_key}]"
    // This ensures we get the specific reputation document for this user in this tag
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
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

    // If no reputation document exists, return None
    // This means the user hasn't received any votes in this tag yet
    if results.items.is_empty() {
        return Ok(None);
    }

    // Get the first (and should be only) document
    // We use .first() on items which returns Option<(&String, &Doc)>
    let (_doc_key, doc) = results.items.first()
        .ok_or_else(|| {
            let err_msg = format!("No reputation document found for user {} in tag {}", user_key, tag_key);
            log_error(&err_msg);
            err_msg
        })?;

    // Convert the document into our Reputation struct
    // The document contains the user's reputation data including:
    // - total_basis_reputation (from received votes)
    // - total_voting_rewards_reputation (from casting votes)
    // - last_known_effective_reputation (the final cached score)
    // - last_calculation (timestamp of last calculation)
    // - vote_weight (user's vote weight in this tag)
    // - has_voting_power (whether their votes are active)
    let reputation: Reputation = decode_doc_data(&doc.data)
        .map_err(|e| {
            let err_msg = format!("Failed to deserialize reputation data: {}", e);
            log_error(&err_msg);
            err_msg
        })?;
    
    // Return the complete reputation data
    Ok(Some(reputation.data))
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

/// Calculates and stores a user's vote weight for a specific tag
/// 
/// This function calculates a single normalized weight value that represents how much each of
/// a user's votes should count, relative to their total voting power. This is a critical part
/// of the reputation system's anti-inflation and bot prevention mechanisms.
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
/// ```
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
/// ```
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
/// # Arguments
/// * `user_key` - The key of the user whose vote weight is being calculated
/// * `tag_key` - The key of the tag to calculate vote weight for
/// 
/// # Returns
/// * `Result<f64, String>` - The calculated vote weight or an error message
pub async fn calculate_and_store_vote_weight(user_key: &str, tag_key: &str) -> Result<f64, String> {
    // Step 1: Get Tag Configuration
    // ----------------------------
    // - Retrieve tag settings for time period calculations
    // - Tag contains configuration for vote multipliers based on time periods
    // - Used to determine how much each vote counts based on its age
    let tag = get_tag(tag_key).await?;

    // Step 2: Get User's Votes
    // -----------------------
    // - Query all votes where this user is the AUTHOR (they made the vote)
    // - Uses description format: "[owner:{author_key}],[tag:{tag_key}]"
    // - This gives us all votes the user has cast in this tag
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
    desc.add_owner(user_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
    let results = list_docs(
        String::from("votes"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Step 3: Calculate Total Weighted Votes
    // ------------------------------------
    // - Sum up all votes with their time-based multipliers
    // - Each vote starts with base value of 1.0
    // - Multiplier increases/decreases based on vote age
    // - Newer votes generally have higher multipliers
    let mut total_weighted_votes = 0.0;
    for (_, doc) in results.items {
        // Decode vote data from binary format
        let vote_data: VoteData = decode_doc_data(&doc.data)
            .map_err(|e| {
                let err_msg = format!("Failed to deserialize vote data: {}", e);
                log_error(&err_msg);
                err_msg
            })?;
        
        // Get time-based multiplier for this vote using the document's created_at timestamp
        let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;
        
        // Add to total: base value (1.0) * time multiplier
        total_weighted_votes += 1.0 * time_multiplier;
    }

    // Step 4: Calculate Individual Vote Weight
    // -------------------------------------
    // - If user has votes: weight = 1/total_weighted_votes
    //   This ensures their total voting power = 100%
    // - If no votes: weight = 0
    //   New users start with no voting power
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
        match VoteWeight::new(0.0) {
            Ok(weight) => weight,
            Err(e) => {
                log_error(&format!(
                    "[calculate_and_store_vote_weight] Error creating zero vote weight: user={}, tag={}, error={}",
                    user_key, tag_key, e
                ));
                return Err(format!("Invalid vote weight calculated: {}", e));
            }
        }
    };

    // Step 5: Get Existing Reputation Document
    // -------------------------------------
    // - Look for user's existing reputation in this tag
    // - Query format: "[owner:{user_key}],[tag:{tag_key}]"
    // - Will be used to update or create reputation data
    
    // Create properly formatted description for reputation document query
    let mut rep_desc = crate::utils::description_helpers::DocumentDescription::new();
    rep_desc.add_owner(user_key)
           .add_field("tag", tag_key);
    let rep_filter = rep_desc.build();
    
    let results = list_docs(
        String::from("reputations"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(rep_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Step 6: Prepare Reputation Data
    // ----------------------------
    // - Either update existing reputation or create new
    // - For existing: decode and update vote weight
    // - For new: initialize with default values
    let mut reputation_data = if let Some((doc_key, doc)) = results.items.first() {
        // If exists, decode existing reputation
        match decode_doc_data(&doc.data) {
            Ok(rep) => {
                let rep: Reputation = rep;
                rep.data
            },
            Err(e) => {
                log_error(&format!(
                    "[calculate_and_store_vote_weight] Error deserializing reputation document: key={}, error={}",
                    doc_key, e
                ));
                return Err(format!("Failed to deserialize reputation: {}", e));
            }
        }
    } else {
        // If not exists, create new with default values
        match VoteWeight::new(0.0) {
            Ok(weight) => ReputationData {
                user_key: user_key.to_string(),
                tag_key: tag_key.to_string(),
                total_basis_reputation: 0.0,
                total_voting_rewards_reputation: 0.0,
                last_known_effective_reputation: 0.0,
                last_calculation: ic_cdk::api::time(),
                vote_weight: weight,
                has_voting_power: false,
            },
            Err(e) => {
                log_error(&format!(
                    "[calculate_and_store_vote_weight] Error creating initial vote weight: user={}, tag={}, error={}",
                    user_key, tag_key, e
                ));
                return Err(format!("Invalid vote weight calculated: {}", e));
            }
        }
    };

    // Step 7: Update Reputation Data
    // ---------------------------
    // - Set the newly calculated vote weight (clone it since we need it later)
    // - Update the last calculation timestamp
    reputation_data.vote_weight = vote_weight.clone();
    reputation_data.last_calculation = ic_cdk::api::time();

    // Step 8: Create Complete Document
    // -----------------------------
    // - Build full reputation document with metadata
    // - Include key, description, timestamps, etc.
    
    // Create the description for the reputation document using proper format
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
    desc.add_owner(&reputation_data.user_key)
        .add_field("tag", &reputation_data.tag_key);
    let description = desc.build();
    
    let reputation = Reputation {
        key: format!("rep_{}_{}", user_key, tag_key),
        description,
        owner: ic_cdk::id(),  // Use canister's Principal ID as owner
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        version: 1,
        data: reputation_data.clone(),
    };

    // Step 9: Store Document
    // -------------------
    // - Encode reputation data to binary
    // - Store in Juno datastore
    // - Return the calculated vote weight on success
    match encode_doc_data(&reputation.data) {
        Ok(encoded_data) => {
            let doc = SetDoc {
                data: encoded_data,
                description: Some(reputation.description),
                version: Some(1),  // For new documents
            };

            match set_doc_store(
                ic_cdk::id(),  // Use canister's Principal ID as caller
                String::from("reputations"),
                reputation.key.clone(),
                doc,
            ) {
                Ok(_) => Ok(vote_weight.value()),
                Err(e) => {
                    log_error(&format!(
                        "[calculate_and_store_vote_weight] Error storing reputation document: key={}, error={}",
                        reputation.key, e
                    ));
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
///    - Format is now: "[owner:{user_key}],[tag:{tag_key}]"
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
    // Get the tag once at the start - we'll reuse this for all calculations
    let tag = get_tag(tag_key).await?;

    // Step 1: Query Votes
    // ----------------------
    // Query all votes targeted the user under a specified tag
    // We use the description field to filter votes efficiently
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
    desc.add_field("target", user_key)
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
                    continue;
                }

                // Store author information
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

    // Step 4: Trust Status Check
    // -------------------------
    // Compare total_basis_reputation against tag's minimum threshold
    // to determine if user has voting power
    let has_voting_power = total_basis_reputation >= tag.data.reputation_threshold;

    // Step 5: Voting Rewards Calculation
    // --------------------------------
    // Retrieve all votes where author is the user being calculated and uses the tag key
    // Get voting reward value from tag's configuration (tag.vote_reward)
    // For each vote made by user:
    // - Calculate reward = tag.vote_reward * time multiplier
    // Sum all rewards to get total_voting_rewards_reputation
    // Query votes where this user is the author
    // Format is now: "[owner:{user_key}],[tag:{tag_key}]"
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
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
        let vote_data: VoteData = decode_doc_data(&doc.data)
            .map_err(|e| format!("Failed to deserialize vote: {}", e))?;
        
        // Get time-based multiplier for this vote using the document's created_at timestamp
        let time_multiplier = get_period_multiplier(doc.created_at, tag_key).await?;
        
        let reward = tag.data.vote_reward * time_multiplier;
        total_voting_rewards_reputation += reward;
    }

    // Step 6: Final Reputation Calculation
    // ----------------------------------
    // Calculate final effective reputation based on trust status and bootstrap phase
    let active_users = get_active_users_count(tag_key).await?;
    let effective_reputation = if has_voting_power || active_users < tag.data.min_users_for_threshold {
        // If user has voting power OR community is in bootstrap phase:
        // effective_reputation = total_basis_reputation + total_voting_rewards_reputation
        total_basis_reputation + total_voting_rewards_reputation
    } else {
        // Otherwise:
        // effective_reputation = total_basis_reputation
        total_basis_reputation
    };

    // Step 7: Storage
    // --------------
    // Store all calculated values in the reputations collection
    let reputation_data = ReputationData {
        user_key: user_key.to_string(),
        tag_key: tag_key.to_string(),
        total_basis_reputation,
        total_voting_rewards_reputation,
        last_known_effective_reputation: effective_reputation,
        last_calculation: ic_cdk::api::time(),
        vote_weight: VoteWeight::new(0.0)
            .map_err(|e| format!("Invalid vote weight calculated: {}", e))?,
        has_voting_power,
    };

    // Create or update the reputation document
    let reputation = Reputation {
        key: format!("rep_{}_{}", user_key, tag_key),
        description: {
            let mut desc = crate::utils::description_helpers::DocumentDescription::new();
            desc.add_owner(user_key)
               .add_field("tag", tag_key);
            desc.build()
        },
        owner: ic_cdk::id(),  // Use canister's Principal ID as owner
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        version: 1,
        data: reputation_data.clone(),
    };

    // Attempt to set the document
    match junobuild_satellite::set_doc_store(
        ic_cdk::id(),  // Use canister's Principal ID as caller
        String::from("reputations"),  // Collection
        reputation.key.clone(),  // Key
        SetDoc {
            data: encode_doc_data(&reputation.data).map_err(|e| {
                log_error(&format!("[calculate_user_reputation] Failed to encode reputation data: {}", e));
                e.to_string()
            })?,
            description: Some({
                let mut desc = crate::utils::description_helpers::DocumentDescription::new();
                desc.add_owner(&reputation.data.user_key)
                   .add_field("tag", &reputation.data.tag_key);
                desc.build()
            }),
            version: Some(1),  // For new documents
        }
    ) {
        Ok(_) => {
            log_info(&format!(
                "[calculate_user_reputation] Updated reputation document: user={}, tag={}, score={}",
                user_key, tag_key, reputation.data.last_known_effective_reputation
            ));
            Ok(reputation.data)
        },
        Err(e) => {
            log_error(&format!(
                "[calculate_user_reputation] Failed to store reputation: {}",
                e
            ));
            Err(format!("Failed to store reputation: {}", e))
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
        tag_key.to_string(),
        String::from("tags"),
    )
    .ok_or_else(|| {
        log_error(&format!(
            "[get_tag] Tag not found: tag={}",
            tag_key
        ));
        format!("Tag not found: {}", tag_key)
    })?;

    // Decode the tag data directly
    let tag: Tag = decode_doc_data(&tag_doc.data)
        .map_err(|e| {
            log_error(&format!(
                "[get_tag] Failed to deserialize tag data: tag={}, error={}",
                tag_key, e
            ));
            format!("Failed to deserialize tag: {}", e)
        })?;

    Ok(tag)
}

/// Updates a user's reputation when they receive a vote
///
/// This function is called when a user receives a vote in a specific tag.
/// It looks up or creates a reputation document for the user, then updates
/// the reputation score based on the vote's impact.
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
    // Step 1: Get or create reputation document for target user in this tag
    // Query format: "[owner:{target_key}],[tag:{tag_key}]"
    
    // Create properly formatted description using the DocumentDescription helper
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
    desc.add_owner(target_key)
        .add_field("tag", tag_key);
    let description_filter = desc.build();
    
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

    // Step 2: Process existing reputation data or create new
    let (rep_key, mut reputation_data, version) = if let Some((doc_key, doc)) = results.items.first() {
        // If reputation document exists, decode it
        let reputation: Reputation = decode_doc_data(&doc.data)
            .map_err(|e| {
                let err_msg = format!("Failed to deserialize reputation data: {}", e);
                log_error(&err_msg);
                err_msg
            })?;
        
        (doc_key.clone(), reputation.data, doc.version)
    } else {
        // If no reputation document exists, create a new one with default values
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
        
        (format!("rep_{}_{}", target_key, tag_key), rep_data, Some(0))
    };

    // Step 3: Update reputation based on vote
    // The basis reputation is directly affected by votes
    reputation_data.total_basis_reputation += vote_value * vote_weight;
    reputation_data.last_calculation = ic_cdk::api::time();

    // Step 4: Create updated reputation document
    // Create the description using proper format
    let mut desc = crate::utils::description_helpers::DocumentDescription::new();
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
        version: version,
    };

    // Important: Clone rep_key when passing it to set_doc_store to preserve it for error messages
    match set_doc_store(
        ic_cdk::id(),
        String::from("reputations"),
        rep_key.clone(), // Clone here to preserve the value for the error message
        doc,
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = format!("Failed to store reputation: {}", e);
            log_error(&err_msg);
            Err(err_msg)
        }
    }
}
