<script lang="ts">
  import { onMount } from 'svelte';

  let dark = false;

  onMount(() => {
    const stored = localStorage.getItem('theme');
    if (stored) {
      dark = stored === 'dark';
    } else {
      dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    updateHtmlClass();
  });

  function toggleDark() {
    dark = !dark;
    localStorage.setItem('theme', dark ? 'dark' : 'light');
    updateHtmlClass();
  }

  function updateHtmlClass() {
    if (dark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }
</script>

<button
  onclick={toggleDark}
  class="w-10 h-10 rounded-full flex items-center justify-center transition hover:scale-110 bg-surface text-default dark:bg-surface dark:text-default"
>
  {#if dark}
    <span class="text-accent-light text-xl">🌙</span>
  {:else}
    <span class="text-accent-light text-xl">☀️</span>
  {/if}
</button>
