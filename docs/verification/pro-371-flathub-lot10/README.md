# PRO-371 · Decimo lotto desktop Flathub su Linux x86-64

Prova NativeBackend del 27 novembre 2026 su VPS Strumenti. Per ogni app profilo per utente temporaneo, finestra Xvfb privata, bus D-Bus separato, rete ridotta al loopback. Nessuna apertura di dati personali.

Licenza libera e publisher verificato nei candidati. [Evince](https://flathub.org/apps/org.gnome.Evince) · [Flatseal](https://flathub.org/apps/com.github.tchx84.Flatseal) · [GNOME Maps](https://flathub.org/apps/org.gnome.Maps) · [QGIS](https://flathub.org/apps/org.qgis.qgis) · [Mumble](https://flathub.org/apps/info.mumble.Mumble) · [Logseq](https://flathub.org/apps/com.logseq.Logseq) · [Anki](https://flathub.org/apps/net.ankiweb.Anki) · [LeoCAD](https://flathub.org/apps/org.leocad.LeoCAD).

Il ciclo copre azioni NativeBackend e apertura dei due pacchetti con verifica delle firme. Prima di simulare il profilo storico, il motore rifiuta modifica o rimozione di una copia esterna. Il marker del profilo privato resta dopo aggiornamento e rimozione.

## Risultati

Ciclo completato per 7 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| Evince 48.4 → 48.5 | 40,79 | 9,98 / 9,74 | 6,73 | 5,73 | 119,34 |
| Flatseal 2.4.0 → 2.4.1 | 36,96 | 10,21 / 9,93 | 6,33 | 4,93 | 98,39 |
| GNOME Maps 51.2 → 51.3 | 39,19 | 16,54 / 15,61 | 6,33 | 4,49 | 94,30 |
| QGIS 4.4.0 → 4.4.0 | 61,12 | 24,83 / 24,52 | 8,43 | 6,64 | 139,43 |
| Logseq 0.10.14 → 0.10.15 | 38,38 | 21,36 / 19,37 | 12,30 | 5,62 | 138,11 |
| Anki 26.09.4 → 26.09.4 | 51,88 | 16,50 / 16,99 | 7,54 | 5,83 | 119,67 |
| LeoCAD 25.09 → 26.09 | 47,84 | 11,45 / 10,33 | 9,54 | 6,02 | 149,33 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| Evince | `app/org.gnome.Evince/x86_64/stable` | `dc553d3b8e4d7e32cc674ebc184a49ba2609fbec24539e71dcf16e80367c6931` | `03718c051588a30f2bd07769c0545e44c04ee708d4129ce6524dbf0ed4d082ac` |
| Flatseal | `app/com.github.tchx84.Flatseal/x86_64/stable` | `14ba14f237835365b3b5f2c8f6eee2dcaf7f248d92f3eaf51e00ab28cfe523b1` | `ef9fe38e9cb96c170ea579fe1bbf8c76011255d4962d7fc4b3aa4e0a6063f8ae` |
| GNOME Maps | `app/org.gnome.Maps/x86_64/stable` | `793510ce8befb86375514cd1b2a517f36b83c9cae2faa930ba19d29be5414966` | `cae20ad6fd95b440a53b7e8c4fa36d12bac07cc10c0eac341e1e8c8f01018c34` |
| QGIS | `app/org.qgis.qgis/x86_64/stable` | `0c09ba71e4f0eeafe5fb996eec9a5302269b29f302185ca989a5e2b57de9923d` | `c64f2df502de93319219a34675e82157d979718a8da4ddeb6d47bda87e7a2c4e` |
| Logseq | `app/com.logseq.Logseq/x86_64/stable` | `e0e49d396a630c00ba46f859bbfcb5e3fe4455cc089cd4a776b77029783d60cd` | `7579d39d56379d88969a93a201c1656b3d12936faa3845b1db6bbb521ecb34f1` |
| Anki | `app/net.ankiweb.Anki/x86_64/stable` | `3f79a0ca0556910be028d43f5eb7b2cc4a823c1ca4341e371619f7e8ab3fa40f` | `53e471bfc1d618952be26572f85a3b6965a70d0c5e2fb4240c985d871b8ddd00` |
| LeoCAD | `app/org.leocad.LeoCAD/x86_64/stable` | `99a8416a25e08bfebb78b2486f48b8523b067b7276df8d505a5304125ad5fb45` | `c8e9f0aeba03cbbd615a8bb274b8cc4bbe8cf92bd6161f1001c79f8c5a03ee60` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- Evince. Finestra iniziale Document Viewer leggibile nelle due revisioni, con azione Open. Archivio di documenti recenti vuoto nel profilo temporaneo. Nessun PDF personale aperto. [Prima](evidence/evince/previous.png) · [Dopo](evidence/evince/current.png) · [Log](evidence/evince/lifecycle.log)
- Flatseal. Interfaccia Applications, ricerca e controlli leggibili nelle due revisioni. “No applications found” è lo stato atteso nel banco senza accesso alle altre installazioni. Nessun permesso modificato. [Prima](evidence/flatseal/previous.png) · [Dopo](evidence/flatseal/current.png) · [Log](evidence/flatseal/lifecycle.log)
- GNOME Maps. Letti entrambi i PNG: finestra con ricerca, livelli, preferiti, zoom e scala. Le tile geografiche richiedono rete, isolata durante entrambe le aperture. [Prima](evidence/gnome-maps/previous.png) · [Dopo](evidence/gnome-maps/current.png) · [Log](evidence/gnome-maps/lifecycle.log)
- QGIS. QGIS Untitled Project renderizzato nelle due revisioni, menu, strumenti, layer e sezione Templates leggibili. In isolamento compare l’avviso “Open Sans font installation failed”. Nessuna mappa esterna, layer privato, account o plugin aperti. [Prima](evidence/qgis/previous.png) · [Dopo](evidence/qgis/current.png) · [Log](evidence/qgis/lifecycle.log)
- Logseq. Letti entrambi i PNG: editor locale, toolbar e grafo dimostrativo. La nuova revisione mostra il tutorial nel grafo. [Prima](evidence/logseq/previous.png) · [Dopo](evidence/logseq/current.png) · [Log](evidence/logseq/lifecycle.log)
- Anki. Selettore di lingua di Anki leggibile al primo avvio delle due revisioni, con elenco e conferma. Nessuna sincronizzazione, mazzo o account toccati. [Prima](evidence/anki/previous.png) · [Dopo](evidence/anki/current.png) · [Log](evidence/anki/lifecycle.log)
- LeoCAD. Letti entrambi i PNG: editor New Model.ldr, griglia 3D, catalogo dei pezzi e palette dei colori, senza finestre modali. [Prima](evidence/leocad/previous.png) · [Dopo](evidence/leocad/current.png) · [Log](evidence/leocad/lifecycle.log)

Il runner è compilato da `6d1506439ee95a2be90d15b73480fe5910135df1` più la matrice di prova. Le sette ricette sono in [flathub.rs](../../../crates/core/src/flathub.rs). Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `22694085082ae3c449b6a96f29cde124234eb54ce8a071772a0568c9b9a6beb4`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.Ny73CjmA`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Manifest SHA256](SHA256SUMS)
