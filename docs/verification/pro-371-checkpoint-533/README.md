# Build locale Sh*ftAnything con 533 app

27 novembre 2026. Sorgente di entrambi i pacchetti `6fadcc03ef901762716c1d501ea3d15ef7d8f3b9`. Il catalogo include 45 schede di sicurezza difensiva, 18 di VPN e accesso remoto e 21 di rete. [Fonti delle ultime aggiunte](../pro-371-catalog-533/README.md)

## Mac Apple Silicon

`ShiftAnything-0.20.3-macos-arm64-tauri-local.dmg`, SHA-256 `4d17ae29c9137fb1283d87c89f90c19d2bcc5b3cabc9385523f76c6ed6bc0b0a`. Verificati firma ad hoc, mount in sola lettura, bundle, licenze e versione. Finestra a 1440 × 900 aperta dalla copia estratta, in italiano con profilo temporaneo. Il confronto Photoshop mostra il marchio Adobe nella selezione e nell'intestazione, e i loghi di Krita, PhotoCraft, GIMP, PhotoDemon, Drawpile e Pinta in tabella. [Schermata SH*FTER](macos-shifter-photoshop-it.png)

Termix ha passato installazione, aggiornamento e rimozione dal motore nativo su Apple Silicon. La ricerca della release 26.11.3 è riuscita anche con l’API GitHub bloccata. Apertura di Termix e Dock rimangono da provare. [Ciclo, fonti e condizioni della prova](../pro-371-macos-termix/README.md)

## Ubuntu 24.04.5 x86-64

`shiftanything_0.20.3_amd64-local.deb`, SHA-256 `42f182209b489fdf4698e0172c0bee2ee46dd616ec1919338c7b0db1c788f510`. Verificati il checksum dopo la copia, versione del binario estratto, architettura, voce di menu e icona. Il binario estratto dal DEB è stato aperto in un profilo temporaneo con Xvfb e rete limitata a loopback. Il report WebKit a 1440 × 900 ha confermato 533 schede da 408,328 × 400 px, con le prime sei copertine e i primi sei loghi caricati. SH*FTER aperto con un clic mostra il marchio Adobe e i loghi delle alternative. La prova riguarda finestra e catalogo. [Schermata SH*FTER](linux-shifter-photoshop-it.png) · [Catalogo](linux-catalog-it.png)

## Copertura

Su Mac sono passati 130 test core, 560 native e 12 Tauri, con 8 test preesistenti ignorati. Formattazione e Clippy passati. MCP stdio ha verificato 533 schede e 11 profili, con azioni in fixture IPC isolate per questo checkpoint.

La matrice Rust conta 40 voci con ricetta nativa candidata su Mac Apple Silicon, 17 su Mac Intel, 18 su Linux ARM64, 19 più 20 ricette Flathub stabili su Linux x86-64 e 18 per ciascuna architettura Windows. Sono conteggi di ricette. I cicli reali sono documentati app per app. [Matrice](recipe-scope.json) · [Prove precedenti](../pro-371-checkpoint-528/README.md) · [Windows, prova di avvio](../pro-371-windows-setup/README.md)
