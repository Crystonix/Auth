<script lang="ts">
  import Login from '$lib/components/Login.svelte';
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
	import NavLinks from './NavLinks.svelte';
  import homeicon from '$lib/assets/homeicon.svg'
</script>

<nav class="w-full shadow-md transition-colors duration-500 bg-navbar text-default">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between h-16 items-center">

      <!-- Logo / Branding -->
      <div class="shrink-0">
        <button class="homeButton">
          <svg width="24", height="24">{homeicon}</svg>
        </button>
      </div>
        
      <div class="appLogo">
        <h1>Discord Authenticator</h1>
      </div>

      <div>
        <NavLinks/>
      </div>

      <!-- Right side: Login & Dark Mode -->
      <div class="flex items-center space-x-4">
        <Login />
        <DarkModeToggle />
      </div>

    </div>
  </div>
</nav>
