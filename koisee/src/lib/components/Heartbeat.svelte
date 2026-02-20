
<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let isAlive = $state(false)
$effect(() => {
	let unlisten: () => void;

	(async () => {
		isAlive = await invoke("get_heartbeat");
		unlisten = await listen<boolean>("server-status", (event) => {
			isAlive = event.payload;
		});
	})();

	return () => {
		if (unlisten) unlisten();
	};
});
</script>

<p>server alive?: {isAlive ? "yea!" : "nay.."}</p>
