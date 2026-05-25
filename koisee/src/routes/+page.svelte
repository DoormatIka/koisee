<script lang="ts">
	import FilePicker from "$lib/components/FilePicker.svelte";
	import Spinner from "$lib/components/Spinner.svelte";
	import Progress from "$lib/components/ProgressCounter.svelte";
	import GridProgress from "$lib/components/GridProgress.svelte";
	import Scans from "$lib/components/Scans.svelte";

	import { listen } from "@tauri-apps/api/event";
	import { 
		clearProgress, createState, moveToNextState, 
		progress_items, fileError, finishFile, assignTotal,
		
		type Info, type ImageDecoding, type Hash,
		type ImageFinished, type ImageTotal, type FileError
	} from "$lib/stores/progress";


	listen("process:start", () => clearProgress());
	listen<Info>("log:info", (s) => console.log(`[info]: ${JSON.stringify(s)}`));
	listen<ImageDecoding>("file:decoding", (s) => createState(progress_items, s.payload));
	listen<Hash>("file:hash", (s) => moveToNextState(progress_items, s.payload));
	listen<FileError>("file:error", (s) => fileError(s));
	listen<ImageFinished>("file:finished", (s) => finishFile(s));
	listen<ImageTotal>("file:total", (n) => assignTotal(n.payload));
	listen("process:finished", () => clearProgress());
</script>

<main class="flex flex-col h-full w-full gap-4 p-3">
	<div class="flex flex-row gap-3 justify-between items-center shrink-0">
		<Spinner />
		<div class="flex flex-row flex-1 gap-3">
			<FilePicker />
		</div>
		<Progress />
	</div>
	
	<GridProgress />
	<Scans />
</main>

