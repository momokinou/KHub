<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let new_path = "";

  async function greet() {
    name = await invoke("set_audio_path", { newPath: name });
    new_path = await invoke("get_all_audio", {
      path: "E:\\mkv\\Saiki Kusuo no Ψ Nan_S01E01_Épisode 1.mkv",
    });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{name}</p>
</div>
