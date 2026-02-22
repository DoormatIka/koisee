
<script lang="ts">
	import { events } from "$lib/components/tab-manager.svelte";
	import Icon from "@iconify/svelte";
	import TabRow from "./TabRow.svelte";

	async function onRemove(selectedPath: string) {
		// nightmare function, i know. 
		// the only reason why i used an array for now is because 
		// Set and Map weren't being reactively updated by svelte. very annoying.
		const noContents: number[] = [];

		for (let i = 0; i < events.length; i++) { 
			// replace events to use a Map<uuid, ScanIntermediate> in v1.1
			const el = events[i];
			const scanIntermediate = el[1];
			if (scanIntermediate.type !== "result") {
				return;
			}
			// replace matched_images to be buckets instead of pairs.
			scanIntermediate.matched_images = scanIntermediate.matched_images.filter(
					match => !match.paths.includes(selectedPath)
			);

			if (scanIntermediate.matched_images.length <= 0) {
				noContents.push(i);
			}
		}

		for (const index of noContents.reverse()) {
			events.splice(index, 1);
		}
	}

	let tabs = $derived(events);
</script>

<div class="tabs tabs-lift w-full h-full min-h-0 flex-1">
	{#if tabs.length <= 0}
		<div class="flex w-full h-full justify-center items-center">
			<p>Press "Scan" to start.</p>
		</div>
	{/if}
	{#each tabs as [uuid, state], i (uuid)}
		<label class="tab flex flex-row gap-2 px-2 pt-2">
			<input type="radio" name="my_tabs_3" />
			Tab #{i}
			<button class="btn btn-circle w-6 h-6">
				<Icon icon="material-symbols:close-rounded" class="w-6 h-6" />
			</button>
		</label>
		<div class="tab-content min-w-0 overflow-y-auto bg-base-100 border-base-300 p-3">
			{#if state.type === "progress"}
				<span>Scanning job {uuid}.. this may take a while.</span>
			{:else if state.type === "result"}
				<p>
					The score dictates how similar the two images are to each other. 
						0 - closest, 4-8 - minor differences.
				</p>
				<p>
					Total duplicate pairs: {state.matched_images.length}
				</p>
				<br>
				{#each state.matched_images as srcList (srcList.uuid)}
					<TabRow src={srcList} onRemove={onRemove} />
				{/each}
			{:else if state.type === "error"}
				<span class="text-red-500">Error: {state.error}</span>
			{/if}
		</div>
	{/each}
</div>
