
# Module Extraction

## Variables Globales

### Petite explication

```Rust
static mut N: i32 = 5;
```

Comme il s'agit d'un objet mutable, un thread pourrait mettre à jour N pendant qu'un autre le lit, ce qui entraînerait une insécurité au niveau de la mémoire. Ainsi, l'accès et la mutation d'un mut statique ne sont pas sûrs, et doivent donc être effectués dans un bloc non sûr :

```Rust
unsafe {
    N += 1;

    println!("N: {}", N);
}
```

### subtitle_path: String

Variable stockant le path vers où sont enregistrés les fichiers de sous-titres.

### audio_path: String

Variable stockant le path vers où sont enregistrés les fichiers audio.

## Fonctions

### set_subtitle_path (tauri::AppHandle, String): String

Utilisée pour sauvegarder le path d'enregistrement des fichiers.
Utilisée à chaque démarrage de l'applciation.

### get_subtitle_path (tauri::AppHandle): String

Retourne le path actuellement définit.

### set_audio_path (tauri::AppHandle, String): String

Utilisée pour sauvegarder le path d'enregistrement des fichiers.
Utilisée à chaque démarrage de l'applciation.

### get_audio_path (tauri::AppHandle): Path

Retourne le path actuellement définit.

### get_all_subtitles (file_path: Vec<Path>, lang: Vec<String>)

Créé les fichiers de sous-titres dans un path prédéfinis via les paramètres globaux du module.

Récupère, en paramètre, une liste de fichiers Matroska (.mkv) ainsi qu'une liste de langages respectant la notation ISO.
Ne retourne rien.

### get_all_audios (file_path: Vec<Path>)

Créé les fichiers audios dans un path prédéfinis via les paramètres globaux du module.

Récupère, en paramètre, une liste de fichiers Matroska (.mkv) ainsi qu'une liste de langages respectant la notation ISO.
Ne retourne rien.

### is_mkv (file_path: Path)

Check si le fichier est bien un mkv.
Est effectué au début de chaque fonction impliquant un mkv.
