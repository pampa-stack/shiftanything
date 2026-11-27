# SH*FTER con l'intero catalogo

27 novembre 2026. Finestra Tauri nativa macOS Apple Silicon, profilo temporaneo, 1440 × 900. L'apertura del confronto Figma da Kit di lavoro presenta 419 risultati in italiano e inglese. Doop e Penpot risultano con 7 funzioni coperte su 8, FigCraft con 6. La colonna Indispensabili compare scegliendo Migrazione dei file Figma. Le schede senza evidenze mostrano un trattino. Il pulsante Fonti compare sulle schede con evidenze.

La prova browser locale, con risposta nativa di fixture, ha percorso tutte le 35 pagine, verificato ricerca bilingue, 419 ID distinti, tabella senza overflow del documento da mobile e transizioni della colonna Indispensabili. Il protocollo MCP su stdio reale ha restituito l'intero catalogo dai nove profili e nel confronto manuale. `verify_shifter.py` concorda con Rust su 20.175 selezioni.

Il confronto è documentale. L'installazione delle app è verificata da prove separate.

[Finestra macOS con i loghi di Doop, Penpot e FigCraft](figma-logos-en.png). SH*FTER usa il caricamento nativo del catalogo per le righe in vista. Un riquadro chiaro rende leggibile il logo scuro di Penpot.

Con il catalogo da 462 app, [confronto Figma in italiano online](figma-logos-462-it.png) e [dallo stesso confronto in un profilo nuovo a rete disabilitata](figma-logos-offline-it.png). I loghi di Doop, Penpot, FigCraft, draw.io ed Excalidraw sono inclusi nel pacchetto. Per le righe prive di un logo locale o scaricato la tabella mostra le iniziali.

Altri 32 loghi sono inclusi per le alternative dei profili documentati. [Il confronto Photoshop, aperto nella build Mac con rete disabilitata](photoshop-logos-offline-it.png), mostra Krita, PhotoCraft, GIMP, Drawpile e Pinta con i rispettivi loghi.

Build macOS Apple Silicon `4fe44dd`, estratta dal DMG locale verificato. La finestra 1440 × 900 avviata dal bundle in un profilo temporaneo con connessioni in uscita bloccate mostra 479 app. Verificati [i loghi nel confronto Photoshop](photoshop-479-offline-it.png) e [i loghi nel confronto Figma](figma-479-offline-it.png) selezionando Figma dalla tendina.

Il pacchetto Ubuntu 24.04+ amd64 della stessa build, estratto e aperto su VPS Strumenti (Ubuntu 26.04, Xvfb 1440 × 900 con rete isolata), riporta 479 app e mostra [gli stessi loghi in SH*FTER](photoshop-479-offline-linux.png). Il pacchetto contiene la scheda del menu e l'icona. La prova usa l'eseguibile estratto in una directory temporanea.

Build Mac `34266dc`, DMG locale `9057444c2132f2ec7f8737820cee24467c36985d7fbf83c729ee5f9848ef37b0`. Il pacchetto include 359 loghi app con fonte e SHA-256. Aperto dal bundle estratto, in profilo nuovo con rete bloccata. Nella tabella [Photoshop](photoshop-359-icons-offline-it.png) sono leggibili anche AbiWord, Actual Budget, AdGuard Home, AiderDesk e AionUi. Nella tabella [Figma](figma-359-icons-offline-it.png) sono visibili Doop, Penpot, FigCraft, draw.io e Excalidraw. Le righe senza logo disponibile mostrano le iniziali.

## 471 icone incluse

Le prove della build `b78fa1a` usano i 471 loghi locali verificati uno per uno con SHA-256 sui 479 record. I PNG derivati da SVG sono renderizzati prima della build. Le otto schede rimanenti mostrano le iniziali finché un logo è indisponibile.

Mac Apple Silicon, bundle estratto dal DMG locale `dc6e2a78de22d8787fde70a930da17078bd8357422400d05d0e2040a8871d7b8`, nuova home e rete in uscita bloccata. [Photoshop](photoshop-471-offline-it.png), [Figma](figma-471-offline-it.png) e [ricerca Wazuh](wazuh-471-offline-it.png), tutti in finestra nativa a 1440 × 900.

Ubuntu 24.04+ amd64, pacchetto locale `6437fee7970acadd7035201d26bae86ff5aaf36e1d241bff515d4a6ca763b28f` verificato prima e dopo il trasferimento. Il binario estratto dal pacchetto è stato aperto in Ubuntu 26.04, Xvfb 1440 × 900 e rete isolata. [Photoshop su Linux](photoshop-471-offline-linux.png). La prova Tauri misura 479 riquadri uniformi a 408,328 × 400 px e verifica viewport, barra, sidebar e media.

Avviato anche l'eseguibile del pacchetto senza argomenti, come da voce di menu Ubuntu. Il log conferma il motore nativo attivo e il [confronto Photoshop](photoshop-471-default-start-linux.png) mostra i loghi anche in questo percorso, con rete isolata.

## 477 icone incluse

La build `f569e40` aggiunge i loghi di Authentik Community, Cardinal, Enchanted, mailcow, Stats e WireGuard for Windows da siti o repository ufficiali. Il [manifest](../../../apps/shiftanything-tauri-proof/media-provenance.json) registra URL e SHA-256 per tutti i file. Sono stati controllati i 477 hash e la decodifica delle 477 icone sul catalogo di 479 app. Whisper-WebUI e VideoX-Fun mostrano le iniziali.

Mac Apple Silicon, DMG locale `e27b4e501af71343e62ef369093a63e30f99c0b43e5eb2c7e6ac1b150c4d5725`. Finestra aperta dalla copia estratta, con profilo nuovo e rete in uscita bloccata. [Confronto Photoshop](photoshop-477-offline-it.png) · [Icona di WireGuard for Windows nel confronto](wireguard-477-offline-it.png) · [Immagini ufficiali esaminate](official-logo-sources.png). Core 127 passati, uno ignorato, e Tauri 12 passati.

Ubuntu x64, DEB locale `ad6525c2bc3b3bf00ed8f84a5851bcc86907225601c1a81833913410cc04ee09`. Verificato via SHA-256, estratto ed eseguito senza argomenti con profilo nuovo e rete isolata su Ubuntu 26.04. Le prove mostrano [il confronto Photoshop](photoshop-477-offline-linux.png) e [il logo aggiunto di WireGuard](wireguard-477-offline-linux.png). La finestra ha avviato il motore nativo e il pacchetto contiene la voce di menu e l’icona Linux.
