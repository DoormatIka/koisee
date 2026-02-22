
<script>
	import { events } from "$lib/components/tab-manager.svelte";
    import TabRow from "./TabRow.svelte";
	let tabs = $derived(events);
</script>

{#key tabs}
<div class="tabs tabs-lift w-full h-full min-h-0 flex-1">
	{#if tabs.length <= 0}
		<div class="flex w-full h-full justify-center items-center">
			<p>Press "Scan" to start.</p>
		</div>
	{/if}
	{#each tabs as [uuid, state], i}
		<input type="radio" name="my_tabs_3" class="tab" aria-label="Tab {i + 1}" />
		<div class="tab-content min-w-0 overflow-y-auto bg-base-100 border-base-300 p-3">
			{#if state.type === "progress"}
				<span>Scanning job {uuid}.. this may take a while.</span>
			{:else if state.type === "result"}
				<p>
					The score dictates how similar the two images are to each other. 
						0 - closest, 4-8 - minor differences.
				</p>
				<br>
				{#each state.matched_images as srcList}
					<TabRow srcList={srcList} />
				{/each}
			{:else if state.type === "error"}
				<span class="text-red-500">Error: {state.error}</span>
			{/if}
		</div>
	{/each}
</div>
{/key}
