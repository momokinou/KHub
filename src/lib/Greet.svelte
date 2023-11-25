<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let new_path = "";

  async function greet() {
    new_path = await invoke("set_subtitle_path", { newPath: name });
    new_path = await invoke("get_all_subtitle", {
      path: "E:\\mkv\\Saiki Kusuo no Ψ Nan_S01E01_Épisode 1.mkv",
    });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{new_path}</p>
</div>
