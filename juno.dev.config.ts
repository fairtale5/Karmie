import { defineDevConfig, PermissionText, MemoryText } from '@junobuild/config';

/**
 * Juno Development Configuration
 * 
 * IMPORTANT SATELLITE IDs:
 * - Local Development: "jx5yt-yyaaa-aaaal-abzbq-cai"
 *   Used when running `juno dev start` for local testing
 *   The emulator will use this ID automatically
 * 
 * - Production: "YOUR_PRODUCTION_SATELLITE_ID"
 *   Get this from your Juno Console after deployment
 *   Replace when deploying to production
 * 
 * To switch between environments:
 * 1. Local Development:
 *    - Run `juno dev start` to start local emulator
 *    - Frontend will automatically connect to local instance
 * 
 * 2. Production:
 *    - Deploy using `juno deploy`
 *    - Update satelliteId in your frontend initialization
 * 
 * Collection Permissions:
 * - read: "public" | "private" | "managed" | "controllers"
 * - write: "public" | "private" | "managed" | "controllers"
 * - memory: "stable" | "heap"
 * - mutablePermissions: true = can change permissions later
 */

export default defineDevConfig(() => ({
	satellite: {
		// Local Development ID (default for emulator)
		satelliteId: "jx5yt-yyaaa-aaaal-abzbq-cai",
		
		// Production ID (uncomment and replace when deploying)
		// satelliteId: "YOUR_PRODUCTION_SATELLITE_ID",
		
		collections: {
			datastore: [
				{
					collection: "users",
					read: "public" as PermissionText,
					write: "private" as PermissionText,
					memory: "stable" as MemoryText,
					mutablePermissions: true
				},
				{
					collection: "tags",
					read: "public" as PermissionText,
					write: "managed" as PermissionText,
					memory: "stable" as MemoryText,
					mutablePermissions: true
				},
				{
					collection: "votes",
					read: "public" as PermissionText,
					write: "private" as PermissionText,
					memory: "stable" as MemoryText,
					mutablePermissions: true
				},
				{
					collection: "reputations",
					read: "controllers" as PermissionText,
					write: "controllers" as PermissionText,
					memory: "stable" as MemoryText,
					mutablePermissions: true
				}
			],
			storage: []  // No storage collections needed yet
		}
	}
}));
