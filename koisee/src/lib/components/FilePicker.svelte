
<script lang="ts">
	import Icon from "@iconify/svelte";

	import { open } from "@tauri-apps/plugin-dialog"
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { scanResults, scanError, scanLoading } from "$lib/stores/scan";
	import type { ImageData, RawImageData } from "$lib/stores/scan";


	let selected_dir = $state("")

	let cancelling = $state(false);

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
			scanResults.set([]);
			scanLoading.set(true);
			scanError.set(null);
			// res: Result<Vec<Vec<ImageData>>, String>
			const res = await invoke<RawImageData[][]>("scan", {dir: selected});
			const imageData = convertIntoImageData(res);

			scanLoading.set(false);
			scanResults.set(imageData);
		} catch (error: any) {
			scanError.set(error);
		}
  }

	function convertIntoImageData(raw: RawImageData[][]): ImageData[][] {
		const data: ImageData[][] = [];
		for (const rawData of raw) {
			const b: ImageData[] = [];
			for (const rawImg of rawData) {
				b.push({ ...rawImg, selected: false })
			}
			data.push(b);
		}
		return data;
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
<button disabled={cancelling || $scanLoading} class="btn btn-circle" onclick={scanDirectory}>
	<Icon icon="material-symbols:search" width={20} height={20} />
</button>
<button disabled={cancelling || !$scanLoading} class="btn btn-circle" onclick={cancel}>
	<Icon icon="material-symbols:cancel-outline" width={20} height={20} />
</button>
