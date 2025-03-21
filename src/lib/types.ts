export interface TagData {
    name: string;
    description: string;
    time_periods: Array<{
        months: number;
        multiplier: number;
    }>;
    reputation_threshold: number;
    vote_reward: number;
    min_users_for_threshold: number;
} 