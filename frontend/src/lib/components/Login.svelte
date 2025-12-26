<script lang="ts">
  let loading = false;

  function login() {
    loading = true;
    window.location.href = 'http://localhost/api/auth/discord/login';
  }
</script>

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
