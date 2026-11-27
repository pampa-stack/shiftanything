# Sh*ftAnything, prova desktop del 27 novembre 2026

Catalogo **528 app e dashboard**. Incluse **44 di sicurezza, 17 VPN e 18 di rete**. Controllati 541 asset e 505 loghi delle app inclusi nel pacchetto. Gli altri loghi passano dal motore Rust con le fonti curate, quando disponibili. Altrimenti la tabella mostra le iniziali. [Verifica degli asset](catalog-media.txt) · [Fonti delle aggiunte](../pro-371-catalog-522/README.md)

## Pacchetti finali

Compilati dal commit `fdf5becb0afcb7c1f09810d73cc789b1b331565a` e verificati con la finestra desktop a 1440 × 900 e profili temporanei.

- **macOS Apple Silicon**. `ShiftAnything-0.20.3-macos-arm64-tauri-local.dmg` SHA-256 `bd7597ccc25739b4a7dfe8d4f8b3fbd3ef831046cb3606d9dce39f9b25816f21`. Montato in sola lettura, con firma ad hoc verificata e avvio della finestra dalla copia estratta. SH*FTER mostra i loghi delle app nella tabella Photoshop. Trova app restituisce Handy, OpenWhispr Local e Whispering per la dettatura, e «1 risultato» per NormCap. Installa apre il consenso di Cryptomator. Rifiuta lascia l'app assente. [SH*FTER](macos-shifter-photoshop-it.png) · [Dettatura](macos-find-dictation-it.png) · [OCR](macos-find-ocr-it.png) · [Consenso](macos-cryptomator-consent.png)
- **Ubuntu x86-64**. `shiftanything_0.20.3_amd64-local.deb` SHA-256 `cc87859b2c3445e15356e698bbde5df7b96f6ff520f15bf5e07ed1b8d919a3bc`. Compilato su Ubuntu 24.04.5 e avviato dal pacchetto estratto su Ubuntu 26.04 con rete isolata. Wispr Flow mostra tre loghi nella tabella SH*FTER. Trova app restituisce tre alternative per la dettatura e «1 result» per NormCap dopo il cambio lingua. Il rifiuto del consenso di LibreOffice dà `rejected`, con app assente e job nullo. [SH*FTER](linux-final-shifter-wispr-results.png) · [Dettatura](linux-final-find-it-dictation.png) · [OCR](linux-final-find-en-singular.png) · [Consenso](linux-final-libreoffice-consent.png)
- **Test finali**. Rust core 128 passati e 1 ignorato, Tauri 12 passati, formattazione e clippy passati. MCP stdio riprovato con 528 app e 11 profili. I cicli di installazione, aggiornamento e rimozione sono documentati più sotto.

## Prove del checkpoint precedente su macOS

Sorgente del checkpoint `d6be6d4dcc18f744de140be56154a1e984f80eef`.

- **SH*FTER**. Nella tabella Little Snitch le icone sono a sinistra del nome dell'app. OpenSnitch e Portmaster Free sono visibili offline. LuLu passa dalle iniziali offline al logo ufficiale con rete disponibile. La ricerca di Tailscale mostra l'icona ufficiale anche offline. [Little Snitch offline IT](little-snitch-528-offline-it.png) · [Little Snitch online EN](little-snitch-528-online-en.png) · [Tailscale offline IT](tailscale-528-offline-it.png)
- **Trova app**. In profilo temporaneo senza rete, il testo «Vorrei scrivere in tutte le app dettando» mostra Handy per macOS. Il pacchetto finale amplia la ricerca e corregge il singolare. [Prova](find-apps-528-offline-it.png)
- **Kit di lavoro**. Il ruolo Designer mostra quattro fasi con strumenti a pagamento, alternative con fonti e collegamenti al confronto. Il pulsante per installare la selezione usa lo stato delle ricette della piattaforma. [Prova](workflow-kits-528-offline-it.png)
- **MCP e punteggi**. Client stdio reale con 528 app, 11 profili e ricerche combinate. Stessa copertura Rust e MCP su 20.337 combinazioni. Le schede senza fonti per un confronto mostrano un trattino. Le azioni MCP sono state provate in un profilo isolato e, per Handy, anche con il backend reale e il consenso nella finestra. [Ciclo Handy](../pro-371-tauri-lifecycle/README.md)

## Prove del checkpoint precedente su Linux

DEB Ubuntu 24.04+ x86-64 SHA-256 `21194a64a847369e10651355bd08de18adc0825e0db17561f6e4a8413593d8c5`. Compilato su Ubuntu 24.04.5 e avviato dal pacchetto estratto su Ubuntu 26.04, in profilo temporaneo con rete isolata. Nel confronto Wispr Flow selezionato da tastiera, si vedono i loghi di Handy, OpenWhispr Local e Whispering accanto ai nomi. Il footer riporta 528 app. Pinta mostra 57,5 MB per il pacchetto Linux Flatpak nella griglia e nella scheda, che contiene anche le dimensioni per macOS e Windows. L'anteprima Pinta resta indisponibile in questa prova offline. [Tendina](linux-wispr-option.png) · [Tabella](linux-wispr-results.png) · [Pinta in griglia](linux-pinta-card.png) · [Scheda Pinta](linux-pinta-detail.png)

## Prove di installazione, aggiornamento e rimozione

- **macOS Apple Silicon**. DMG del checkpoint precedente SHA-256 `b7c1064bebe4378a58eb0dc21e1f3145b49016d2a7ff9c0217823d0ef96ae273`. Montato in sola lettura, controllati codice con firma ad hoc, bundle e licenze. Finestra aperta dalla copia estratta. 21 app hanno evidenza nativa di installazione, aggiornamento e rimozione da pacchetti ufficiali. Le prove di apertura sono separate, per app. [Ultima ricetta, Cryptomator](../pro-371-macos-cryptomator/README.md)
- **Linux x86-64**. 17 app hanno evidenza nativa di installazione, apertura, aggiornamento e rimozione da revisioni firmate Flathub. Le prove LocalSend e ArtCraft AppImage hanno canali separati. [Ultima ricetta, LibreOffice](../pro-371-linux-flathub/libreoffice/README.md) · [Aggiornamento di due app dalla finestra](../pro-371-tauri-bulk-flathub/README.md) · [Annullamento di GIMP dalla finestra](../pro-371-tauri-cancel-flathub/README.md)

## Prima di distribuire

Resta da coprire il catalogo installabile per piattaforma e verificare la parità visiva completa con Doop. Mac Intel resta da provare. Il DMG locale ha firma ad hoc. Le prove di Windows sono arrivate a Windows Setup avviato da disco, con VM fermata al limite del disco di prova e sotto al margine RAM configurato. Desktop, pacchetto MSI e installazioni Windows restano aperti. L'aggiornamento del catalogo e delle fonti SH*FTER è riproducibile da script. La pianificazione automatica esterna resta da attivare. [Referto Windows](../pro-371-windows-setup/desktop/README.md)
