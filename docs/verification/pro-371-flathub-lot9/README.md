# PRO-371 · Nono lotto desktop Flathub su Linux x86-64

Prova NativeBackend del 27 novembre 2026 su VPS Strumenti. Per ogni app profilo per utente temporaneo, finestra Xvfb privata, bus D-Bus separato, rete ridotta al loopback. Nessuna apertura di dati personali.

Licenza libera e publisher verificato. [Firefox](https://flathub.org/apps/org.mozilla.firefox) · [VSCodium](https://flathub.org/apps/com.vscodium.codium) · [LibreCAD](https://flathub.org/apps/org.librecad.librecad) · [PrusaSlicer](https://flathub.org/apps/com.prusa3d.PrusaSlicer) · [GnuCash](https://flathub.org/apps/org.gnucash.GnuCash) · [FamiStudio](https://flathub.org/apps/org.famistudio.FamiStudio) · [Seamly2D](https://flathub.org/apps/io.seamly.seamly2d) · [nomacs](https://flathub.org/apps/org.nomacs.ImageLounge).

Il ciclo copre azioni NativeBackend e apertura dei due pacchetti con verifica delle firme. Prima di simulare il profilo storico, il motore rifiuta modifica o rimozione di una copia esterna. Il marker del profilo privato resta dopo aggiornamento e rimozione. Firefox, VSCodium e GnuCash sono riprove grafiche con window manager e attesa aumentata.

## Risultati

Ciclo completato per 4 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| Firefox 160.0.1 → 161.0 | 41,34 | 30,56 / 35,69 | 10,93 | 4,42 | 142,12 |
| VSCodium 1.140.06880 → 1.143.17232 | 54,42 | 18,93 / 19,30 | 12,84 | 5,93 | 161,97 |
| LibreCAD 2.2.1.5 → 2.2.1.6 | 40,92 | 6,80 / 6,89 | 7,53 | 4,93 | 85,32 |
| GnuCash 5.16+ (Flathub 5.16) → 5.17+ (Flathub 5.17) | 49,09 | 21,99 / 21,80 | 8,64 | 4,93 | 125,45 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| Firefox | `app/org.mozilla.firefox/x86_64/stable` | `3ab15a895b6a9935e2cc33dd2771c600422ed3cfa153bcadf8c60dff5b8f41fc` | `b8dd418bd71860acb006d755db5b47184d7b88a6dea4f626dc52a1dc5f2dc96e` |
| VSCodium | `app/com.vscodium.codium/x86_64/stable` | `bdf75afd9a0cbd14a12e0dfa9a68904e67b2176e400b2be339a083bf84b3c574` | `b63788f25cee55b738d5aa6df3b864c237bda087ce0ee43e803e5a7b207a25ab` |
| LibreCAD | `app/org.librecad.librecad/x86_64/stable` | `fa1853ecb789acc9761826876e48f92e3bc8c75cef24d654d495a59324122001` | `28541921d9f3a9293c7b7a803d3381c3874013e13555270f055412c506a92585` |
| GnuCash | `app/org.gnucash.GnuCash/x86_64/stable` | `ef3f97227486c297ce4e89b21e22122342a544ee1660d43df7625727790b7edf` | `1ae4ecbe79a6a4f4b3f8911697a3712c7a5c23cf6e34076d9c15424607d7a67e` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- Firefox. Finestra con barra di Firefox e pagina interna Welcome to Firefox renderizzate in entrambe le revisioni. La pagina chiede accettazione dei termini. Nessun consenso dato e nessun sito aperto; isolamento verificato dalla rete del processo ospite. [Prima](evidence/firefox/previous.png) · [Dopo](evidence/firefox/current.png) · [Log](evidence/firefox/lifecycle.log)
- VSCodium. Editor VSCodium leggibile nelle due revisioni in modalità ristretta. Storico sul README del pacchetto, corrente su avviso di migrazione SDK 26.08. Le due release impiegano SDK ufficiali 25.08 e 26.08. Nessuna estensione attivata, nessuna cartella utente aperta. [Prima](evidence/vscodium/previous.png) · [Dopo](evidence/vscodium/current.png) · [Log](evidence/vscodium/lifecycle.log)
- LibreCAD. Finestra Welcome to LibreCAD leggibile in entrambe le revisioni, con controlli per unità del disegno, lingua GUI, lingua dei comandi e conferma. Nessun progetto aperto. [Prima](evidence/librecad/previous.png) · [Dopo](evidence/librecad/current.png) · [Log](evidence/librecad/lifecycle.log)
- GnuCash. Finestra Unsaved Book e procedura Welcome to GnuCash leggibili in entrambe le revisioni. Suggerimento Xvfb chiuso nel display privato. Visibili menu Reports, Actions, Business e controlli per creare/importare un piano conti. Nessun dato contabile importato. [Prima](evidence/gnucash/previous.png) · [Dopo](evidence/gnucash/current.png) · [Log](evidence/gnucash/lifecycle.log)

Il runner è compilato da `2a8800e1bf2fddbcc8cc2be327ae5815c9f498a0` più la matrice di prova. Le quattro ricette sono in [flathub.rs](../../../crates/core/src/flathub.rs). Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `4cfa3f8dd0c252a605a1ef3d8e44fc9ff50cedc9de58ec3473ff461cf11e381c`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.8NiwE1xk`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Storico Flatpak](history.log) · [Manifest SHA256](SHA256SUMS)
