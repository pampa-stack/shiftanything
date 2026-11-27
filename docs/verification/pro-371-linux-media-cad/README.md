# HandBrake, Shotcut e FreeCAD su Linux x86-64

27 novembre 2026. Tre schede già presenti nel catalogo usano ora il canale stabile Flathub su Linux x86-64. La [matrice generata da Rust](recipe-scope.json) riporta 23 ricette Flathub stabili e 533 schede totali.

| App | Release provata | Licenza indicata da Flathub | Fonte | Finestra |
| --- | --- | --- | --- | --- |
| HandBrake | 1.11.1 → 1.11.2 | GPL-2.0-only | [Flathub](https://flathub.org/apps/fr.handbrake.ghb) | [Schermata](handbrake-1.11.2.png) |
| Shotcut | 26.9.27 → 26.11.15 | GPL-3.0-only | [Flathub](https://flathub.org/apps/org.shotcut.Shotcut) | [Schermata](shotcut-26.11.15.png) |
| FreeCAD | 1.1.4 → 1.1.4 con cambio di revisione Flathub | LGPL-2.1 | [Flathub](https://flathub.org/apps/org.freecad.FreeCAD) | [Schermata](freecad-1.1.4.png) |

Il motore nativo Rust ha scaricato, installato, avviato, aggiornato e rimosso i tre pacchetti Flathub in profili utente separati su Ubuntu 26.04 x86-64. La prova ha verificato la firma OSTree di entrambe le revisioni, la voce di menu e l’icona esportate, il rifiuto di aggiornare una copia esterna e la conservazione dei dati del profilo dopo aggiornamento e rimozione.

Per simulare un aggiornamento, la prova ripristina la revisione storica firmata e registra quella revisione nel manifest del solo profilo temporaneo, dopo la prova del rifiuto. Le app si aprono con Xvfb, un bus privato e la sola interfaccia di rete loopback. Lo screenshot di FreeCAD proviene da un secondo avvio della release aggiornata con più tempo per il rendering. I cicli specifici di queste tre ricette sono stati eseguiti con i comandi del motore nativo. La prova dei clic Tauri per queste ricette resta separata.

[Estratto del log nativo](native-cycle.txt)

## Pacchetto Ubuntu e prova dei pulsanti

Il DEB locale da `ff517db` è stato compilato su Ubuntu 24.04.5 e ha SHA-256 `5329bce9470dcd59e1fb59eda3bedab30e61eea76c1c62714f872fedbc68f64e`. Verificati versione, architettura, checksum dopo la copia, voce di menu e icona. La finestra estratta dal pacchetto mostra 533 schede e 60 stati nativi, con geometria uniforme. Nel comparatore sono visibili i loghi di Figma, Doop, Penpot e FigCraft. [Tendina](linux-shifter-figma-picker-it.png) · [Confronto](linux-shifter-figma-it.png)

Su Ubuntu 26.04, la finestra dello stesso DEB ha rifiutato il primo consenso di installazione di HandBrake senza avviare un job. Il consenso successivo ha installato HandBrake 1.11.2 dal ref Flathub. Dopo la rimozione dalla scheda, il ref è risultato assente e il marker del profilo è rimasto intatto. Il clic Apri è arrivato al motore, che ha accettato la richiesta anche se la finestra HandBrake non è comparsa nel contenitore. L'apertura grafica e l'aggiornamento documentati sopra sono stati eseguiti con il banco del motore. [Consenso](linux-handbrake-consent.png) · [Rimozione](linux-handbrake-removed.png)

## Copertura delle prove del launcher

- Linux x86-64. 38 app con installazione, avvio, aggiornamento e rimozione documentati dal motore. Sono 23 Flatpak e 15 AppImage. Le prove di finestra Tauri includono installazione, aggiornamento multiplo e rimozione per Calcolatrice GNOME e GNOME Text Editor. I controlli sull'uso delle funzioni interne delle app sono più limitati, tra cui l'anteprima di Kdenlive in Xvfb. [Flatpak](../pro-371-linux-flathub/README.md) · [AppImage](../pro-371-tauri-linux-recipes.md) · [Aggiornamento multiplo](../pro-371-tauri-bulk-flathub/README.md)
- Mac Apple Silicon. 24 app con installazione, aggiornamento e rimozione documentati, incluse Meetily e Rizzo PII. I clic Tauri per queste operazioni sono documentati per Handy, LocalSend e OBS. Per KopiaUI sono documentati installazione e rimozione Tauri, aggiornamento dal motore e verifica del Dock. La finestra di KopiaUI si è fermata al caricamento. Le prove di avvio grafico sono tracciate a parte. [Handy](../pro-371-tauri-lifecycle/README.md) · [LocalSend](../pro-371-tauri-macos-localsend.md) · [OBS](../pro-371-tauri-macos-obs.md) · [KopiaUI](../pro-371-tauri-kopia/README.md) · [Meetily e Rizzo](../pro-371-macos-meetily-rizzo/README.md)

Fonti del conteggio Mac. [Audacity e KeePassXC](../pro-371-macos-audacity-keepassxc/README.md) · [LibreOffice](../pro-371-macos-audacity-keepassxc/libreoffice-native-report.md) · [Joplin](../pro-371-macos-joplin/README.md) · [ActivityWatch](../pro-371-macos-activitywatch/README.md) · [Cryptomator](../pro-371-macos-cryptomator/README.md) · [Godot](../pro-371-macos-godot/README.md) · [Termix](../pro-371-macos-termix/README.md) · [Pinta e Shotcut](../pro-371-macos-creative/README.md) · [Transmission, IINA e HandBrake](../pro-371-macos-transmission-iina-handbrake/README.md) · [Stats e Whispering](../pro-371-stats-whispering/README.md) · [Ente Auth](../pro-371-ente-auth/README.md) · [draw.io Desktop e Zed](../pro-371-daily-apps-505/README.md).
