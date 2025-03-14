import { defineDevConfig, PermissionText, MemoryText } from '@junobuild/config';

export default defineDevConfig(() => ({
	satellite: {
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
					read: "public" as PermissionText,
					write: "managed" as PermissionText,
					memory: "stable" as MemoryText,
					mutablePermissions: true
				}
			],
			storage: []
		}
	}
}));
