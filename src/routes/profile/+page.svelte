<script lang="ts">
    import { onMount } from 'svelte';
    import { authSubscribe, type User } from '@junobuild/core';
    import { goto } from '$app/navigation';
    import { initJuno } from '$lib/juno';
    import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';

    let user: User | null = null;

    onMount(async () => {
        await initJuno();
        authSubscribe((state) => {
            user = state;
            // Redirect to home if not logged in
            if (user === null) {
                goto('/');
            }
        });
    });
</script>

<!-- Show warning if not logged in -->
<NotLoggedInAlert />

<div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">Profile</h1>

    {#if user}
        <div class="bg-[var(--card-bg)] border-[var(--card-border)] rounded-lg p-6 shadow-lg">
            <div class="space-y-4">
                <div>
                    <label for="principal-id" class="block text-sm font-medium text-[var(--text-secondary)]">Principal ID</label>
                    <p id="principal-id" class="mt-1 text-[var(--text-primary)] break-all">{user.key}</p>
                </div>
                <div>
                    <label for="created-at" class="block text-sm font-medium text-[var(--text-secondary)]">Account Created</label>
                    <p id="created-at" class="mt-1 text-[var(--text-primary)]">
                        {new Date(Number(user.created_at) / 1_000_000).toLocaleDateString()}
                    </p>
                </div>
            </div>
        </div>
    {:else}
        <div class="text-center">
            <p class="text-[var(--text-primary)]">Please sign in to view your profile.</p>
        </div>
    {/if}
</div> 