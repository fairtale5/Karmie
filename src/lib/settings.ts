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
export const IS_PLAYGROUND = false;  // Set to false for production

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
        { months: 1, multiplier: 1.0 },     // Period 1: First month (baseline for very recent)
        { months: 2, multiplier: 0.6 },     // Period 2: Months 2-3 (60% for recent)
        { months: 3, multiplier: 0.4 },     // Period 3: Months 4-6 (40% for semi-recent)
        { months: 6, multiplier: 0.3 },     // Period 4: Months 6-12 (30% for older)
        { months: 12, multiplier: 0.25 },   // Period 5: Months 12-24 (25% for mature)
        { months: 12, multiplier: 0.2 },    // Period 6: Months 24-36 (20% for historical)
        { months: 12, multiplier: 0.15 },   // Period 7: Months 37-48 (15% for very old)
        { months: 12, multiplier: 0.1 },    // Period 8: Months 48-60 (10% for ancient)
        { months: 999, multiplier: 0.05 }   // Period 9: Months 60+ (5% for archival)
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
        WHOLE_NUMBERS: false,           // Whether to show reputation as whole numbers
    } as UISettings
};

/**
 * URL to redirect to after successful login
 */
// export const LOGIN_REDIRECT_URL = '/dashboard'; // Original redirect for normal operation
export const LOGIN_REDIRECT_URL = '/tag/ICP'; // Temporary change for stress testing - redirect users directly to ICP tag page

/**
 * URL to redirect to after logout
 */
export const LOGOUT_REDIRECT_URL = '/';

/**
 * GitHub repository URL
 * Used in Footer, Sidebar navigation, and other external links
 */
export const GITHUB_URL = 'https://github.com/fairtale5/Reputator';

/**
 * Frontend validation toggles for each document type
 * Set to true to enable validation, false to disable
 */
export const VALIDATE_USER_DOC = true;
export const VALIDATE_TAG_DOC = true;
export const VALIDATE_VOTE_DOC = true; 