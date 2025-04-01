/**
 * Global settings for the Reputator application
 * This file contains all configurable values that can be adjusted
 * to tune the reputation system's behavior
 */

// System Mode Configuration
/**
 * Controls whether the system operates in playground or production mode
 * 
 * Playground Mode (true):
 * - Single user creates all documents
 * - Uses document key in description for ownership
 * - Relaxed validation for testing
 * 
 * Production Mode (false):
 * - Each user creates their own documents
 * - Uses Juno's Principal ID for ownership
 * - Strict validation rules
 */
export const IS_PLAYGROUND = true;  // Set to false for production

export interface TimePeriod {
    months: number;
    multiplier: number;
}

export interface TagSettings {
    REPUTATION_THRESHOLD: number;
    VOTE_REWARD: number;
    MIN_USERS_FOR_THRESHOLD: number;
}

export interface VoteSettings {
    DEFAULT_WEIGHT: number;
    MIN_WEIGHT: number;
    MAX_WEIGHT: number;
}

export interface UISettings {
    DECIMAL_PLACES: number;
    WHOLE_NUMBERS: boolean;
}

export const REPUTATION_SETTINGS = {
    // Default values for new tags
    DEFAULT_TAG: {
        REPUTATION_THRESHOLD: 10,        // Minimum reputation needed for voting power
        VOTE_REWARD: 0.1,               // Reputation points given for casting a vote
        MIN_USERS_FOR_THRESHOLD: 5,     // Minimum number of users that need to reach threshold
                                        // before vote rewards are restricted
    } as TagSettings,

    // Time period multipliers for vote decay
    DEFAULT_TIME_PERIODS: [
        { months: 1, multiplier: 1.5 },    // Period 1: First month
        { months: 2, multiplier: 1.2 },    // Period 2: Months 2-3
        { months: 3, multiplier: 1.1 },    // Period 3: Months 4-6
        { months: 6, multiplier: 1.0 },    // Period 4: Months 7-12
        { months: 12, multiplier: 0.95 },  // Period 5: Months 13-24
        { months: 12, multiplier: 0.75 },  // Period 6: Months 25-36
        { months: 12, multiplier: 0.50 },  // Period 7: Months 37-48
        { months: 999, multiplier: 0.25 }  // Period 8: Months 49+ (treated as infinity)
    ] as TimePeriod[],

    // Vote settings
    VOTE: {
        DEFAULT_WEIGHT: 1,              // Base weight for all votes
        MIN_WEIGHT: 0,                  // Minimum possible vote weight
        MAX_WEIGHT: 10,                  // Maximum possible vote weight
    } as VoteSettings,

    // UI settings
    UI: {
        DECIMAL_PLACES: 1,              // Number of decimal places to show for reputation scores
        WHOLE_NUMBERS: true,            // Whether to show reputation as whole numbers
    } as UISettings
}; 