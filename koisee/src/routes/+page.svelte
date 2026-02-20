<script lang="ts">
	import FilePicker from "$lib/components/FilePicker.svelte";
	import Heartbeat from "$lib/components/Heartbeat.svelte";
    import Spinner from "$lib/components/Spinner.svelte";
	import Tabs from "$lib/components/Tabs.svelte";
</script>

<main class="flex flex-col h-full w-full gap-4 p-3">
	<div class="flex flex-row gap-3 justify-between items-center shrink-0">
		<Spinner />
		<div class="flex flex-row flex-1 gap-3">
			<FilePicker />
		</div>
		<Heartbeat />
	</div>
	
	<Tabs />
</main>

