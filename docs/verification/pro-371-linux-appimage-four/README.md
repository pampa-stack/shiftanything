# Quattro AppImage ArtCraft su Linux x86-64

27 novembre 2026, VPS1 Ubuntu. Il banco usa il motore Rust condiviso con il launcher. Ogni ciclo installa dal repository ufficiale, apre la prima versione, aggiorna, apre la seconda e rimuove. I digest SHA-256 delle release corrispondono ai file installati, i dati sintetici rimangono e i quattro profili di prova sono stati eliminati.

| App | Installata → aggiornata | Durata con download | Finestre |
| --- | --- | ---: | --- |
| Craft Anything | 0.6.0 → 0.6.1 | 128 s | [Iniziale](craft-anything/launch-old.png) · [Aggiornata](craft-anything/launch.png) |
| DesignCraft | 0.20.1 → 0.21.1 | 130 s | [Iniziale](designcraft/launch-old.png) · [Aggiornata](designcraft/launch.png) |
| MailCraft | 0.14.0 → 0.15.0 | 70 s | [Iniziale](mailcraft/launch-old.png) · [Aggiornata](mailcraft/launch.png) |
| SolidCraft | 0.49.1 → 0.50.0 | 132 s | [Iniziale](solidcraft/launch-old.png) · [Aggiornata](solidcraft/launch.png) |

Le finestre usano Xvfb, un bus DBus privato, Bubblewrap senza rete e software rendering. La finestra aggiornata di SolidCraft mostra il report di chiusura non pulita della prima versione quando termina il sandbox. Per MailCraft si raggiunge la configurazione account senza effettuare l'accesso. Queste prove coprono l'apertura delle finestre dal motore, non l'interazione con la finestra Tauri o le funzioni interne delle app.

[Banco Rust](verify.rs) · [Procedura](run.sh) · [Digest prove](SHA256SUMS.txt). Gli input release e il binario collegato alle librerie Rust del banco sono nello scratch VPS1 `/mnt/dati/lavori/shiftanything/remaining-appimages.X6onMrzB/`.
