# Reputator: App for tracking user reputation and bot prevention

---

# **User Reputation System Documentation**

## **Introduction**

This document outlines the design and implementation of a user reputation system intended to serve as a decentralized platform for communities and applications. The system enables users to rate each other using positive or negative votes associated with specific reputation tags (e.g., `#teamplay`, `#crypto`). The aim is to create a versatile and self-regulating ecosystem where reputation is earned and maintained through active participation and recognition by others.

---

## **Goals of the System**

1. **Flexible Tag-Based Reputation:**
    - Allow any community to create and use custom `#tags` as they see fit.
    - Enable global tags (e.g., `#crypto`) and app-specific tags (e.g., `#appname`) to coexist.
    - Facilitate the formation of in-game factions, discussion forums, or any group identity through tags.
2. **Decentralized Trust Management:**
    - Communities can trust or distrust certain users or "trust-providers" based on collective upvotes.
    - Trust-providers can influence reputations according to automatic rules and tracking under community oversight.
    - If a group misuses a tag, the community can downvote them, reducing their voting power.
3. **API Integration for Diverse Applications:**
    - Serve as an API that multiple apps can integrate and utilize.
    - Enable features like:
        - In-game dynamics (e.g., marking players with certain reputations).
        - Forum moderation rights based on reputation within a tag.
        - Decentralized content moderation driven by community votes.
4. **Authentic Reviews and Feedback:**
    - Function similarly to platforms like Trustpilot for app and service reviews.
    - Allow users to rate platforms, services, and each other, generating genuine data free from fake reviews.
    - Support for web3 marketplaces where transaction partners rate each other, enhancing trustworthiness.
5. **Protection Against Bad Actors:**
    - Implement safeguards to prevent exploitation by bots, scammers, or malicious users.
    - Ensure that new users cannot game the system before reaching a trusted level.
6. **Community Self-Regulation:**
    - Encourage communities to govern themselves by upvoting positive contributors and downvoting bad actors.
    - Enable the system to adjust reputations dynamically based on community feedback.

---

## **System Overview**

### **Voting Mechanics**

- **Vote Types:** Users can cast positive (`+1`) or negative (`1`) votes on other users.
- **Reputation Tags:** Each vote is associated with a specific reputation tag (e.g., `#teamplay`, `#crypto`).
- **Voting Power:** The influence of a user's vote is determined by their voting power for that tag.
- **Decay Factor:** Voting power and the impact of votes decay over time to reflect recent activity.
- **Per-Tag Reputation Management:** Users maintain separate reputations and voting powers for each tag.

### **Phases of the Reputation System**

1. **Initial Phase:**
    - Active when a reputation tag has fewer active users than a specified minimum threshold.
    - All users can gain and exert voting power to bootstrap the community.
2. **Post-Threshold Phase:**
    - Activated once the community reaches the minimum number of active users.
    - Stricter rules apply to maintain system integrity.
    - Only trusted users can influence reputations.

---

## **Detailed Mechanics**

### **1. Initial Phase**

- **Active User Threshold:** Define a minimum number of active users (e.g., `M = 100`) for each reputation tag.
- **Voting Power Accumulation:**
    - **All users** can gain voting power by:
        - **Receiving votes** from others.
        - **Casting votes**, earning a small bonus per vote cast.
- **Voting Power Calculation:**
    
    ```
    TotalVotingPower = VotingPowerFromVotesReceived + (NumberOfVotesCast × BonusPerVote) x decay
    ```
    
    - **VotingPowerFromVotesReceived:**
        - Calculated from the sum of `VoteResultingVotingPower` from votes received.
    - **BonusPerVote:**
        - A fixed amount (e.g., `BonusPerVote = 0.05`) added for each vote cast.
    - decay:
        - A number between 0-1 (%), reducing the reputation gained based on time past.

### **2. Post-Threshold Phase**

- **Trusted Level Threshold:**
    - **Minimum Voting Power (T):** Users must have a voting power ≥ `T` (e.g., `1.0`) to be considered trusted.
- **Founder Limitations:**
    - Once the system reaches 100+ reputable users, founders no longer have special control over entry into the system.
    - System becomes fully community-governed through reputation mechanics.
- **Voting Power Restrictions:**
    - **Untrusted Users** (Voting Power < `T`):
        - **Voting Power:** Set to `0`.
        - **Votes Cast:** Have no effect on others' reputations.
    - **Trusted Users** (Voting Power ≥ `T`):
        - **Voting Power:** Calculated from votes received.
        - **Votes Cast:** Influence others' reputations.
- **Dynamic Vote Weight Updates:**
    - Previous votes are recalculated when voter's reputation changes:
    - Decreased reputation reduces impact of past votes upon target's next login.
    - Achieving trusted status applies vote weight retroactively upon target's next login.
- **Voting Power Calculation:**
    
    ```
    TotalVotingPower = VotingPowerFromVotesReceived
    ```
    
    - **No additional bonuses** are given for casting votes.
- **Voting Power Decay:**
    - Apply a decay rate (e.g., `1%` per day) to all users' voting power.

---

## **Reputation and Voting Power Calculations**

### **VoteResultingVotingPower**

For each vote received:

```
VoteResultingVotingPower = (PositiveOrNegative) × Decay × AuthorVotingPower
```

- **PositiveOrNegative:** `+1` for positive votes, `1` for negative votes.
- **Decay:** A factor between `0` and `1` that may reduce over time.
- **AuthorVotingPower:** Voting power of the user who cast the vote at the time of voting.

### **VotingPowerFromVotesReceived**

```
VotingPowerFromVotesReceived = Sum of all VoteResultingVotingPower
```

### **TotalVotingPower During Initial Phase**

```
TotalVotingPower = VotingPowerFromVotesReceived + (NumberOfVotesCast × BonusPerVote)
```

### **TotalVotingPower Post-Threshold**

```
TotalVotingPower = VotingPowerFromVotesReceived

```

---

## **Database Layout**

All data is stored using Juno collections. The primary collections are:

### **1. Votes Collection**

Stores individual vote records.

**Document Structure**:

```json
{
    "collection": "votes",
    "document": {
        "key": "vote_1234567890",  // Unique key for the vote
        "data": {
            "author": "user_id_voter",          // ID of the voter
            "target_user": "user_id_target",    // ID of the user being voted on
            "vote_type": "positive",            // 'positive' or 'negative'
            "reputation_tag": "#teamplay",      // Reputation tag
            "timestamp": "2024-09-29T12:34:56Z",// Vote timestamp
            "decay_value": 0.0                  // Initial decay value
        },
        "metadata": {
            "voting_power": "referenced_in_user_admin",  // Reference to voter's voting power
            "created_at": "2024-09-29T12:34:56Z",
            "updated_at": "2024-09-29T12:34:56Z"
        }
    }
}

```

### **2. User_Admin Collection**

Stores administrative data for each user per reputation tag.

**Document Structure**:

```json
{
    "collection": "user_admin",
    "document": {
        "key": "user_id_reputation_tag",  // Combination of user ID and reputation tag
        "data": {
            "user_id": "user_id_voter",          // ID of the user
            "reputation_tag": "#teamplay",       // Reputation tag
            "total_voting_power": 1.25,          // Calculated total voting power
            "last_decay_timestamp": "2024-09-29T12:34:56Z"  // Last time decay was applied
        },
        "metadata": {
            "created_at": "2024-09-29T12:34:56Z",
            "updated_at": "2024-09-29T12:34:56Z"
        }
    }
}

```

---

## **Simulation Example**

### **User Groups and Voting Activity**

| Votes Cast | Number of Users | Avg. Votes Received | TotalVotingPower per User (Initial Phase) |
| --- | --- | --- | --- |
| 1 vote | 10 | 1 | 0.15 |
| 2-5 votes | 20 | 2 | 0.40 |
| 6-10 votes | 30 | 3 | 0.70 |
| 11-20 votes | 20 | 5 | 1.25 |
| 21-50 votes | 10 | 10 | 2.75 |
| 51-100 votes | 5 | 15 | 5.25 |
| Over 100 votes | 5 | 20 | 8.00 |

### **Transition to Post-Threshold Phase**

- **Upon reaching 100 active users**, the system disables the bonus voting power.
- **Users with TotalVotingPower ≥ 1.0** become trusted users.
- **Users with TotalVotingPower < 1.0** have their voting power set to `0` until they reach the trusted threshold.

---

## **Use Cases and Applications**

### **1. Gaming Communities**

- **In-Game Factions:** Players create tags like `#democrats` and `#republicans` to form factions.
- **Dynamic Gameplay:** Players with certain reputations (e.g., `#PK` for player killers) can be marked in-game, affecting interactions like being attacked by city guards.
- **User-Driven Dynamics:** Game mechanics adapt based on reputations built through user votes.

### **2. Forums and Discussion Platforms**

- **Access Control:** Users earn reputation within a tag to gain privileges (e.g., creating topics, sharing links).
- **Community Moderation:** Users with higher reputations participate in moderation, enforcing community standards.
- **Decentralized Governance:** Forum rules and content curation are driven by collective user reputations.

### **3. Service Reviews and Trustworthiness**

- **Authentic Feedback:** Functions like Trustpilot, allowing genuine reviews of apps and services.
- **Business Reputation:** Companies and service providers are rated by users, fostering transparency.
- **Mitigating Fake Reviews:** Real data emerges from actual user experiences, free from manipulation.

### **4. Web3 Marketplaces**

- **Transactional Trust:** Buyers and sellers rate each other after transactions.
- **Platform Ratings:** Users rate the platform and services, enhancing overall trust.
- **Decentralized Verification:** Builds a trust network that is user-driven and resistant to fake feedback.

---

## **Encouraging Participation Post-Threshold**

- **Quality Contributions:** Users are encouraged to contribute positively to receive votes from trusted users.
- **Active Engagement:** Trusted users must remain active to maintain their voting power due to the decay mechanism.
- **Community Support:** Trusted users are incentivized to vote on others to support the growth and health of the community.

---

## **Safeguards Against Exploitation**

- **Minimum Voting Power Requirement:** Prevents new or anonymous users from influencing reputations until they are trusted.
- **Decay Mechanism:** Ensures that inactive users' voting power diminishes over time.
- **Per-Tag Reputation:** Users must build and maintain reputation separately for each tag, reducing the risk of widespread abuse.
- **Community Oversight:** If a group misuses a tag, the community can collectively downvote them, reducing their influence.

---

## **Action Items**

1. **Define Threshold Values:**
    - **Minimum Active Users (M)** for each reputation tag.
    - **Trusted Level Threshold (T)** for voting power.
    - **Decay Rate** and how it is applied (e.g., daily).
2. **Implement Decay Process:**
    - Decide on scheduling (e.g., nightly batch job).
    - Ensure it updates `decay_value` in vote records and adjusts `total_voting_power` accordingly.
3. **Update Database Schema if Necessary:**
    - Confirm that all required fields are present in the `votes` and `user_admin` collections.
    - Add indexes for efficient querying (e.g., on `user_id`, `reputation_tag`).
4. **Develop Voting Logic:**
    - Ensure the application enforces voting power restrictions based on trusted status.
    - Implement calculations for `VoteResultingVotingPower` and `TotalVotingPower`.
5. **Monitor and Adjust Parameters:**
    - Regularly review threshold values and decay rates.
    - Adjust `BonusPerVote` during the initial phase if necessary.
6. **API Development:**
    - Create a robust API that applications can use to integrate the reputation system.
    - Ensure the API supports necessary functionalities like casting votes, retrieving reputations, and managing tags.

---

## **Questions for Further Clarification**

- **Decay Mechanism:**
    - Are there specific decay formulas or schedules preferred for the decay of voting power and votes?
    - Should decay be linear, exponential, or follow another model?
- **Negative Votes Impact:**
    - Should negative votes contribute differently to the decay or reduction of voting power?
    - How should the system handle users who receive a large number of negative votes?
- **Inactive Users:**
    - Is there a plan for handling users who become inactive for extended periods?
    - Should their voting power decay to zero, or is there a minimum floor?
- **Additional Safeguards:**
    - Are there other measures needed to prevent coordinated attacks or misuse of tags?
    - How will the system verify the uniqueness of users to prevent bot activity?
- **Data Privacy and Security:**
    - How will user data be protected within the Juno collections?
    - Are there compliance requirements (e.g., GDPR) that need to be considered?

---

## **Conclusion**

This reputation system is designed to foster a flexible, decentralized, and self-regulating ecosystem that can be integrated across various platforms and applications. By leveraging tag-based reputations, voting power thresholds, and decay mechanisms, the system encourages active participation, quality contributions, and community governance while safeguarding against malicious activities.

---

Please review the document and let me know if there are any additional details you'd like to include or if you have answers to the questions posed. This will help ensure the documentation fully meets your requirements and is ready for implementation.