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
        { months: 1, multiplier: 5 },       // Period 1: First month (5x bonus for very recent)
        { months: 2, multiplier: 2.5 },     // Period 2: Months 2-3 (2.5x bonus for recent)
        { months: 3, multiplier: 1.8 },     // Period 2: Months 4-6 (2.5x bonus for recent)
        { months: 6, multiplier: 1.4 },     // Period 3: Months 6-12 (1.4x bonus for semi-recent)
        { months: 12, multiplier: 1.0 },    // Period 4: Months 12-24 (baseline - normal weight)
        { months: 12, multiplier: 0.8 },    // Period 5: Months 24-36 (0.8x for older content)
        { months: 12, multiplier: 0.6 },    // Period 6: Months 37-48 (0.6x for mature content)
        { months: 12, multiplier: 0.4 },    // Period 7: Months 48-60 (0.4x for historical)
        { months: 999, multiplier: 0.2 }    // Period 8: Months 60+ Over 5 years (0.2x for very old content)
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

/**
 * URL to redirect to after successful login
 */
export const LOGIN_REDIRECT_URL = '/tags-hub';

/**
 * URL to redirect to after logout
 */
export const LOGOUT_REDIRECT_URL = '/';

/**
 * Frontend validation toggles for each document type
 * Set to true to enable validation, false to disable
 */
export const VALIDATE_USER_DOC = true;
export const VALIDATE_TAG_DOC = true;
export const VALIDATE_VOTE_DOC = true; 