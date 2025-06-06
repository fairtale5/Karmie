<script lang="ts">
// Placeholder data for dashboard widgets
import QuickActionsDashboard from '$lib/components/dashboard/QuickActionsDashboard.svelte';
import { setPageMeta } from '$lib/stores/page';
import { onMount } from 'svelte';

const user = {
  handle: 'johndoe',
  displayName: 'John Doe',
  avatarUrl: 'https://i.pravatar.cc/100?img=3',
  topTags: [
    { name: 'Helpful', score: 120 },
    { name: 'Skillful', score: 98 },
    { name: 'Friendly', score: 87 }
  ]
};

const recentTags = [
  { name: 'React', created: '2h ago' },
  { name: 'Svelte', created: '5h ago' },
  { name: 'Rust', created: '1d ago' }
];

const popularTags = [
  { name: 'JavaScript', votes: 1200 },
  { name: 'Python', votes: 1100 },
  { name: 'TypeScript', votes: 900 }
];

const repChangeUsers = [
  { name: 'alice', change: +50, tags: ['Helpful', 'Skillful'] },
  { name: 'bob', change: -30, tags: ['Friendly'] },
  { name: 'carol', change: +25, tags: ['Helpful'] }
];

const tagsByVotes = [
  { name: 'Svelte', votes: 300 },
  { name: 'Rust', votes: 250 },
  { name: 'Go', votes: 200 }
];

const tagsByTrusted = [
  { name: 'Python', trusted: 12 },
  { name: 'JavaScript', trusted: 10 },
  { name: 'Solidity', trusted: 8 }
];

// New data for additional widgets
const tagTrends = [
  { name: 'AI', growth: '+45%', period: '7d' },
  { name: 'Web3', growth: '+32%', period: '7d' },
  { name: 'Rust', growth: '+28%', period: '7d' }
];

const userMilestones = [
  { type: 'Tag Creation', description: 'Created #Web3 tag', date: '2h ago' },
  { type: 'Reputation', description: 'Reached 1000 points in #JavaScript', date: '1d ago' },
  { type: 'Trust', description: 'Became trusted in #Rust', date: '3d ago' }
];

const systemAnnouncements = [
  { title: 'New Feature', content: 'Tag suggestions now available!', priority: 'high' },
  { title: 'Update', content: 'Improved reputation calculation algorithm', priority: 'medium' }
];

const quickActions = [
  { name: 'Create Tag', icon: '➕' },
  { name: 'Invite User', icon: '📨' },
  { name: 'View Reports', icon: '📊' }
];

// Set page title
onMount(() => {
  setPageMeta({ title: 'Dashboard' });
});
</script>

<!-- Main dashboard grid -->
<div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6 p-4">
  <!-- Column 1 -->
  <div class="space-y-6">
    <!-- User Profile Card -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 flex flex-col items-center">
      <img src={user.avatarUrl} alt="avatar" class="rounded-full w-24 h-24 mb-4 mt-2 border-4 border-primary-500" />
      <div class="text-2xl font-bold">{user.displayName}</div>
      <div class="opacity-60 mb-2">@{user.handle}</div>
      <div class="w-full mt-2">
        <div class="font-semibold text-center mb-4">Most Active In:</div>
        <div class="flex gap-2 justify-center flex-wrap">
          {#each user.topTags as tag}
            <span class="badge preset-tonal-primary">#{tag.name}: {tag.score}</span>
          {/each}
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <QuickActionsDashboard />

    <!-- Most Recent Tags -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="font-bold text-lg mb-2">Most Recent Tags</div>
      <ul class="space-y-1">
        {#each recentTags as tag}
          <li class="flex justify-between">
            <span class="text-primary-500">#{tag.name}</span>
            <span class="opacity-60 text-xs">{tag.created}</span>
          </li>
        {/each}
      </ul>
    </div>
  </div>

  <!-- Column 2 -->
  <div class="space-y-6">
    <!-- Reputation Change Users -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="flex justify-between items-center mb-2">
        <div class="font-bold text-lg">Users: Most Reputation Change</div>
        <div class="flex gap-2">
          <button class="btn preset-tonal-primary text-xs">24h</button>
          <button class="btn preset-tonal-primary text-xs">7d</button>
          <button class="btn preset-tonal-primary text-xs">30d</button>
          <button class="btn preset-tonal-primary text-xs">90d</button>
        </div>
      </div>
      <div class="flex gap-2 mb-2">
        <button class="btn preset-outlined-primary-500 text-xs">Gains</button>
        <button class="btn preset-outlined-primary-500 text-xs">Losses</button>
        <button class="btn preset-filled-primary-500 text-xs">Both</button>
      </div>
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>User</th>
              <th>Change</th>
              <th>Tags</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each repChangeUsers as u}
              <tr>
                <td class="font-mono">{u.name}</td>
                <td class={u.change > 0 ? 'text-success-500' : 'text-error-500'}>
                  {u.change > 0 ? '+' : ''}{u.change}
                </td>
                <td class="opacity-60 text-xs">
                  {#each u.tags as tag, i}
                    #{tag}{i < u.tags.length - 1 ? ', ' : ''}
                  {/each}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- Popular Tags -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="flex justify-between items-center mb-2">
        <div class="font-bold text-lg">Popular Tags</div>
        <div class="flex gap-2">
          <button class="btn preset-tonal-primary text-xs">24h</button>
          <button class="btn preset-tonal-primary text-xs">7d</button>
          <button class="btn preset-tonal-primary text-xs">30d</button>
          <button class="btn preset-tonal-primary text-xs">90d</button>
        </div>
      </div>
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>Tag</th>
              <th class="text-right">Votes</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each tagsByVotes as tag}
              <tr>
                <td class="text-primary-500">#{tag.name}</td>
                <td class="text-right">
                  <span class="badge preset-filled-primary-500">{tag.votes} votes</span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- Tag Trends -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="font-bold text-lg mb-2">Trending Tags</div>
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>Tag</th>
              <th>Growth</th>
              <th class="text-right">Period</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each tagTrends as tag}
              <tr>
                <td class="text-primary-500">#{tag.name}</td>
                <td class="text-success-500">{tag.growth}</td>
                <td class="text-right opacity-60 text-xs">{tag.period}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <!-- Column 3 -->
  <div class="space-y-6">
    <!-- Tags by Most Trusted Users -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="font-bold text-lg mb-2">Tags: Most Trusted Users</div>
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>Tag</th>
              <th class="text-right">Trusted Users</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each tagsByTrusted as tag}
              <tr>
                <td class="text-primary-500">#{tag.name}</td>
                <td class="text-right">
                  <span class="badge preset-filled-primary-500">{tag.trusted} trusted</span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- User Milestones -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="font-bold text-lg mb-2">Your Milestones</div>
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>Type</th>
              <th>Description</th>
              <th class="text-right">Date</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each userMilestones as milestone}
              <tr>
                <td class="font-semibold">{milestone.type}</td>
                <td class="text-sm opacity-80">{milestone.description}</td>
                <td class="text-right opacity-60 text-xs">{milestone.date}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- System Announcements -->
    <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
      <div class="font-bold text-lg mb-2">Announcements</div>
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>Title</th>
              <th>Content</th>
              <th class="text-right">Priority</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each systemAnnouncements as announcement}
              <tr>
                <td class="font-semibold">{announcement.title}</td>
                <td class="text-sm opacity-80">{announcement.content}</td>
                <td class="text-right">
                  <span class="badge preset-filled-{announcement.priority === 'high' ? 'error' : 'primary'}-500">
                    {announcement.priority}
                  </span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

<!--
DASHBOARD IDEAS:
- Activity feed: recent votes, tag creations, user milestones
- Tag creation trends: chart of tags created over time
- Reputation leaderboard: top users overall or by tag
- Tag suggestion prompts: tags that need more votes or trusted users
- User milestones: badges for new achievements
- System announcements or tips
- Quick actions: create tag, invite user, etc.
-->
