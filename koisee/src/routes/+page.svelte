<script lang="ts">
	import FilePicker from "$lib/components/FilePicker.svelte";
	import Spinner from "$lib/components/Spinner.svelte";
</script>

<main class="flex flex-col h-full w-full gap-4 p-3">

	<div class="flex flex-row gap-3 justify-between items-center shrink-0">
		<Spinner />
		<div class="flex flex-row flex-1 gap-3">
			<FilePicker />
		</div>
	</div>
	
</main>

