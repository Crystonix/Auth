<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from '$lib/components/Navbar.svelte';
	import { onMount } from 'svelte';
	import { user } from '$lib/stores';

	let { children } = $props();

	onMount(async () => {
    try {
      const res = await fetch('/api/auth/me', { credentials: 'include' });
      if (!res.ok) {
        user.reset();
        return;
      }
      const data = await res.json();
      user.setUser(data);
    } catch (_) {
      user.reset();
    }
  });
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<Navbar/>
{@render children()}
