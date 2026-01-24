<script lang="ts">
  import Login from '$lib/components/Login.svelte';
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
	import NavLinks from './NavLinks.svelte';
</script>

<nav class="w-full shadow-md transition-colors duration-500 bg-navbar text-default">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between h-16 items-center">

      <!-- Logo / Branding -->
      <div class="shrink-0">
        <a href="/" class="text-xl font-bold transition-colors duration-500 text-primary">
          MyApp
        </a>
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
