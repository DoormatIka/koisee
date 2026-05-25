<script lang="ts">
	import Icon from "@iconify/svelte";
	import {
		total, images_processed, images_error, progress_items
	} from "$lib/stores/progress";
</script>

<div class="flex gap-2">
	<span class="flex gap-1">
		{images_processed}/{total}
		<Icon width={25} height={25} inline icon="material-symbols:check-circle" />
	</span>
	<span class="flex gap-1">
		{images_error}
		<Icon width={25} height={25} inline icon="material-symbols:error" />
	</span>
	<span class="flex gap-1">
		{$progress_items.size}
		<Icon width={25} height={25} inline icon="material-symbols:search-gear" />
	</span>
</div>
