import type { UserDocument, UserData } from '$lib/types';

/**
 * Centralized avatar utility for consistent avatar handling across the app
 */
export interface AvatarConfig {
  src: string | undefined;
  name: string;
  initials: string;
}

/**
 * Gets avatar configuration for a user document
 * Handles fallback logic: avatar_url -> display_name initials -> user_handle initials
 */
export function getUserAvatar(user: UserDocument | UserData | { data: UserData }): AvatarConfig {
  // Handle different input formats
  const userData = 'data' in user ? user.data : user;
  
  // Get avatar URL (only if it exists and is not empty)
  const avatarUrl = userData.avatar_url && userData.avatar_url.trim() !== '' 
    ? userData.avatar_url 
    : undefined;
  
  // Get name for the Avatar component (required by SkeletonUI)
  const name = userData.display_name || userData.user_handle || 'User';
  
  // Generate initials with fallback logic
  let initials: string;
  
  if (userData.display_name && userData.display_name.trim() !== '') {
    // Use display name initials (e.g., "John Doe" -> "JD")
    initials = userData.display_name
      .trim()
      .split(' ')
      .map(word => word.charAt(0).toUpperCase())
      .slice(0, 2) // Max 2 initials
      .join('');
  } else if (userData.user_handle) {
    // Use user handle initials (e.g., "peterparker" -> "PP")
    initials = userData.user_handle
      .slice(0, 2)
      .toUpperCase();
  } else {
    // Ultimate fallback
    initials = 'U';
  }
  
  return {
    src: avatarUrl,
    name,
    initials
  };
}

/**
 * Convenience function for components that need just the initials
 */
export function getUserInitials(user: UserDocument | UserData | { data: UserData }): string {
  return getUserAvatar(user).initials;
} 