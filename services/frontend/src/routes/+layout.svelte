<script lang="ts">
  // Layout imports
  import './layout.css';
  import favicon from '$lib/assets/favicon.svg';
  import Navbar from '$lib/components/Navbar.svelte';
  import { user } from '$lib/stores/user.svelte';

  let { children } = $props();
</script>

<svelte:head>
  <!-- Favicon -->
  <link rel="icon" href={favicon} />

  <!-- Expose user colors as CSS variables -->
  <style>
    :root {
      --user-accentColor: {$user.accentColor};
      --user-contrastColor: {$user.contrastColor};
    }
  </style>
</svelte:head>

<!-- Navbar -->
<Navbar />

<!-- Page container -->
<main class="flex items-center justify-center min-h-screen px-4 py-8">
  <div
    class="max-w-4xl w-full p-8 rounded-xl shadow-xl text-default transition-colors duration-500 bg-gray-100 dark:bg-gray-900"
  >
    {@render children()}
  </div>
</main>
