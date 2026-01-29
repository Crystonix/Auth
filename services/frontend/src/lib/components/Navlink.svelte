<script lang="ts">
	import { page } from "$app/state";
  type Props = {href: string; label: string; };
  const { href, label } = $props();
</script>

<a
  href={href}
  class="text-default dark:text-default hover:text-primary transition-colors duration-300 decoration-0"
  class:text-primary={page.url.pathname + page.url.hash === href}
	data-sveltekit-preload-data="hover"
>
  {label}
</a>
