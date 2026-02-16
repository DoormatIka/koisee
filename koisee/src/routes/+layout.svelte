
<script lang="ts">
	let { children } = $props();
	import "../app.css";
</script>

<div class="h-dvh w-dvw">
	{@render children()}
</div>
