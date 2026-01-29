<script lang="ts">
	type Props = {
		href: string;
		label: string;
	};

	const { href, label } = $props();
</script>

<a
	href={href}
	class="hover:text-primary transition-colors duration-300 decoration-0"
>
	{label}
</a>
