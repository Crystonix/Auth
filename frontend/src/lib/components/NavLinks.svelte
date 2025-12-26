<script lang="ts">
  import { Roles, user } from "$lib/stores";
</script>

<div class="flex space-x-4 items-center">
  {#if user}
    <!-- Links for all logged-in users -->
		 {#if user.role != null}
				<a href="/dashboard" class="hover:text-primary transition-colors duration-300">Dashboard</a>
		 {/if}

    {#if user.role === Roles.ADMIN}
      <!-- Admin-only links -->
      <a href="/admin" class="hover:text-primary transition-colors duration-300">Admin</a>
      <a href="/admin/settings" class="hover:text-primary transition-colors duration-300">Settings</a>
    {/if}
  {/if}
</div>
