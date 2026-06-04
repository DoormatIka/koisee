
<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { progress_items, overlayColor } from "$lib/stores/progress";
</script>

<div class="w-full grid grid-cols-4 lg:grid-cols-6 gap-2">
	{#each [...$progress_items] as [imgPath, value], i}
		<img class="object-cover h-60 w-60 border-4 {overlayColor(value.index)}" src={convertFileSrc(imgPath)} alt="Image #{i + 1}"> 
	{/each}
</div>
