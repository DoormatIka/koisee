
<script lang="ts">
	import Icon from "@iconify/svelte";

	import { scanResults } from "$lib/stores/scan";
	import ScansItem from "./ScansItem.svelte";
    import { invoke } from "@tauri-apps/api/core";

	function deleteAllSelectedItems(index: number) {
		scanResults.update(imageGrid => {
			if (!imageGrid) {
				return imageGrid;
			}
			const imageRow = imageGrid[index];
			for (const img of imageRow) {
				if (img.selected) {
					invoke("remove_file", { path: img.path }).catch(console.log);
					console.log(`Deleting ${img.path}`)
				}
			}
			imageGrid.splice(index, 1);
			return [...imageGrid];
		});
	}
</script>
  
{#if !$scanResults || $scanResults.length > 0}
	
<div class="w-full h-full">
	{#each $scanResults ?? [] as data, i}
		<!-- work on the delete button that grabs selected from scanResults and deletes them -->
		<form class="flex flex-col gap-5 py-4">
			<div class="flex flex-row gap-5 py-4 justify-center overflow-x-auto">
				<ScansItem data={data} />
			</div>
			<div class="flex justify-center">
				<button class="btn py-3" onclick={() => deleteAllSelectedItems(i)}>
					<Icon width={30} height={30} inline icon="material-symbols:delete" />
					Delete Selected
				</button>
			</div>
		</form>
	{/each}
</div>

{/if}

