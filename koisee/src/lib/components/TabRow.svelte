
<script lang="ts">
	import { convertFileSrc, invoke } from "@tauri-apps/api/core";
	import type { MatchedBucket } from "./tab-manager.svelte";
	import Icon from "@iconify/svelte";

	type RowProps = { src: MatchedBucket, onRemove: (uuid: string) => {} }
	const { src, onRemove }: RowProps = $props();

	console.log(src)

	let selectedPaths = $state<string[]>([]);
	async function deleteFile() {
		for (const f of selectedPaths) {
			// await invoke("remove_file", { path: selectedPath });
		}
		if (selectedPaths.length >= 0) {
			onRemove(src.uuid)
		}
	}
</script>

<form class="flex flex-col flex-1 w-full min-w-0 p-3 bg-base-200 rounded">
	<div class="flex flex-row min-w-0 gap-2 overflow-x-auto">
		{#each src.paths as image, i}
			<label class="flex flex-col flex-none rounded cursor-pointer has-checked:bg-base-100 has-checked:ring-1 m-4 p-4">
				<input type="checkbox" name="selected-image" value={image.path} bind:group={selectedPaths} class="hidden">
				<img loading="lazy" class="h-auto max-h-[33vh] w-full object-cover" src={convertFileSrc(image.path)} alt="Image #{i}">
				<p class="py-1 break-all">{image.path} ({image.width}x{image.height}) ({image.similarity})</p>
			</label>
		{/each}
	</div>
	<div class="flex flex-row-reverse min-w-0 gap-2 h-14 items-center justify-between">
		<div>
			<button type="button" onclick={deleteFile} class="btn btn-circle btn-primary">
				<Icon icon="material-symbols:delete-rounded" width="24" height="24" />
			</button>
			<button type="button" class="btn btn-circle btn-secondary">
				<Icon icon="material-symbols:visibility-off-rounded" width="24" height="24" />
			</button>
		</div>
	</div>
</form>
