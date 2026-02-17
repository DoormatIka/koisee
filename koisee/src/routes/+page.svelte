<script lang="ts">
	import Icon from "@iconify/svelte"
	import { open } from "@tauri-apps/plugin-dialog"
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	let selected_dir = $state("")
  let err = $state("");

  async function scanDirectory() {
		const selected = await open({
			directory: true,
			multiple: false,
		});
		if (selected === null) {
			return;
		}
		selected_dir = selected;

		try {
			err = await invoke("get_similar_images", {dir: selected});
		} catch (error: any) {
			err = error.toString()
		}

  }

	let isAlive = $state(false)
	$effect(() => {
		let unlisten: () => void;

		(async () => {
			isAlive = await invoke("get_heartbeat");
			console.log(isAlive)
			unlisten = await listen<boolean>("server-status", (event) => {
				isAlive = event.payload;
			});
		})();

		return () => {
			if (unlisten) unlisten();
		};
	});
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

  <form class="flex flex-row gap-2">
    <input type="text" class="input flex-1" placeholder="No directory selected..." value={selected_dir} readonly />
    <button class="btn" onclick={scanDirectory}>Scan</button>
  </form>
	<p class="px-5 text-center">{JSON.stringify(err)}</p>

	<p>server alive?: {isAlive ? "yea!" : "nay.."}</p>
</main>

