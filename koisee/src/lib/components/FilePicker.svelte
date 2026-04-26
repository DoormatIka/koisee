
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
			const res = await invoke("scan", {dir: selected});
			console.log(res);
		} catch (error: any) {
			err = error.toString()
		}

  }
</script>

<input type="text" class="input flex-1" placeholder="No directory selected..." value={selected_dir} readonly />
<button class="btn" onclick={scanDirectory}>Select</button>
