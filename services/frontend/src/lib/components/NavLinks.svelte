<script lang="ts">
	import { Roles } from "$lib/auth/user";
  import { user } from "$lib/stores";
	import Navlink from "./Navlink.svelte";

  type NavLink = {
		href: string;
		label: string;
	};

  const links: NavLink[] = [
		{ href: "/#ezchart", label: "Simple Chart" },
		{ href: "/#diagram", label: "Diagram" },
		{ href: "/#oauth", label: "OAuth2 Flow" }
	];
</script>

<div class="flex space-x-4 items-center">
	{#each links as link}
		<Navlink href={link.href} label={link.label} />
	{/each}

	{#if user.authenticated}
		{#if user.role != null}
			<Navlink href="/dashboard" label="Dashboard" />
		{/if}

		{#if user.role === Roles.ADMIN}
			<Navlink href="/admin" label="Admin" />
			<Navlink href="/admin/settings" label="Settings" />
		{/if}
	{/if}
</div>
