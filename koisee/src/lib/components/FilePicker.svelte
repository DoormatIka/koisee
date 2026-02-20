
<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog"
	import { invoke } from "@tauri-apps/api/core";

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
			err = await invoke("queue_scan", {dir: selected});
		} catch (error: any) {
			err = error.toString()
		}

  }
</script>

<form class="flex flex-row gap-2">
	<input type="text" class="input flex-1" placeholder="No directory selected..." value={selected_dir} readonly />
	<button class="btn" onclick={scanDirectory}>Scan</button>
</form>
<p class="px-5 text-center">{JSON.stringify(err)}</p>
