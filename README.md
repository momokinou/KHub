# KHub

## Besoins non fonctionnelles

    - Performance:
    - Sécurité:
    - Plateformes visées: PC, mobiles si possible.
    - Hébergement: côté client/torrents.
    - Technologies: Tauri. Rust/Framework JS à définir.
    - IHM: Prévoir 16/9 au début puis adapter. 1080p.
    - Tests: Tests automatique de chaque fonction.
    - Code:
        - Pas/Peu de charge front: Fournir une classe/un objet contenant les données pour la vue.
        - Obligation de charger le front du module "Streaming" pour la création d'un vrai lecteur.

## Contexte

    - Rapidité: Le plus rapide possible. Eviter les MAJ inutiles car volume de données très important.
    - Volume de données: 100Mo => Plusieurs To.
    - Dépendance réseau: Dépend de la localisation des données. Dépendance forte si utilisation de torrents.
    - Quelle langue: Initiale français (vive la France) puis langue disposant de traductions pour les vidéos/livres.
    - Public: Grand public => Application légèrement customisable mais fermée pour faciliter son utilisation/prise en main.

## Use cases

    - Récupération via torrent
    - Analyse des fichiers du PC: Check taille des fichiers pour vérifier si doublons potentiels.
    - vidéos:
        - Visionnage
        - Multiple langues audio ET sous titres
        - Edition pour faciliter les traductions/fantrad
        - Extraction d'audio
        - Extraction de sous titres
    - Livres:
        - Visionnage
        - Multiple langues
        - Edition pour faciliter les traductions/fantrad

## Contraintes/Sujets à réflexion

    - Dépendance forte à FFMPEG au moment de l'extraction des sous-titres et pistes audios.
    - Séparation Back/Front? Je ne vois pas de possibilité.
    - Découpage du back en modules avec leur source propre.

## Authors

- [momokinou](https://github.com/momokinou)
