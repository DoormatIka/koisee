
<script lang="ts">
	import { progress_items, progressColor, progressText } from "$lib/stores/progress";
</script>

<div class="w-full h-full flex flex-col overflow-y-auto gap-2">
	{#each [...$progress_items] as [imgPath, value]}
		<div class="w-full p-2 border {progressColor(value.index)}">
			<p>{progressText(value.index)}: {imgPath}</p>
		</div>
	{/each}
</div>
