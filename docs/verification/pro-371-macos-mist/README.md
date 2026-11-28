# Mist su Apple Silicon

Il motore nativo ha scaricato e installato Mist 0.30, rilevato l’aggiornamento a 0.40, scaricato il secondo DMG e spostato l’app aggiornata nel Cestino del profilo temporaneo. Il file di prova del profilo è rimasto intatto. L’apertura della finestra e i download di firmware restano da verificare. [Registro del ciclo](mist-native-cycle.log).

- macOS 26.4 su Apple Silicon, Gatekeeper attivo. I due DMG corrispondono a dimensioni e SHA-256 pubblicati su GitHub. Il requisito riportato dal bundle e dal binario è macOS 12.0. Il ciclo è stato eseguito su arm64. [Release 0.30](https://github.com/ninxsoft/Mist/releases/tag/v0.30) · [Release 0.40](https://github.com/ninxsoft/Mist/releases/tag/v0.40).
- I bundle `com.ninxsoft.mist` firmati dal team `7K3HVCLV7Z` hanno superato la verifica ricorsiva rigorosa e la valutazione come Notarized Developer ID. [Controlli 0.30](mist-v0.30.log) · [Controlli 0.40](mist-v0.40.log) · [Licenza MIT](https://github.com/ninxsoft/Mist/blob/v0.40/LICENSE).
- Per ripetere il ciclo usa il [banco macOS](../pro-371-macos-overnight/README.md), salva le pagine degli asset come `mist-v0.30-assets.html` e `mist-v0.40-assets.html`, poi esegui `bash run-native.sh mist ninxsoft/Mist Mist v0.30 v0.40`. L’elenco delle versioni storiche è caricato dagli asset ufficiali. Le operazioni di installazione e aggiornamento scaricano i pacchetti effettivi.

Le release Syncthing per macOS 2.1.5-1 e 2.1.6-1 contengono binari universali, ma la verifica rigorosa segnala metadati Finder o resource fork nel file firmato `libswiftCore.dylib`. La scheda resta disponibile con la fonte ufficiale. [Log 2.1.5-1 e 2.1.6-1](syncthing-signature.log) · [Release di Syncthing per macOS](https://github.com/syncthing/syncthing-macos/releases).
