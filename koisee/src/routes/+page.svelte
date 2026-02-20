<script lang="ts">
	import FilePicker from "$lib/components/FilePicker.svelte";
	import Heartbeat from "$lib/components/Heartbeat.svelte";
	import Icon from "@iconify/svelte"

	import { events } from "$lib/components/tab-manager.svelte";

	let tabs = $derived(events);
</script>

<main class="flex flex-col justify-center items-center h-full gap-4">
	<span class="text-rotate text-2xl duration-6000">
		<span class="flex flex-col justify-center items-center">
			<span class="flex flex-row items-center">
				Tauri 
				<Icon class="mx-2" icon="material-symbols:webhook" width="24" height="24" />
			</span>
			<span class="flex flex-row items-center">
				Vite
				<Icon class="mx-2" icon="material-symbols:webhook" width="24" height="24" />
			</span>
			<span class="flex flex-row items-center">
				SvelteKit
				<Icon class="mx-2" icon="material-symbols:webhook" width="24" height="24" />
			</span>
		</span>
	</span>

	<FilePicker />
	<Heartbeat />

	<div class="flex flex-col overflow-auto">
		{#each tabs as [uuid, state]}
			{#if state.type === "progress"}
				<span>loading {uuid}..</span>
			{:else if state.type === "result"}
				<span>{JSON.stringify(state.matched_images)}</span>
			{:else if state.type === "error"}
				<span class="text-red-500">Error: {state.error}</span>
			{/if}
		{/each}
		<p></p>
	</div>

</main>

