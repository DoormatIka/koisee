<script lang="ts">
	import FilePicker from "$lib/components/FilePicker.svelte";
	import Spinner from "$lib/components/Spinner.svelte";

	import { listen } from "@tauri-apps/api/event";
	import { writable, type Writable } from "svelte/store";

	type Info = string;
	type ImageDecoding = string;
	type Hash = string;
	type ImageFinished = string;
	type ImageTotal = number;
	type FileError = [string, string];
	type LogError = string;

	type States = {
		index: number,
		states: [ImageDecoding, Hash, ImageFinished],
	};
	type ProgressItems = Writable<Map<string, States>>;
	function moveToNextState(items: ProgressItems, state: string) {
		items.update(i => {
			const item = i.get(state);
			if (!item) {
				throw new Error("AAA");
			}

			i.set(state, { ...item, index: item.index + 1 });			
			return new Map(i);
		});
	}
	function createState(items: ProgressItems, state: string) {
		items.update(i => {
			i.set(state, { index: 0, states: ["1", "2", "3"] });
			return new Map(i);
		})
	}


	const progress_items: ProgressItems = writable(new Map());
	let images_processed = 0;
	let images_error = 0;
	let total = 0;

	listen<Info>("log:info", (s) => console.log(`[info]: ${JSON.stringify(s)}`));
	listen<ImageDecoding>("file:decoding", (s) => createState(progress_items, s.payload));
	listen<Hash>("file:hash", (s) => moveToNextState(progress_items, s.payload));
	listen<FileError>("file:error", (s) => progress_items.update(i => {
		console.log(`${JSON.stringify(s.payload)}`);
		const [file, err] = s.payload;
		i.delete(file);
		console.log(`[ERR]: ${file} - ${err}`);
		images_error++;
		return new Map(i);
	}));
	listen<ImageFinished>("file:finished", (s) => {
    progress_items.update(i => {
        i.delete(s.payload);
				images_processed++;
        return new Map(i);
    });
	});
	listen<ImageTotal>("file:total", (n) => total = n.payload);
</script>

<main class="flex flex-col h-full w-full gap-4 p-3">

	<div class="flex flex-row gap-3 justify-between items-center shrink-0">
		<Spinner />
		<div class="flex flex-row flex-1 gap-3">
			<FilePicker />
		</div>
		<div>
			<p>Images processed: (({images_processed} SUCCESS, {images_error} FAIL)/{total})</p>
			<p>Threads used: {$progress_items.size}</p>
		</div>
	</div>
	
	{#each [...$progress_items] as [key, value]}
		<p>{key}: {JSON.stringify(value)}</p>
	{/each}
</main>

