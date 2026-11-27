# LibreOffice su macOS Apple Silicon

Il 27 novembre 2026 alle 05:00 CET il ciclo NativeBackend ha installato 26.8.2, selezionato 26.8.3 con Controlla aggiornamenti, aggiornato e rimosso l'app. Il test ha ristretto Applications, metadati e Cestino a un profilo usa e getta, senza avviare LibreOffice. Il marker del profilo è rimasto intatto in tutte le fasi. Dopo la rimozione il bundle era nel Cestino del test e la ricevuta nativa era assente. Il profilo è stato cancellato.

| Versione stabile | Bundle | Byte scaricati | SHA-256 |
|---|---|---:|---|
| 26.8.2 | 26.8.2.2 | 299236406 | `9f24fe83ab1ed5c95eb3df580119f32932f713e577feeac02bf619fb00749b55` |
| 26.8.3 | 26.8.3.1 | 299927705 | `de943203795e5057785f65515063e2db5eb60b0bb6d8e2534fec59e5cc5fb988` |

I pacchetti sono arrivati dall'[indice stabile TDF](https://download.documentfoundation.org/libreoffice/stable/). Architettura ARM64, macOS 11+, bundle `org.libreoffice.script`, Developer ID Application di The Document Foundation `7P5S3ZLCN7`. La firma e Gatekeeper attivo hanno accettato entrambe le versioni notarizzate.

La ricetta usa solo i DMG Apple Silicon della directory stabile. Deriva dal tag l'URL esatto del DMG, verifica dimensione e SHA-256 nel relativo Metalink HTTPS e ignora i mirror elencati nei metadati. Il download consente redirect HTTPS. Il tag pubblico usa tre componenti. La ricetta limita al bundle LibreOffice il riconoscimento del quarto componente numerico e usa le prime tre cifre per l'aggiornamento. Un respin dello stesso tag con solo il quarto componente differente è fuori dalla comparazione.

Il probe `libreoffice-native-cycle.rs` e il wrapper `libreoffice-native-cycle.sh` affiancano il log `libreoffice-native-cycle.log`. Il probe si collega agli artefatti prodotti da `cargo build -p artcraft-launcher-native --lib --message-format=json` salvati in `libreoffice-cargo-build.jsonl` tramite `build-libreoffice-native-cycle.py`. Il wrapper richiede le versioni esatte riportate sopra. Una nuova release stabile rende obsoleta l'aspettativa sul controllo aggiornamenti, quindi il test deve scegliere due tag pubblicati e aggiornarne gli hash attesi.
