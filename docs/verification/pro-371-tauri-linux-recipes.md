# Ricette Linux x86_64 nella prova Tauri

La prova desktop usa il motore nativo condiviso con 15 AppImage Linux x86_64 che hanno completato installazione, apertura, aggiornamento e rimozione in VPS1. La scheda richiede sia una ricetta con identità e repository fissati sia un asset Linux x86_64 della release. Le voci editoriali e gli asset per altre architetture non abilitano installazioni.

| Applicazione | Versione iniziale | Versione aggiornata |
| --- | --- | --- |
| LocalSend | 1.18.2 | 1.19.0 |
| AniCraft | 0.2.0 | 0.2.1 |
| SoundCraft | 0.20.0 | 0.20.1 |
| PhotoCraft | 0.23.0 | 0.23.1 |
| PDFCraft | 0.21.1 | 0.22.0 |
| VectorCraft | 0.24.0 | 0.24.1 |
| LightCraft | 0.21.0 | 0.21.1 |
| DeckCraft | 0.18.1 | 0.19.1 |
| GridCraft | 0.19.0 | 0.19.1 |
| FilmCraft | 0.20.0 | 0.20.1 |
| EffectCraft | 0.24.1 | 0.25.1 |
| WordCraft | 0.18.1 | 0.19.1 |
| CADCraft | 0.20.0 | 0.20.1 |
| TextureCraft | 0.6.0 | 0.6.1 |
| FigCraft | 0.18.0 | 0.18.1 |

Il processo Tauri Linux crea o riapre un profilo privato con HOME e XDG separati. Il backend vede i record interni al profilo e imposta la directory `Applications` privata, senza adottare copie dall'host. Il processo figlio imposta `APPIMAGE_EXTRACT_AND_RUN=1`, la modalità usata durante i cicli su Xvfb. [Documentazione AppImage](https://docs.appimage.org/user-guide/troubleshooting/fuse.html)

Le release ufficiali sono state scaricate e confrontate con i digest SHA-256 su VPS1. Ogni versione è stata aperta in un sandbox Bubblewrap senza rete, con rendering software e profilo temporaneo. Per LocalSend si è verificato anche l'avvio dal collegamento desktop senza FUSE. I test originali hanno verificato versione attiva e mantenimento dei dati. [Prova LocalSend](pro-371-linux-localsend.md)

Gli output originali dei tre lotti Storytold sono nello scratch VPS `/mnt/dati/lavori/shiftanything/shift-linux-release-audit.EoN6VTOI/proofs/`. Contengono URL, dimensione, digest, log di avanzamento, hash del profilo prima e dopo e PNG di entrambe le versioni. La matrice dimostra i cicli del motore condiviso e l'apertura delle singole app. La prova Linux della finestra Tauri e dei suoi clic richiede GTK e WebKitGTK 4.1, assenti sulle VPS controllate.

La verifica dell'adattatore `session.rs` su VPS1 ha eseguito Apri e Rimuovi, passando da richiesta e approvazione a `NativeBackend`. Una fixture eseguibile ha scritto un marker dopo aver letto HOME, XDG e l'opzione di apertura senza FUSE. Il test ha anche respinto un collegamento della cartella `Applications` e un record di installazione esterno al profilo. Il log è in `/mnt/dati/lavori/shiftanything/shift-tauri-linux-integration.WeQPpEhy/proof-fixture.log`. La prova ha usato il sorgente Rust condiviso con Tauri, senza una WebView Linux. Su macOS è passato il controllo del crate Tauri con `cargo check --tests`.

## Prove successive

Quattro ricette già presenti hanno completato il ciclo con finestre Xvfb: Craft Anything, DesignCraft, MailCraft e SolidCraft. A seguire Pencil2D e OpenShot sono state integrate dopo i cicli con apertura e collegamenti di menu. [Quattro cicli ArtCraft](pro-371-linux-appimage-four/README.md) · [Pencil2D e OpenShot](pro-371-linux-pencil-openshot/README.md).

La finestra Tauri Ubuntu e i clic dalla build estratta dal DEB sono stati verificati in prove separate. [Pacchetto e clic HandBrake](pro-371-linux-media-cad/README.md).
