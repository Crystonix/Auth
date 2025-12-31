<script lang="ts">
  import { user } from '$lib/stores/user.svelte';
  let loading = $state(false);
  let showDropdown = $state(false);

  function login() {
    loading = true;
    window.location.href = '/api/auth/discord/login';
  }

  function logout() {
    window.location.href = '/api/auth/logout';
  }

  function toggleDropdown() {
    showDropdown = !showDropdown;
  }
</script>

{#if user.authenticated}
  <!-- Dropdown button when logged in -->
  <div class="relative inline-block text-left">
    <button
      onclick={toggleDropdown}
      class="flex items-center px-4 py-2 rounded-md font-semibold transition transform hover:scale-[1.02] active:scale-100 bg-primary text-default hover:bg-primary-hover"
    >
      {#if user.avatar}
        <img src="{user.avatar}" alt="Avatar" class="w-6 h-6 rounded-full mr-2" />
      {/if}
      {user.username}
      <svg class="ml-2 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    {#if showDropdown}
      <div
        class="absolute right-0 mt-2 w-40 origin-top-right bg-white border border-gray-200 rounded-md shadow-lg z-50"
      >
        <button
          onclick={logout}
          class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
        >
          Logout
        </button>
      </div>
    {/if}
  </div>
{:else}
  <!-- Login button when not logged in -->
  <button
    onclick={login}
    class="px-4 py-2 rounded-md font-semibold transition transform hover:scale-[1.02] active:scale-100 disabled:opacity-50 disabled:cursor-not-allowed bg-primary text-default hover:bg-primary-hover"
    disabled={loading}
  >
    {#if loading}
      Redirecting…
    {:else}
      Login with Discord
    {/if}
  </button>
{/if}
