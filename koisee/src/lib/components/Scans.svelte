
<script lang="ts">
	import { scanResults } from "$lib/stores/scan";
    import ScansItem from "./ScansItem.svelte";
</script>

<div class="w-full h-full">
	{#each $scanResults ?? [] as data, i}
		<!-- work on the delete button that grabs selected from scanResults and deletes them -->
		<form class="flex flex-row gap-5 justify-center py-4 overflow-x-auto">
			<ScansItem data={data} />
		</form>
	{/each}
</div>
