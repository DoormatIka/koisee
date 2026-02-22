
<script lang="ts">
	import { convertFileSrc } from "@tauri-apps/api/core";
	import type { MatchedImage } from "./tab-manager.svelte";
	import Icon from "@iconify/svelte";

	type RowProps = { srcList: MatchedImage }
	const { srcList }: RowProps = $props();
</script>

<div class="flex flex-col flex-1 w-full min-w-0 p-3 bg-base-200 rounded">
	<div class="flex flex-row min-w-0 gap-2">
		{#each srcList.paths as path, i}
			<div class="flex-1 flex flex-col min-w-0 w-0 rounded border-primary">
				<img loading="lazy" class="h-auto max-h-[33vh] w-full object-cover" src={convertFileSrc(path)} alt="Image #{i}">
				<p class="px-2 py-1 break-all">{path}</p>
			</div>
		{/each}
	</div>
	<div class="flex flex-row min-w-0 gap-2 h-14 items-center justify-between">
		<p class="text-center text-xl">Score: {srcList.similarity}</p>
		<div>
			<button class="btn btn-circle btn-primary">
				<Icon icon="material-symbols:delete-rounded" width="24" height="24" />
			</button>
			<button class="btn btn-circle btn-secondary">
				<Icon icon="material-symbols:visibility-off-rounded" width="24" height="24" />
			</button>
		</div>
	</div>
</div>
