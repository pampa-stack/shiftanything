# Cicli macOS Apple Silicon isolati · PRO-371

Il probe usa selettore e funzioni d’installazione/rimozione del motore Rust, download HTTPS ufficiale, verifica SHA-256 e profili, database e Cestino separati. Sostituzione eseguita sulla stessa cartella di test in ogni ciclo. Verificati versione, receipt, assenza del percorso installato dopo la rimozione, destinazione del Cestino privato e sopravvivenza del file sintetico. App GUI e comandi del launcher UI restano da provare da un account isolato. Nessuna apertura né richiesta TCC sul profilo personale.
## KeyCastr

- Installato 0.11.0 in 1,873 s. Aggiornato a 0.11.1 in 1,399 s. Rimosso in 0,017 s. [Log](keycastr-cycle.log)
- App universale, ARM64 macOS 11+, `io.github.keycastr`, Developer ID Andrew Kitchen, Team `68TUCLNAPS`. Firma strict, notarizzazione e Gatekeeper PASS per entrambe, firma verificata anche nel Cestino privato. [Preflight vecchio](keycastr-v0.11.0-preflight.log) · [Preflight nuovo](keycastr-v0.11.1-preflight.log) · [Identità rimossa](keycastr-removed-identity.log)
- 0.11.0, 2.628.593 byte, SHA256 `00cc656a786012bcbdb5b203b1a56604727ad2ced5e4f5c1d189296f62881033`. [Release ufficiale](https://github.com/keycastr/keycastr/releases/tag/v0.11.0)
- 0.11.1, 2.714.588 byte, SHA256 `71fb7dd90a9bb96de2990499da280ce03e13cbd559a77477515191864977a65c`. [Release ufficiale](https://github.com/keycastr/keycastr/releases/tag/v0.11.1)
- Estrazione dei framework con link relativi confinati e metadati AppleDouble riconosciuti. Routine abilitata solo per KeyCastr. Test di confinamento, nuova estrazione e regressione ZIP Godot PASS. Prima prova rilevò differenze AppleDouble, con rollback dell’installazione precedente confermato. [Test](recipes-native-final-test.log) · [Regressione Godot](godot-final-regression.log) · [Aggiornamento](keycastr-update-receipt.json) · [Rimozione](keycastr-remove-receipt.json)
- Core 132 passati e 1 ignorato. Patch controllata con `git apply --check --whitespace=error` su `90156b3`.

