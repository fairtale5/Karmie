<script lang="ts">
  import { Modal, Accordion } from '@skeletonlabs/skeleton-svelte';
  import { ExternalLink, Users, Zap, MessageSquare, Twitter, Heart, Target, AlertTriangle } from 'lucide-svelte';
  import { REPUTATION_SETTINGS, STRESS_TEST } from '$lib/settings';
  import { goto } from '$app/navigation';

  // Modal state
  let openState = $state(false);

  // Show modal on first load
  $effect(() => {
    if (typeof window !== 'undefined') {
      const hasBeenShown = sessionStorage.getItem(STRESS_TEST.STORAGE_KEY);
      if (!hasBeenShown) {
        openState = true;
      }
    }
  });

  function modalClose() {
    openState = false;
    if (typeof window !== 'undefined') {
      sessionStorage.setItem(STRESS_TEST.STORAGE_KEY, 'true');
    }
  }

  function goToICPTag() {
    modalClose();
    goto('/tag/ICP');
  }

  // Get threshold values from settings
  const { DEFAULT_TAG } = REPUTATION_SETTINGS;
  const reputationThreshold = DEFAULT_TAG.REPUTATION_THRESHOLD;
  const minUsersThreshold = DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD;
  const voteReward = DEFAULT_TAG.VOTE_REWARD;
</script>

<Modal
  open={openState}
  onOpenChange={(e) => (openState = e.open)}
  contentBase="card bg-surface-100-900 p-6 space-y-6 shadow-xl max-w-4xl max-h-[90vh] overflow-y-auto"
  backdropClasses="backdrop-blur-sm"
>
  {#snippet content()}
    <header class="flex justify-between items-start">
      <div class="space-y-2">
        <h2 class="h2 flex items-center gap-2">
          <Zap class="text-warning-500" size={32} />
          Welcome to the <span class="text-primary-500">Karmie</span> Stress Test!
        </h2>
        <p class="text-lg opacity-80">Help test if this reputation app can help reward real users who contribute, and protect the community from bad actors.</p>
      </div>
      <button type="button" class="btn-icon variant-ghost" onclick={modalClose}>
        ✕
      </button>
    </header>

    <div class="space-y-6">
      <!-- Introduction -->
      <article class="card variant-soft-primary p-4">
        <div class="flex items-start gap-3">
          <Target class="text-primary-500 flex-shrink-0" size={24} />
          <div class="space-y-2">
            <h3 class="h4">Our Testing Goals</h3>
            <ul class="list-disc list-inside space-y-1 opacity-90">
              <li><strong>Performance Testing:</strong> Monitor how many cycles the app consumes and identify optimization needs</li>
              <li><strong>Security Testing:</strong> Test for exploits and bad actors (note: anyone can vote until we reach {minUsersThreshold} trusted users with {reputationThreshold}+ reputation)</li>
              <li><strong>User Feedback:</strong> Gather feedback on how the system feels and works in practice</li>
            </ul>
          </div>
        </div>
      </article>

      <!-- Solo Project Notice -->
      <article class="card variant-soft-secondary p-4">
        <div class="flex items-start gap-3">
          <Heart class="text-secondary-500 flex-shrink-0" size={24} />
          <div class="space-y-3">
            <h3 class="h4">Solo Indie Project</h3>
            <p class="opacity-90">
              This is a solo indie project that needs community support to gain funding and succeed. But not everyone on ICP believes in fair & transparent community launches, so it is up to <span class="font-bold underline">us</span> to help spread this message. We need to reach at least 100 users using this, testing it, and sharing their experience, to prove that there is undeniable demand for it, so please share your experience on twitter and help ICP reach that goal.
            </p>
            <a 
              href={STRESS_TEST.TWEET_URL} 
              target="_blank" 
              rel="noopener noreferrer"
              class="btn preset-filled-secondary"
            >
              <Twitter size={16} />
              Comment & Retweet
              <ExternalLink size={16} />
            </a>
            <p class="text-xs opacity-70">
              I will personally answer and retweet every constructive comment, positive or negative.
            </p>
          </div>
        </div>
      </article>

      <!-- Testing Steps -->
      <div class="space-y-3">
        <h3 class="h3 flex items-center gap-2">
          <MessageSquare size={24} />
          How to Help Test
        </h3>
        
        <div class="space-y-2">
        <Accordion>
          <Accordion.Item value="step1">
            {#snippet control()}
              <div class="flex items-center gap-3 p-4 bg-gradient-to-r from-primary-50 to-primary-100 dark:from-primary-900/20 dark:to-primary-800/20 hover:from-primary-100 hover:to-primary-150 dark:hover:from-primary-800/30 dark:hover:to-primary-700/30 transition-colors rounded-lg cursor-pointer">
                <span class="chip preset-filled-primary text-sm">1</span>
                <span class="font-medium">Visit the #ICP Reputation Page</span>
              </div>
            {/snippet}
            {#snippet panel()}
              <div class="space-y-3 p-4 bg-surface-50 dark:bg-surface-800/50 rounded-lg border border-surface-200 dark:border-surface-700">
                <p class="opacity-90">
                  This is a reputation I prepared for users to play with, but you can also <a href="/new/tag" class="anchor">create your own</a>.
                </p>
              </div>
            {/snippet}
          </Accordion.Item>

          <Accordion.Item value="step2">
            {#snippet control()}
              <div class="flex items-center gap-3 p-4 bg-gradient-to-r from-secondary-50 to-secondary-100 dark:from-secondary-900/20 dark:to-secondary-800/20 hover:from-secondary-100 hover:to-secondary-150 dark:hover:from-secondary-800/30 dark:hover:to-secondary-700/30 transition-colors rounded-lg cursor-pointer">
                <span class="chip preset-filled-secondary text-sm">2</span>
                <span class="font-medium">Vote on Users (Positive & Negative)</span>
              </div>
            {/snippet}
            {#snippet panel()}
              <div class="space-y-3 p-4 bg-surface-50 dark:bg-surface-800/50 rounded-lg border border-surface-200 dark:border-surface-700">
                <p class="opacity-90">
                  On the ICP tag page, vote positive and negative on different users to test the system:
                </p>
                <ul class="list-disc list-inside space-y-1 opacity-80">
                  <li>Use the Quick Actions panel to search for users</li>
                  <li>Vote based on your perception of their contributions</li>
                  <li>Try voting on multiple users to see how it affects reputation</li>
                  <li>Each vote earns you {voteReward} reputation points (during bootstrap phase)</li>
                </ul>
                <div class="card variant-soft-warning p-3">
                  <div class="flex items-start gap-2">
                    <AlertTriangle class="text-warning-500 flex-shrink-0" size={16} />
                    <p class="text-sm">
                      <strong>Attention:</strong> Until the reputation reaches 10 reputable users, every vote will be rewarded, and every vote will count. This is normal. Once it reaches the minimum users, it will upgrade automatically and close itself off to bad actors.
                    </p>
                  </div>
                </div>
              </div>
            {/snippet}
          </Accordion.Item>

          <Accordion.Item value="step3">
            {#snippet control()}
              <div class="flex items-center gap-3 p-4 bg-gradient-to-r from-tertiary-50 to-tertiary-100 dark:from-tertiary-900/20 dark:to-tertiary-800/20 hover:from-tertiary-100 hover:to-tertiary-150 dark:hover:from-tertiary-800/30 dark:hover:to-tertiary-700/30 transition-colors rounded-lg cursor-pointer">
                <span class="chip preset-filled-tertiary text-sm">3</span>
                <span class="font-medium">Monitor Performance & Try to Exploit</span>
              </div>
            {/snippet}
            {#snippet panel()}
              <div class="space-y-3 p-4 bg-surface-50 dark:bg-surface-800/50 rounded-lg border border-surface-200 dark:border-surface-700">
                <p class="opacity-90">
                  Help us find issues and optimize the system:
                </p>
                <ul class="list-disc list-inside space-y-1 opacity-80">
                  <li>Watch for slow loading times or UI glitches</li>
                  <li>Try rapid voting to test rate limits</li>
                  <li>Look for ways to game the reputation system</li>
                  <li>Test edge cases like voting on the same user multiple times</li>
                  <li>Check if reputation calculations seem accurate</li>
                </ul>
                <p class="text-sm opacity-70">
                  Any bugs or exploits you find help make the system more robust!
                </p>
              </div>
            {/snippet}
          </Accordion.Item>

          <Accordion.Item value="step4">
            {#snippet control()}
              <div class="flex items-center gap-3 p-4 bg-gradient-to-r from-surface-100 to-surface-150 dark:from-surface-700/50 dark:to-surface-600/50 hover:from-surface-150 hover:to-surface-200 dark:hover:from-surface-600/60 dark:hover:to-surface-500/60 transition-colors rounded-lg cursor-pointer">
                <span class="chip preset-filled-surface text-sm">4</span>
                <span class="font-medium">Share Feedback</span>
              </div>
            {/snippet}
            {#snippet panel()}
              <div class="space-y-3 p-4 bg-surface-50 dark:bg-surface-800/50 rounded-lg border border-surface-200 dark:border-surface-700">
                <p class="opacity-90">
                  Your feedback shapes the future of this project:
                </p>
                <ul class="list-disc list-inside space-y-1 opacity-80">
                  <li>Comment on the Twitter thread with your experience</li>
                  <li>Suggest improvements or features</li>
                  <li>Share what type of project you'd like to see this used for</li>
                  <li>Report any bugs or confusing UI elements</li>
                </ul>
                <div class="card variant-soft-surface p-3">
                  <p class="text-sm opacity-80">
                    <strong>Need inspiration?</strong> Think about how this could work for:
                    games, blog platforms, ecommerce, professional networks, 
                    community governance, or social media platforms.
                  </p>
                </div>
                <a 
                  href={STRESS_TEST.TWEET_URL} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  class="btn preset-filled-surface"
                >
                  <Twitter size={16} />
                  Share Your Feedback
                  <ExternalLink size={16} />
                </a>
              </div>
            {/snippet}
          </Accordion.Item>
        </Accordion>
        </div>
      </div>
    </div>

    <footer class="flex flex-col sm:flex-row gap-4 justify-between items-center pt-4 border-t border-surface-300-700">
      <p class="text-sm opacity-70">
        This modal will not appear again until you refresh the page
      </p>
      <div class="flex gap-3">
        <button type="button" class="btn preset-tonal" onclick={modalClose}>
          I'll explore on my own
        </button>
        <button type="button" class="btn preset-filled-primary" onclick={goToICPTag}>
          Take me to #ICP tag
        </button>
      </div>
    </footer>
  {/snippet}
</Modal> 