<script lang="ts">
  let loading = false;

  function login() {
    loading = true;
    // Redirect to auth-service login endpoint
    window.location.href = 'http://localhost/api/auth/discord/login';

  }
</script>

<button on:click={login} disabled={loading}>
  {#if loading}
    Redirecting…
  {:else}
    Login with Discord
  {/if}
</button>
