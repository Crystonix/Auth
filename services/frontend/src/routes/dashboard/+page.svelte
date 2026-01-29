<script lang="ts">
	import { user } from "$lib/stores";
</script>

<div>
	<h3>Welcome {user.username}</h3>
	<p>
		<a href="/" class="text-primary hover:text-primary-hover transition-colors duration-300">
			Go to explanations
		</a>
	</p>
</div>

