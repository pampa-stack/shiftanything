# LibreOffice su Apple Silicon

Pacchetti Apple Silicon delle release stabili 26.8.2 e 26.8.3 scaricati HTTPS dai mirror indicati da The Document Foundation. Le dimensioni e SHA-256 coincidono con i dati forniti dal publisher. Il bundle arm64 `org.libreoffice.script` richiede macOS 11 ed è firmato con Developer ID The Document Foundation (`7P5S3ZLCN7`). [Indice delle release stabili](https://download.documentfoundation.org/libreoffice/stable/)

| Release | Byte | SHA-256 ufficiale |
| --- | ---: | --- |
| 26.8.2 | 299.236.406 | [`9f24fe83ab1ed5c95eb3df580119f32932f713e577feeac02bf619fb00749b55`](https://download.documentfoundation.org/libreoffice/stable/26.8.2/mac/aarch64/LibreOffice_26.8.2_MacOS_aarch64.dmg.sha256) |
| 26.8.3 | 299.927.705 | [`de943203795e5057785f65515063e2db5eb60b0bb6d8e2534fec59e5cc5fb988`](https://download.documentfoundation.org/libreoffice/stable/26.8.3/mac/aarch64/LibreOffice_26.8.3_MacOS_aarch64.dmg.sha256) |

Il 27 novembre 2026 entrambi i DMG sono stati montati in sola lettura, verificati con `codesign --deep --strict` e accettati da Gatekeeper come `Notarized Developer ID`. Copia di `26.8.2.2`, sostituzione con `26.8.3.1` e rimozione nel cestino temporaneo sono passate. Il marker sintetico era integro dopo aggiornamento e rimozione. Nessun avvio di `soffice` o della GUI. I volumi sono stati smontati e il profilo temporaneo rimosso.

La prova usa uno script di verifica separato dal launcher. L’integrazione nativa richiede ricerca release da TDF e interpretazione delle versioni a quattro componenti. Vedere `libreoffice-integration-notes.md`, `libreoffice.log` e `libreoffice-architecture.log` nella stessa directory.
