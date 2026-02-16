<script lang="ts">
	import Icon from "@iconify/svelte"
	import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="flex flex-col justify-center items-center h-full gap-4">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="flex flex-row gap-3">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="w-14 vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="w-14 tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="w-14 svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
	<span class="text-rotate text-2xl duration-6000">
		<span class="flex flex-col justify-center items-center">
			<span class="flex flex-row items-center">
				Tauri 
				<Icon class="mx-2" icon="material-symbols:webhook" width="24" height="24" />
			</span>
			<span>Vite</span>
			<span>SvelteKit</span>
		</span>
	</span>

  <form class="flex flex-row gap-4" onsubmit={greet}>
    <input class="input" id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button class="btn" type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</main>

