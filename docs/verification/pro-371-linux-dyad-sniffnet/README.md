# Dyad e Sniffnet su Linux x86-64

Il 28 novembre 2026, in una VM Ubuntu 24.04 su AX42, il motore Rust ha scaricato i pacchetti AppImage originali, installato e aperto entrambe le versioni, aggiornato e rimosso le due app. Ha verificato il volume di download, il digest pubblicato e la versione installata. Le voci nel menu Applicazioni sono state create, aggiornate e rimosse. I dati sintetici e le preferenze sono rimasti integri fino alla rimozione del profilo di test.

| App | Versioni provate | Finestre | Registro |
| --- | --- | --- | --- |
| Dyad | 1.23.0 → 1.24.0 | [Iniziale](dyad/launch-old.png) · [Aggiornata](dyad/launch.png) | [Registro](dyad/lifecycle.log) |
| Sniffnet | 1.5.1 → 1.6.0 | [Iniziale](sniffnet/launch-old.png) · [Aggiornata](sniffnet/launch.png) | [Registro](sniffnet/lifecycle.log) |

[Release di Dyad](https://github.com/dyad-sh/dyad/releases) · [Release di Sniffnet](https://github.com/GyulyVGC/sniffnet/releases)

Il banco usa una cache di metadati temporanea per selezionare le versioni sopra, e trasferisce i file via HTTPS dal repository ufficiale. La GUI gira con Xvfb e Bubblewrap senza rete. Dyad usa il flag Electron per il solo test dentro Bubblewrap, poi chiude la finestra con il normale comando di chiusura. Sniffnet usa le librerie di sistema del guest `libxkbcommon-x11-0` e `libpcap0.8t64`. La verifica riguarda installazione, avvio, aggiornamento e rimozione dal motore nativo. Le funzioni AI di Dyad e la cattura del traffico di Sniffnet sono fuori dal ciclo di prova.

Per riprodurre in una VM usa `verify.rs` come binario del crate nativo, copialo come `verify` accanto a `run.sh` e avvia `bash run.sh dyad` o `bash run.sh sniffnet` in un guest di test con hostname `shift-launcher-qa` e Xvfb, Bubblewrap, xdotool, DBus e Python Pillow.
