# PRO-371 · Decimo lotto desktop Flathub su Linux x86-64

Prova NativeBackend del 27 novembre 2026 su VPS Strumenti. Per ogni app profilo per utente temporaneo, finestra Xvfb privata, bus D-Bus separato, rete ridotta al loopback. Nessuna apertura di dati personali.

Licenza libera e publisher verificato nei candidati. [Evince](https://flathub.org/apps/org.gnome.Evince) · [Flatseal](https://flathub.org/apps/com.github.tchx84.Flatseal) · [GNOME Maps](https://flathub.org/apps/org.gnome.Maps) · [QGIS](https://flathub.org/apps/org.qgis.qgis) · [Mumble](https://flathub.org/apps/info.mumble.Mumble) · [Logseq](https://flathub.org/apps/com.logseq.Logseq) · [Anki](https://flathub.org/apps/net.ankiweb.Anki) · [LeoCAD](https://flathub.org/apps/org.leocad.LeoCAD).

Il ciclo copre azioni NativeBackend e apertura dei due pacchetti con verifica delle firme. Prima di simulare il profilo storico, il motore rifiuta modifica o rimozione di una copia esterna. Il marker del profilo privato resta dopo aggiornamento e rimozione.

## Risultati

Ciclo completato per 2 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| Flatseal 2.4.0 → 2.4.1 | 36,96 | 10,21 / 9,93 | 6,33 | 4,93 | 98,39 |
| Anki 26.09.4 → 26.09.4 | 51,88 | 16,50 / 16,99 | 7,54 | 5,83 | 119,67 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| Flatseal | `app/com.github.tchx84.Flatseal/x86_64/stable` | `14ba14f237835365b3b5f2c8f6eee2dcaf7f248d92f3eaf51e00ab28cfe523b1` | `ef9fe38e9cb96c170ea579fe1bbf8c76011255d4962d7fc4b3aa4e0a6063f8ae` |
| Anki | `app/net.ankiweb.Anki/x86_64/stable` | `3f79a0ca0556910be028d43f5eb7b2cc4a823c1ca4341e371619f7e8ab3fa40f` | `53e471bfc1d618952be26572f85a3b6965a70d0c5e2fb4240c985d871b8ddd00` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- Flatseal. Interfaccia Applications, ricerca e controlli leggibili nelle due revisioni. “No applications found” è lo stato atteso nel banco senza accesso alle altre installazioni. Nessun permesso modificato. [Prima](evidence/flatseal/previous.png) · [Dopo](evidence/flatseal/current.png) · [Log](evidence/flatseal/lifecycle.log)
- Anki. Selettore di lingua di Anki leggibile al primo avvio delle due revisioni, con elenco e conferma. Nessuna sincronizzazione, mazzo o account toccati. [Prima](evidence/anki/previous.png) · [Dopo](evidence/anki/current.png) · [Log](evidence/anki/lifecycle.log)

Il runner è compilato da `6d1506439ee95a2be90d15b73480fe5910135df1` più la matrice di prova. Le due ricette sono in [flathub.rs](../../../crates/core/src/flathub.rs). Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `22694085082ae3c449b6a96f29cde124234eb54ce8a071772a0568c9b9a6beb4`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.Ny73CjmA`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Manifest SHA256](SHA256SUMS)
