# PRO-371 · Ottavo lotto desktop Flathub su Linux x86-64

Prova NativeBackend del 27 novembre 2026. Ogni app usa un profilo Flatpak per utente in una directory temporanea, con Xvfb e D-Bus privati e rete confinata al loopback. Nessun account, file o periferica personale.

Canale di publisher verificato e licenza libera. [vokoscreenNG](https://flathub.org/apps/com.github.vkohaupt.vokoscreenNG) · [Icon Library](https://flathub.org/apps/org.gnome.design.IconLibrary) · [RSS Guard](https://flathub.org/apps/io.github.martinrotter.rssguard) · [GNOME Builder](https://flathub.org/apps/org.gnome.Builder) · [Friction](https://flathub.org/apps/graphics.friction.Friction) · [Zettlr](https://flathub.org/apps/com.zettlr.Zettlr) · [Trilium Notes](https://flathub.org/apps/org.triliumnotes.Trilium) · [digiKam](https://flathub.org/apps/org.kde.digikam).

Installazione, apertura, aggiornamento e rimozione tramite NativeBackend. Revisione precedente e dati sintetici preparati in scratch dopo il rifiuto del motore di manomettere la medesima app registrata come esterna. Prova della GUI dell'app avviata dal motore. L'interfaccia del launcher Tauri è verificata in una prova separata.

Il ramo abilita Builder, digiKam, Icon Library, Trilium Notes, Friction e vokoscreenNG. Zettlr conserva il canale AppImage già verificato, perché il motore sceglie un solo canale per ID. RSS Guard attende una seconda cattura senza il dialogo iniziale nero. Qui restano le prove di entrambi.

## Risultati

Ciclo completato per 8 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| Builder 50.0 → 50.0 | 58,58 | 11,86 / 11,18 | 7,14 | 5,12 | 104,83 |
| digiKam 9.1.0 → 9.1.0 | 56,89 | 16,07 / 15,46 | 10,23 | 4,62 | 125,58 |
| Icon Library 0.0.22 → 0.0.23 | 36,42 | 6,42 / 6,11 | 6,23 | 4,42 | 98,64 |
| RSS Guard 5.3.0 → 5.3.0 | 48,78 | 8,76 / 7,45 | 5,32 | 4,22 | 273,83 |
| Trilium Notes 0.106.0 → 0.107.0 | 40,34 | 17,11 / 16,33 | 10,74 | 4,32 | 124,03 |
| Zettlr 4.9.0 → 4.10.0 | 37,78 | 17,17 / 17,00 | 9,84 | 4,52 | 103,97 |
| Friction 1.0.0 → 1.0.0 | 39,25 | 11,21 / 10,46 | 5,63 | 4,83 | 88,03 |
| vokoscreenNG 4.11.1 → 4.11.0 | 52,44 | 7,25 / 6,60 | 5,92 | 4,53 | 93,75 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| Builder | `app/org.gnome.Builder/x86_64/stable` | `ec7ab5aeba400b5b934870d753a94608af444067a503e5977ec98b76134a26e1` | `1aad58b38f886f74f29669c2efe515e8be42eb9ab3cb26cf6318b37f39c62330` |
| digiKam | `app/org.kde.digikam/x86_64/stable` | `63eb4f5143296aa6f1a9eb00481ef747e6d39c734d72c23a0816c45cdadb9d0e` | `7a1f805ba8808c95f028abd6ac357d5a445bf0140bf627c1569535a153da79fd` |
| Icon Library | `app/org.gnome.design.IconLibrary/x86_64/stable` | `06df08ea92916cb5657a1171a895812ed80a273cdec1ead84ac4908b1c35e157` | `0f9019b3589db1ee56f7c2200ec7fa001c2327afec924fb90c1cd6b361635d25` |
| RSS Guard | `app/io.github.martinrotter.rssguard/x86_64/stable` | `198a09f141e1ecf7a4e0df0b0d1294d672074b0bd02c27e2838ae631df9687f8` | `69e101bb6d83941e80cfa00e00aa581bb98da20666affc6a3bb1c7dd6e2ce6b2` |
| Trilium Notes | `app/org.triliumnotes.Trilium/x86_64/stable` | `cfdea575049f1b008e6896bc8c79adb866321f1c28a464819f53abfc83e5023c` | `3072536765c212433f7c7768f382db9cfde968d0a4ac80c265f0bfb32d517cf2` |
| Zettlr | `app/com.zettlr.Zettlr/x86_64/stable` | `866875b85c109ca258c051589abb456a31ab71b92f551ed9b62225403558efdf` | `bb96fc22a7bcbdcce677bfa96ad66f12c1c936cd6bea1ece50aef3d8f50a87b7` |
| Friction | `app/graphics.friction.Friction/x86_64/stable` | `eac4e5a6b0389c377624cb83eed914b5a97298a2bade1ba02345ef2ab7039b58` | `68c8a696e44a7e255ae2a3445b8824520f81cc2d571254de1a2d387d8fa8dce0` |
| vokoscreenNG | `app/com.github.vkohaupt.vokoscreenNG/x86_64/stable` | `5e75f22cb40d4cf01c847cf5a71d2c8f130a62b96020cdbb8a0a458e3dac995c` | `6eeef2c179e8683ec835791b4d628e54846ed47de22016ebae837987d59814c8` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- Builder. La finestra Open a Project mostra ricerca, lista dei progetti GNOME e comandi Create New Project, Select a Folder e Clone Repository in entrambe le revisioni. Nessun progetto aperto o compilato. [Prima](evidence/builder/previous.png) · [Dopo](evidence/builder/current.png) · [Log](evidence/builder/lifecycle.log)
- digiKam. Wizard Welcome to digiKam 9.1.0 con navigazione e controlli visibili in entrambe le revisioni. Nessuna foto importata. [Prima](evidence/digikam/previous.png) · [Dopo](evidence/digikam/current.png) · [Log](evidence/digikam/lifecycle.log)
- Icon Library. Griglia delle icone e campo di ricerca leggibili per entrambe le revisioni. [Prima](evidence/icon-library/previous.png) · [Dopo](evidence/icon-library/current.png) · [Log](evidence/icon-library/lifecycle.log)
- RSS Guard. Finestra RSS Guard 5.3.0 con menu, azioni feed, ricerca e lista articoli leggibili in entrambe le revisioni. La finestra aggiunta account resta nera su Xvfb e copre il centro. Nessun account o abbonamento configurato. Lo storico Flathub fornisce due commit distinti della stessa versione. L’icona 512x512 è il file originale esportato dal Flatpak firmato. Tempo installazione stimato dal timestamp del log nativo perché il primo passaggio del banco si è fermato cercando un’altra dimensione dell’icona. [Prima](evidence/rssguard/previous.png) · [Dopo](evidence/rssguard/current.png) · [Log](evidence/rssguard/lifecycle.log)
- Trilium Notes. Schermata Getting Started con lista lingue e Continue in entrambe le revisioni. Nessun account associato e nessuna nota personale. [Prima](evidence/trilium/previous.png) · [Dopo](evidence/trilium/current.png) · [Log](evidence/trilium/lifecycle.log)
- Zettlr. Schermata di ingresso con Skip e Start setup nello storico. App aggiornata mostra Update complete, versione 4.10.0 e pulsante Get started. Entrambe le finestre sono avviate dal motore. [Prima](evidence/zettlr/previous.png) · [Dopo](evidence/zettlr/current.png) · [Log](evidence/zettlr/lifecycle.log)
- Friction. Quick Setup aperto con risoluzione, fotogrammi al secondo, durata e controlli di impostazione per entrambe le revisioni. Nessun progetto importato. [Prima](evidence/friction/previous.png) · [Dopo](evidence/friction/current.png) · [Log](evidence/friction/lifecycle.log)
- vokoscreenNG. Finestra con sorgenti schermo, finestra o area, impostazioni codec e comandi Start/Stop per entrambe le revisioni. Nel banco PulseAudio è escluso, nessuna registrazione avviata e nessuna periferica collegata. Entrambe le GUI riportano 4.11.0. L’ultima revisione Flathub corregge la vecchia etichetta AppStream 4.11.1 in 4.11.0. [Correzione del publisher](https://github.com/flathub/com.github.vkohaupt.vokoscreenNG/pull/190/files) [Prima](evidence/vokoscreenng/previous.png) · [Dopo](evidence/vokoscreenng/current.png) · [Log](evidence/vokoscreenng/lifecycle.log)

Il runner è compilato da `d60eca237c0ea60c1abc979d21c5fa357cf614ee` più la matrice di prova. La matrice è stata controllata contro `ac1bc00a9b4928e8afd2d70e0b753d9a0edb9ac1`; le sei ricette abilitate sono elencate in apertura. Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `ab690bb548112d84bc0a974c82795a6ebe01063a5348050b77938f6c8c082bcb`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.eAG7lgtR`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Storico Flatpak](history.log) · [Manifest SHA256](SHA256SUMS)
