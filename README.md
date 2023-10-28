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

___

## Architecture

### Module Core

    - Définition des modèles/types des données.
    - Base de données interne pour facilité la récupération des informations liées à une oeuvre?
        - Dépendant des autres modules.

### Module Extraction

    - Extraction des pistes de sous-titres et des pistes audios.
    - Extraction de toutes les données.
    - Ajouter l'option de le faire automatiquement lors d'un téléchargement d'un torrent?

### Module Insertion

    - Insertion de pistes de sous-titres et des pistes audios à un fichier .mkv.
    - Mettre des erreurs/alertes si la durée de la piste importée est plus longue/courte que la vidéo.

### Module Lecteur vidéo

    - Lecteur de fichiers .mkv.
        - Pouvoir choisir sa piste de sous-titres / En importer une.
        - Pouvoir choisir sa piste audio / En importer une.
    - Lecteur de fichier .mp4.
    - Pouvoir changer la vitesse de lecture.
    - Pouvoir changer la résolution.
    - Pouvoir changer le volume.
    - Play/Pause
    - Se souvenir où est-ce qu'un épisode a été arrêté.

### Module Lecture

    - Chercher la liste des extensions de fichier les plus utilisées pour les mangas/webtoons, etc...
    - Lecteur de ces dits fichiers.
    - Se souvenir de l'endroit où c'est arrêté le lecteur.
    - Lecture automatique.
        - Vitesse de défilement réglable.
        - Vitesse de défilement intelligente suivant le nombre de mots dans la page.
    - Pouvoir choisir sa langue de lecture.
    - Pouvoir choisir une langue que l'on souhaite travailler/apprendre.

### Module Edition

    - Module dépendant des modules d'extraction et d'insertion.
    - Edition des fichiers de sous-titres.
    - Vise à automatiser la récupération des textes sur les webtoons et à la remplacer par du blanc/couleur du fond plus facilement.

### Module Torrent

    - Téléchargement de torrents.
    - Seed de torrents.

### Module Librairie // A réfléchir

    - Consulter/Afficher toutes les oeuvres présentes sur le PC/les dossiers sélectionnés.
    - Récupérer automatiquement des informations sur les oeuvres.
    - Classer automatiquement les oeuvres en fonction du type de fichiers dans les dossiers.
        - Possibilité d'avoir mangas/webtoons/etc... et animés sous la même oeuvre.

## Authors

- [momokinou](https://github.com/momokinou)
