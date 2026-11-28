# Kopia su Ubuntu 24.04 x64

Il 28 novembre 2026, in una VM AX42 temporanea, il backend Rust ha installato KopiaUI da Flathub, rilevato la voce nel menu e l’icona, aperto le due revisioni, aggiornato la precedente e rimosso l’app. Ha conservato i dati sintetici dopo aggiornamento e rimozione. Quando la revisione precedente era registrata come installazione esterna, aggiornamento e rimozione sono stati rifiutati. Il runner ha quindi registrato quella revisione nel solo profilo QA per riprodurre un aggiornamento gestito.

Entrambe le revisioni riportano 0.23.1. Il backend distingue l’aggiornamento dal commit firmato. Entrambe mostrano la schermata di selezione dello storage. I due PNG sono identici byte per byte. Durante le aperture la VM ha usato il solo loopback, nessuno storage è stato configurato e l’accesso ai file e ai dispositivi dell’host è stato limitato nel profilo QA. Il manifest del pacchetto comunitario dichiara accesso ai file dell’host e alla rete.

La 0.23.0 in AppImage ufficiale si è fermata all’avvio con un errore del sandbox Electron. La 0.23.1 in AppImage ha avviato Electron con `--no-sandbox` dal proprio pacchetto anche quando il comando di prova non passava il flag. Il launcher usa il Flatpak comunitario firmato, che avvia KopiaUI con Zypak. Il pacchetto per macOS continua a usare il DMG ufficiale.

[Flathub](https://flathub.org/apps/io.kopia.KopiaUI) · [Manifest del pacchetto comunitario](https://github.com/flathub/io.kopia.KopiaUI) · [Finestra](initial-window.png) · [Log](lifecycle.log) · [Firma precedente](previous.signature.txt) · [Firma corrente](current.signature.txt) · [Sandbox della prova](sandbox.txt)
