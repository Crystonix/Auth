<script lang="ts">
	// src/routes/+layout.svelte
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from '$lib/components/Navbar.svelte';
	import { user } from '$lib/stores/user.svelte';

	let { children } = $props();
</script>

<svelte:head>
<style>
	// Expose user colors as CSS variables
	:root {
		--user-accentColor: {$user.accentColor};
		--user-contrastColor: {$user.contrastColor};
	}
</style>	
<link rel="icon" href={favicon} /></svelte:head>
		<Navbar/>
<div class="flex items-center justify-center min-h-screen">
	<div class="max-w-2xl w-full mx-4 p-8 rounded-lg shadow-lg text-default transition-colors duration-500" style="background-color: hsl(200 0% 95.7%); --dark-bg: hsl(260, 30%, 25%);">
		{@render children()}
	</div>
</div>		

<style>
	:global(.dark) div[style] {
		background-color: var(--dark-bg) !important;
	}
</style>