
<script lang="ts">
	import { scanResults } from '$lib/stores/scan';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import type { ImageData } from "$lib/stores/scan";

	let { data }: { data: ImageData[] } = $props();
	$effect(() => { $scanResults; });
	function toggle(data: ImageData) {
		scanResults.update(groups =>
			groups!.map(group =>
				group.map(img =>
					img.path === data.path ? { ...img, selected: !img.selected } : img
				)
			)
		);
	}
</script>

{#each data as d, i}
	{@const src = convertFileSrc(d.path.replaceAll('\\', '/').replace(/^[A-Z]:/, ''))}
	{@debug src}
	<div class="flex flex-col justify-center items-center">
    <div
      class="cursor-pointer rounded-md border-4 transition-colors {d.selected ? 'border-red-500' : 'border-transparent'}"
      onclick={() => toggle(d)}
      role="checkbox"
      aria-checked={d.selected}
      tabindex="0"
      onkeydown={(e) => e.key === ' ' && toggle(d)}
    >
	<img class="object-cover h-60 w-60 block" {src} alt="Image #{i + 1}" />
	</div>
		<p class="text-center max-w-60 wrap-break-word">{d.path}</p>
		<p>dimensions: {d.dimensions.join("x")}</p>
	</div>
{/each}
