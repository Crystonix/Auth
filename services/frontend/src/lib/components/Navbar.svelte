<script lang="ts">
  import Login from '$lib/components/Login.svelte';
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
	import NavLinks from './NavLinks.svelte';
	import { user } from '$lib/stores';
</script>

<nav class="w-full shadow-md transition-colors duration-500 text-default bg-navbar-light dark:bg-navbar-dark">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between h-16 items-center">

      <!-- Logo / Branding -->
      
        <div class="shrink-0">
          <a href="/" class="text-xl font-bold transition-colors duration-500 logo" >
            Discord Authenticator
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
<style>
  .logo{
    text-decoration: none;
    color: var(--user-accentColor);
  }

  nav {
    background-color: var(--color-navbar-light);
    position: sticky;
    top: 0;
  }

  :global(.dark) nav {
    background-color: var(--color-navbar-dark);
  }
  </style>
