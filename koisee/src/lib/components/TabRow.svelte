
<script lang="ts">
	import { convertFileSrc, invoke } from "@tauri-apps/api/core";
	import type { MatchedImage } from "./tab-manager.svelte";
	import Icon from "@iconify/svelte";

	type RowProps = { src: MatchedImage, onRemove: (selectedPath: string) => {} }
	const { src, onRemove }: RowProps = $props();

	let selectedPath = $state<string | undefined>(undefined);
	async function deleteFile() {
		if (selectedPath !== undefined) {
			// await invoke("remove_file", { path: selectedPath });
			onRemove(selectedPath)
		}
	}
</script>

<form class="flex flex-col flex-1 w-full min-w-0 p-3 bg-base-200 rounded">
	<div class="flex flex-row min-w-0 gap-2">
		{#each src.paths as path, i}
			<label class="flex-1 flex flex-col min-w-0 w-0 rounded cursor-pointer has-checked:bg-base-100 has-checked:ring-1 p-4">
				<input type="radio" name="selected-image" value={path} bind:group={selectedPath} class="hidden">
				<img loading="lazy" class="h-auto max-h-[33vh] w-full object-cover" src={convertFileSrc(path)} alt="Image #{i}">
				<p class="py-1 break-all">{path}</p>
			</label>
		{/each}
	</div>
	<div class="flex flex-row min-w-0 gap-2 h-14 items-center justify-between">
		<p class="text-center text-xl">Score: {src.similarity}</p>
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
