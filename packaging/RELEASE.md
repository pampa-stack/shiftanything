# Pubblicazione della beta

1. Scegli un commit, con la stessa versione in Cargo e Tauri. Usa quel commit per il DMG notarizzato, il DEB e le note di rilascio.
2. Verifica i pacchetti su Mac Apple Silicon e Ubuntu x64. Esegui [il packaging Mac](macos/README.md) e `package-tauri-updater.sh` per firmare anche gli aggiornamenti e generare `latest.json`.
3. Scrivi le novità in Markdown e genera le note con `python3 packaging/release-notes.py "$commit" --changes novita.md > release-notes.md`. Lo script ricalcola schede e verifiche per piattaforma dal commit usato per compilare e rifiuta verifiche per schede assenti.
4. Carica il DMG, gli aggiornamenti firmati, il manifest e i checksum in una bozza GitHub con `--target "$commit"` e `--notes-file release-notes.md`. Controlla che i file caricati corrispondano ai checksum prima di pubblicare come release più recente.
5. Verifica il download anonimo dalla landing e l’aggiornamento dalla beta precedente su entrambi i sistemi. La landing legge versione, peso e download da GitHub. Il README rimanda alle note per i conteggi.
