<script lang="ts">
    import ThemeToggle from './ThemeToggle.svelte';
</script>

<header class="bg-bg-secondary border-b border-border-color">
    <div class="container mx-auto px-4 py-4 flex justify-between items-center">
        <div class="flex items-center space-x-8">
            <a href="/" class="text-2xl font-bold text-text-primary">Reputator</a>
            <nav class="hidden md:flex space-x-6">
                <a href="/" class="text-text-secondary hover:text-text-primary transition-colors">Home</a>
                <a href="/admin" class="text-text-secondary hover:text-text-primary transition-colors">Admin</a>
                <a href="/profile" class="text-text-secondary hover:text-text-primary transition-colors">Profile</a>
            </nav>
        </div>
        <div class="flex items-center">
            <ThemeToggle />
        </div>
    </div>
</header> 