<script lang="ts">
  import './layout.css';
  import favicon from '$lib/assets/favicon.svg';
  import Navbar from '$lib/components/Navbar.svelte';

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />

  <style>
    :root {
      --user-accentColor: {$user.accentColor};
      --user-contrastColor: {$user.contrastColor};
    }
  </style>
</svelte:head>

<!-- Navbar -->
<Navbar />

<main class="flex justify-center px-4 py-8 bg-sunken dark:bg-sunken transition-colors duration-500">
  <div
    class="max-w-4xl w-full p-8 rounded-xl shadow-lg bg-surface dark:bg-surface text-default dark:text-default transition-colors duration-500"
  >
    <div>
      {@render children()}
    </div>
  </div>
</main>
