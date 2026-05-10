
<script lang="ts">
	import Icon from "@iconify/svelte";

	import { open } from "@tauri-apps/plugin-dialog"
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";


	let selected_dir = $state("")
  let err = $state("");

	let cancelling = $state(false);
	let starting = $state(false);

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
			starting = true;
			const res = await invoke("scan", {dir: selected});
			starting = false;

			console.log(res); // pass to other components if possible.
		} catch (error: any) {
			err = error.toString()
		}
  }

	async function cancel() {
		await invoke("cancel");
	}


	listen("process:cancelling", () => cancelling = true);
	listen("process:finished", () => {
		cancelling = false;
		selected_dir = "";
	});
</script>

<input type="text" class="input flex-1" placeholder="No directory selected..." value={selected_dir} readonly />
<button disabled={cancelling || starting} class="btn btn-circle" onclick={scanDirectory}>
	<Icon icon="material-symbols:search" width={20} height={20} />
</button>
<button disabled={cancelling || !starting} class="btn btn-circle" onclick={cancel}>
	<Icon icon="material-symbols:cancel-outline" width={20} height={20} />
</button>
