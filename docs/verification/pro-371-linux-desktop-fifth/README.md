# PRO-371 · Quinto lotto desktop Flathub su Linux x86-64

Prove del 27 novembre 2026 su VPS Strumenti in due corsie. Il runner chiama NativeBackend dallo snapshot compilato offline su VPS1. Ogni app usa installazione Flatpak separata, Xvfb e sessione D-Bus privati. All'apertura il namespace di rete espone solo il loopback. File dell'host, periferiche e audio sono esclusi.

Corrispondenza tra sorgente della scheda, publisher verificato e due revisioni stabili del ref Flatpak. [Firefox](https://flathub.org/apps/org.mozilla.firefox) · [GCompris](https://flathub.org/apps/org.kde.gcompris) · [Document Scanner](https://flathub.org/apps/org.gnome.SimpleScan) · [Clocks](https://flathub.org/apps/org.gnome.clocks) · [RapidRAW](https://flathub.org/apps/io.github.CyberTimon.RapidRAW) · [Apostrophe](https://flathub.org/apps/org.gnome.gitlab.somas.Apostrophe) · [LibrePCB](https://flathub.org/apps/org.librepcb.LibrePCB) · [VSCodium](https://flathub.org/apps/com.vscodium.codium)

Il launcher installa la revisione corrente da profilo vuoto, con il runtime ufficiale. Il runner prepara la revisione storica via Flatpak e verifica che una copia modificata esternamente non sia aggiornabile o rimovibile. Poi registra la revisione storica nel solo profilo sintetico. Il motore apre l'app, aggiorna al digest corrente, la riapre e la rimuove. Nessun account, file personale o clic nella UI Tauri.

## Risultati

Ciclo completato per 5 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| GCompris 26.1 → 26.2 | 42,14 | 12,51 / 11,85 | 5,42 | 4,12 | 83,35 |
| Simple Scan 49.1 → 50.0 | 39,12 | 5,47 / 5,48 | 5,22 | 3,92 | 94,67 |
| GNOME Clocks 50.0 → 51.0 | 38,06 | 5,68 / 5,39 | 3,84 | 3,20 | 91,58 |
| Apostrophe 3.4 → 3.5 | 38,40 | 6,12 / 7,38 | 9,54 | 5,12 | 107,19 |
| LibrePCB 2.1.0 → 2.1.1 | 40,85 | 5,60 / 5,55 | 7,04 | 4,03 | 82,14 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| GCompris | `app/org.kde.gcompris/x86_64/stable` | `e605435a769c79197a5bae4f457720e3189ae655081f8894efca389453e6ae8d` | `312e00eb0afe2cca5073759693de470eadf0e39b448c9cded715ee7269ba1e08` |
| Simple Scan | `app/org.gnome.SimpleScan/x86_64/stable` | `11dcd294c0a6ed800d0b21bffbbe3f8030bcf4a7165051610ee421b6c44363fe` | `03a3320a68af5423ec6e18967906f8e7942057e159f554ced7e5d065c18b0d75` |
| GNOME Clocks | `app/org.gnome.clocks/x86_64/stable` | `3acf5d86824f9fca00a06546a62bdc2968a6d133603bfbfab12420df868b0809` | `4facc20158d901c62d8e68b2cbf1b52c0577c2497c348eb1512c9d7cfee0a519` |
| Apostrophe | `app/org.gnome.gitlab.somas.Apostrophe/x86_64/stable` | `522ff389b5bf48c4fc4f825ba1c9a7cab797d23f851e2d777db0db9339b1817c` | `eebf714c88801c14b0c5bd343088f8a6cbadad29de22c61d345a7a47c795917a` |
| LibrePCB | `app/org.librepcb.LibrePCB/x86_64/stable` | `e69ff1edbe92820e4c06cd2eb74fcc9c914647d1335ae4854227cf0fb206cb15` | `5936072c96166a1c5b05d1a45c2ee8eaf24e7e19e4628ec813e11736b3b86b57` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- GCompris. Menu principale con categorie delle attività e area Preferiti su entrambe le versioni. La cattura della prima prova sulla versione storica arrivava al logo iniziale. Il secondo ciclo ha atteso la GUI dopo il logo. [Prima](evidence/gcompris/previous.png) · [Dopo](evidence/gcompris/current.png) · [Log](evidence/gcompris/lifecycle.log)
- Simple Scan. Finestra di Document Scanner con comandi e ricerca scanner su entrambe le revisioni. Nessuna periferica collegata nel sandbox. [Prima](evidence/simple-scan/previous.png) · [Dopo](evidence/simple-scan/current.png) · [Log](evidence/simple-scan/lifecycle.log)
- GNOME Clocks. Interfaccia con World, Alarms, Stopwatch e Timer e pulsante Add World Clock su entrambe le revisioni. Nessun allarme pianificato. [Prima](evidence/gnome-clocks/previous.png) · [Dopo](evidence/gnome-clocks/current.png) · [Log](evidence/gnome-clocks/lifecycle.log)
- Apostrophe. Editor Markdown aperto su Nuovo file, con pulsanti Open e Save e contatore parole, su entrambe le revisioni. [Prima](evidence/apostrophe/previous.png) · [Dopo](evidence/apostrophe/current.png) · [Log](evidence/apostrophe/lifecycle.log)
- LibrePCB. Procedura iniziale Welcome to LibrePCB con apertura o creazione del workspace, pulsanti Next e Cancel su entrambe le versioni. [Prima](evidence/librepcb/previous.png) · [Dopo](evidence/librepcb/current.png) · [Log](evidence/librepcb/lifecycle.log)

Tre pacchetti hanno completato il ciclo del motore senza evidenza grafica sufficiente. Firefox è rimasto alla schermata di avvio con il portale Desktop assente nel banco. RapidRAW ha mostrato una finestra nera e registrato errori di inizializzazione Clerk/Vite con rete isolata. VSCodium ha aperto il telaio della finestra senza mostrare l'editor. I tre ref restano fuori dalla patch. [Firefox](evidence/firefox/current.png) · [RapidRAW](evidence/rapidraw/current.png) · [VSCodium](evidence/vscodium/current.png)

Il runner è compilato da `e52f1e3ab4bdfc06232c3c23fd6677f6ab5886d8` più la matrice di prova. Le cinque ricette verificate sono nella [matrice Flathub](../../../crates/core/src/flathub.rs). Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `dd95f424cd18549bcd94da8a62084b0d464155daeb03d5327cc6edcd4b19a70f`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.4s4sWfKW`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Storico Flatpak](history.log) · [Manifest SHA256](SHA256SUMS)
