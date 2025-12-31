<script lang="ts">
	// src/routes/+layout.svelte
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from '$lib/components/Navbar.svelte';

	let { children } = $props();

	import { onMount } from 'svelte';
  import { user } from '$lib/stores/user.svelte';

  onMount(async () => {
    try {
      const res = await fetch('/api/auth/me', { credentials: 'include' });
      if (res.ok) {
        const data = await res.json();
        user.setUser(data);
      } else {
        user.reset();
      }
    } catch (err) {
      console.error('Failed to fetch user:', err);
      user.reset();
    }
  });
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<Navbar/>
{@render children()}
