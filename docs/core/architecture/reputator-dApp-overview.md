# Reputator: App for tracking user reputation and bot prevention

## Introduction

This document outlines the design and implementation of a user reputation system intended to serve as a decentralized platform for communities and applications. The system enables users to rate each other using positive or negative votes associated with specific reputation tags (e.g., `#teamplay`, `#crypto`). The aim is to create a versatile and self-regulating ecosystem where reputation is earned and maintained through active participation and recognition by others.

## Goals of the System

1. **Flexible Tag-Based Reputation:**
    - Allow any community to create and use custom `#tags` as they see fit.
    - Enable global tags (e.g., `#crypto`) and app-specific tags (e.g., `#appname`) to coexist.
    - Facilitate the formation of in-game factions, discussion forums, or any group identity through tags.

2. **Decentralized Trust Management:**
    - Communities can trust or distrust certain users based on collective upvotes.
    - Trust is earned through consistent positive contributions.
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

## System Overview

### Voting Mechanics

- **Vote Types:** Users can cast positive (`+1`) or negative (`-1`) votes on other users.
- **Reputation Tags:** Each vote is associated with a specific reputation tag (e.g., `#teamplay`, `#crypto`).
- **Voting Power:** The influence of a user's vote is determined by their voting power for that tag.
- **Time-Based Weighting:** Votes are weighted based on their age using conservative multipliers.
- **Per-Tag Reputation Management:** Users maintain separate reputations and voting powers for each tag.

### Conservative Multiplier Approach

The system uses a time-based decay system to weight votes based on their age. This approach ensures that:
- Recent votes have more influence
- Votes gradually lose weight over time
- Long-term sustainability is maintained
- Historical context is preserved

The time periods are structured as follows:
```typescript
const TIME_PERIODS = [
    { months: 1, multiplier: 1.5 },    // Period 1: First month
    { months: 2, multiplier: 1.2 },    // Period 2: Months 2-3
    { months: 3, multiplier: 1.1 },    // Period 3: Months 4-6
    { months: 6, multiplier: 1.0 },    // Period 4: Months 7-12
    { months: 12, multiplier: 0.95 },  // Period 5: Months 13-24
    { months: 12, multiplier: 0.75 },  // Period 6: Months 25-36
    { months: 12, multiplier: 0.55 },  // Period 7: Months 37-48
    { months: 999, multiplier: 0.25 }  // Period 8: Months 49+ (treated as infinity)
];
```

Key aspects of this approach:
1. First year is split into 4 periods (1+2+3+6 months = 12 months)
2. Following years are split into 12-month periods
3. Multipliers decrease gradually from 1.5x to 0.25x
4. Last period uses 999 months to represent infinity
5. All multipliers use 0.05 step increments for precision

For detailed implementation and calculations, see [Test Calculations](/docs/core/development/test-calculations.md).

## Use Cases and Applications

### 1. Gaming Communities

- **In-Game Factions:** Players create tags like `#democrats` and `#republicans` to form factions.
- **Dynamic Gameplay:** Players with certain reputations (e.g., `#PK` for player killers) can be marked in-game, affecting interactions like being attacked by city guards.
- **User-Driven Dynamics:** Game mechanics adapt based on reputations built through user votes.

### 2. Forums and Discussion Platforms

- **Access Control:** Users earn reputation within a tag to gain privileges (e.g., creating topics, sharing links).
- **Community Moderation:** Users with higher reputations participate in moderation, enforcing community standards.
- **Decentralized Governance:** Forum rules and content curation are driven by collective user reputations.

### 3. Service Reviews and Trustworthiness

- **Authentic Feedback:** Functions like Trustpilot, allowing genuine reviews of apps and services.
- **Business Reputation:** Companies and service providers are rated by users, fostering transparency.
- **Mitigating Fake Reviews:** Real data emerges from actual user experiences, free from manipulation.

### 4. Web3 Marketplaces

- **Transactional Trust:** Buyers and sellers rate each other after transactions.
- **Platform Ratings:** Users rate the platform and services, enhancing overall trust.
- **Decentralized Verification:** Builds a trust network that is user-driven and resistant to fake feedback.

## Safeguards Against Exploitation

1. **Vote Weight Distribution**
   - Total voting power automatically equals voter's reputation
   - Time-based multipliers determine vote influence
   - System remains stable and predictable

2. **Time-Based Weighting**
   - Recent votes have more influence
   - Prevents vote manipulation
   - Encourages active participation

3. **Per-Tag Reputation**
   - Users must build reputation separately for each tag
   - Reduces risk of widespread abuse
   - Enables specialized trust networks

4. **Community Oversight**
   - Users can downvote bad actors
   - Reduces influence of malicious users
   - Promotes self-regulation

## Action Items

1. **Implementation Tasks**
   - Implement vote weight calculation
   - Add reputation caching
   - Create UI components
   - Set up monitoring

2. **Testing Requirements**
   - Unit tests for calculations
   - Integration tests for database
   - Performance testing
   - Security testing

3. **Documentation Updates**
   - API documentation
   - Integration guides
   - Example implementations
   - Best practices

4. **Monitoring Setup**
   - Track reputation changes
   - Monitor system performance
   - Detect potential abuse
   - Gather usage statistics

## Questions for Further Clarification

1. **System Parameters**
   - Should multipliers be adjustable per tag?
   - What is the minimum reputation threshold?
   - How often should reputations be recalculated?

2. **Security Considerations**
   - How to prevent vote manipulation?
   - What are the rate limits for voting?
   - How to handle account recovery?

3. **Performance Optimization**
   - How to handle large vote sets?
   - What caching strategies to use?
   - How to optimize queries?

4. **User Experience**
   - How to display reputation changes?
   - What feedback to show users?
   - How to handle edge cases?

## Conclusion

This reputation system is designed to foster a flexible, decentralized, and self-regulating ecosystem that can be integrated across various platforms and applications. By leveraging tag-based reputations, conservative multipliers, and community oversight, the system encourages active participation, quality contributions, and community governance while safeguarding against malicious activities.

Candid: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/?id=rigfr-siaaa-aaaal-ab4fa-cai