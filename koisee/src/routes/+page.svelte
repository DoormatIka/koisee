<script lang="ts">
	import { listen } from "@tauri-apps/api/event";

	import FilePicker from "$lib/components/FilePicker.svelte";
	import Heartbeat from "$lib/components/Heartbeat.svelte";
	import Icon from "@iconify/svelte"

	let out = $state("");
	listen("scan-finished", (m) => {
		out = JSON.stringify(m.payload);
	})
	listen("scan-error", (m) => {
		out = JSON.stringify(m.payload);
	})
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

	<p>{out}</p>

</main>

