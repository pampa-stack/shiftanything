# PRO-371 · Sesto lotto desktop Flathub su Linux x86-64

Prove del 27 novembre 2026 su VPS Strumenti, con runner NativeBackend compilato da snapshot su VPS1 e doppia corsia. Ogni app usa installazione Flatpak separata, Xvfb e D-Bus privati. Rete di apertura limitata al loopback, file dell'host, periferiche e audio esclusi.

Publisher verificati e distribuzione upstream verificata. [GnuCash](https://flathub.org/apps/org.gnucash.GnuCash) · [Fractal](https://flathub.org/apps/org.gnome.Fractal) · [Subtitle Composer](https://flathub.org/apps/org.kde.subtitlecomposer) · [Qalculate!](https://flathub.org/apps/io.github.Qalculate) · [Shotwell](https://flathub.org/apps/org.gnome.Shotwell) · [Remmina](https://flathub.org/apps/org.remmina.Remmina) · [LabPlot](https://flathub.org/apps/org.kde.labplot) · [Upscayl](https://flathub.org/apps/org.upscayl.Upscayl)

Il launcher installa la revisione corrente e il runtime ufficiale. Il runner mette la revisione storica nel profilo sintetico e verifica che il launcher protegga una copia modificata all'esterno. Poi rende la revisione storica gestita nel solo profilo di prova. Il motore apre l'app, aggiorna al digest corrente, riapre e rimuove. Nessun account, collegamento a rete o clic nella UI Tauri.

## Risultati

Sette cicli tecnici completati e sei ricette abilitate. GnuCash rimane fuori dalle ricette perché la cattura ha un riquadro nero sovrapposto. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| GnuCash 5.16+ (Flathub 5.16) → 5.17+ (Flathub 5.17) | 44,90 | 6,57 / 6,40 | 7,03 | 4,02 | 92,31 |
| Subtitle Composer 0.8.2 → 0.8.2 | 50,02 | 5,59 / 5,81 | 5,64 | 4,53 | 107,00 |
| Qalculate! (GTK) 5.13.0 → 5.13.0 | 38,98 | 5,59 / 5,39 | 5,03 | 3,72 | 66,36 |
| Shotwell 33.2 → 33.2 | 37,83 | 6,55 / 6,20 | 5,53 | 3,82 | 94,28 |
| Remmina 1.4.44 → 1.4.44 | 38,92 | 6,01 / 5,83 | 4,72 | 3,72 | 68,24 |
| Upscayl 2.15.0 → 2.15.0 | 34,90 | 14,60 / 14,28 | 4,62 | 3,92 | 103,91 |
| LabPlot 2.12.1 → 2.12.1 | 51,98 | 6,25 / 6,31 | 6,18 | 4,12 | 87,61 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| GnuCash | `app/org.gnucash.GnuCash/x86_64/stable` | `ef3f97227486c297ce4e89b21e22122342a544ee1660d43df7625727790b7edf` | `1ae4ecbe79a6a4f4b3f8911697a3712c7a5c23cf6e34076d9c15424607d7a67e` |
| Subtitle Composer | `app/org.kde.subtitlecomposer/x86_64/stable` | `90a931ec31812043f9bbc53d60da5679580414d951548bb7238c017f39affbe7` | `1d3c7467ecdd649f676b8c456504d50c6fd002687fb1f4012f5dad9302f753da` |
| Qalculate! (GTK) | `app/io.github.Qalculate/x86_64/stable` | `c8a7f26953e0a694bda83688dcad1c44e0a3f05320e224f4f5f6ea06c0014939` | `dcc78d6f2f03d1f759f191f3ca89eb6e9925a23203dda7038faacd338f4035b1` |
| Shotwell | `app/org.gnome.Shotwell/x86_64/stable` | `858716d9d4ba0893716912ed5ce68bd5e965c4a560211eb380f4122be0ed13c4` | `8aa679f02c0e7cc54d67379931cff5b01be461debda3ff53ae82fb30650590b6` |
| Remmina | `app/org.remmina.Remmina/x86_64/stable` | `86904a724584da7be77e284e6ac382710f14683519a3d961aec8845c335784ce` | `b2a4f56c9b3ee7020266692e4fee0cf600ce5bc6c66acb65307a5c3033f7efc2` |
| Upscayl | `app/org.upscayl.Upscayl/x86_64/stable` | `5a98b1f5e22402daae2e4a1faf63f73f37916187d076a567251c37fc6c318bf3` | `e67f8eeb6a09c1ad6f04dee0477101be895519b63c1575e730f6ac89d5cbdc19` |
| LabPlot | `app/org.kde.labplot/x86_64/stable` | `77d659ccef137da679c1027763e74c0179d01ad1e08ba7e49ef5c22d21074391` | `905ae1c5d5e68bdd7a60b78b81708e4a94233893c65e1f41b925e8a0f70f0fd0` |

Le catture mostrano l’apertura delle app nelle due revisioni.

- GnuCash. Finestra dell'app e pulsanti iniziali dei consigli visibili su entrambe le revisioni. Un riquadro sovrapposto appare nero nella cattura Xvfb. Nessun dato contabile inserito. [Prima](evidence/gnucash/previous.png) · [Dopo](evidence/gnucash/current.png) · [Log](evidence/gnucash/lifecycle.log)
- Subtitle Composer. Editor dei sottotitoli con menu, pannello video vuoto, lista righe, timeline e pulsante Open Subtitle su entrambe le revisioni. [Prima](evidence/subtitlecomposer/previous.png) · [Dopo](evidence/subtitlecomposer/current.png) · [Log](evidence/subtitlecomposer/lifecycle.log)
- Qalculate! (GTK). Interfaccia della calcolatrice visibile con campo espressione, tastiera scientifica, cronologia e conversioni in entrambe le revisioni. [Prima](evidence/qalculate/previous.png) · [Dopo](evidence/qalculate/current.png) · [Log](evidence/qalculate/lifecycle.log)
- Shotwell. Finestra Libreria con barra dei menu, fototeca vuota e comandi Find in entrambe le revisioni. Aggiornamento del pacchetto e del runtime tra revisioni firmate con la stessa etichetta 33.2. [Prima](evidence/shotwell/previous.png) · [Dopo](evidence/shotwell/current.png) · [Log](evidence/shotwell/lifecycle.log)
- Remmina. Finestra con campo indirizzo, selettore protocollo e lista connessioni vuota su entrambe le revisioni; nessun collegamento effettuato. Il pacchetto cambia digest senza cambiare la versione 1.4.44. [Prima](evidence/remmina/previous.png) · [Dopo](evidence/remmina/current.png) · [Log](evidence/remmina/lifecycle.log)
- Upscayl. GUI locale con Welcome to Upscayl, selettore lingua e pulsante Next su entrambe le revisioni. Nessun account né elaborazione di immagini. Stessa versione applicativa 2.15.0 con nuovo digest di distribuzione. [Prima](evidence/upscayl/previous.png) · [Dopo](evidence/upscayl/current.png) · [Log](evidence/upscayl/lifecycle.log)
- LabPlot. Progetto nuovo, tabella a due colonne, navigatore del progetto e pannello proprietà su entrambe le revisioni. [Prima](evidence/labplot/previous.png) · [Dopo](evidence/labplot/current.png) · [Log](evidence/labplot/lifecycle.log)

Fractal completa i comandi di gestione. Lo screenshot di entrambe le versioni mostra Secret Portal Error nel D-Bus isolato. Nessun portachiavi personale usato. La ricetta resta fuori dalla patch. [Prima](evidence/fractal/previous.png) · [Dopo](evidence/fractal/current.png)


Runner di prova basato su `322225d11a9b522601502a17597a4d4fa0e0d411` con la matrice dei candidati. [Test Rust](core-test.log) · [Metadati, digest e tempi](verified-proofs.json)
