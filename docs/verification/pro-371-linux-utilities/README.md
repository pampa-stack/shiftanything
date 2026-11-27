# PRO-371 · Secondo lotto desktop Flathub su Linux x86-64

Prova del 27 novembre 2026 su VPS Strumenti. Il runner usa `NativeBackend` da uno snapshot di Sh*ftAnything compilato offline su VPS1. Ogni app ha HOME e directory XDG, Flatpak per utente, DBus e Xvfb distinti. L'apertura avviene nel sandbox Flatpak con rete isolata, solo interfaccia `lo`, filesystem host azzerato e dispositivi disabilitati.

Le schede del catalogo puntano ai repository canonici e ai ref stable esatti. Gli editori risultano verificati da Flathub tramite il dominio dell'autore o il login GitHub, come registrato nelle istantanee API. Le icone del catalogo sono state controllate aprendo i PNG. [CherryTree](https://flathub.org/apps/net.giuspen.cherrytree) · [Foliate](https://flathub.org/apps/com.github.johnfactotum.Foliate) · [PDF4QT](https://flathub.org/apps/io.github.JakubMelka.Pdf4qt) · [GNOME Calendar](https://flathub.org/apps/org.gnome.Calendar)

I tempi misurati includono il runtime nel download iniziale. Il ciclo include la preparazione del ref storico; le app erano nuove per il profilo. I dati di prova sono un file sintetico controllato con SHA256. Il ref storico è distribuito da Flathub e predisposto con Flatpak CLI nel profilo di prova. Prima dell'associazione esplicita al launcher il motore riconosce la sostituzione esterna e la protegge. L'aggiornamento al digest corrente attraverso il launcher è l'azione misurata.

La prova riguarda gestione dei pacchetti e apertura grafica. Modifica del contenuto, sincronizzazione e clic Tauri sono fuori da questo banco.

## Versioni pubblicate

PDF4QT espone quattro componenti nel numero di versione. Il motore conserva l'etichetta completa e confronta i digest OSTree. L'etichetta di CherryTree 1.7.3 arriva da Flathub come `b'1.7.3'` ed è riportata così dal backend compilato; tale formattazione non cambia il digest e resta una correzione di visualizzazione. [Manifest CherryTree](https://github.com/flathub/net.giuspen.cherrytree) · [Manifest PDF4QT](https://github.com/flathub/io.github.JakubMelka.Pdf4qt)

## Risultati

Ciclo completato per tutte e quattro le app. Firme GPG valide per entrambe le revisioni con chiave primaria Flathub `4184DD4D907A7CAE`. Dati sintetici intatti e profili senza app né runtime dopo la rimozione.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| CherryTree 1.7.2 → 1.7.3 | 39,59 | 4,47 / 4,37 | 5,33 | 3,62 | 101,44 |
| Foliate 3.2.1 → 3.3.0 | 32,83 | 4,46 / 4,32 | 4,60 | 1,95 | 81,69 |
| PDF4QT 1.6.0.0 → 1.7.0.0 | 40,31 | 3,80 / 4,02 | 5,93 | 3,32 | 95,19 |
| GNOME Calendar 50.0 → 51.0 | 36,27 | 4,76 / 5,11 | 4,52 | 3,93 | 88,09 |

| App | Ref | Digest precedente | Digest corrente |
|---|---|---|---|
| CherryTree | `app/net.giuspen.cherrytree/x86_64/stable` | `146db7290d2b169b72cd27a1da7faa690bb02247e6dc244e9dc8b118ff3f0be6` | `306d32be9bdb60fea1a0a37e3aa36008690aebf634d84522924f0b82fb0d2f57` |
| Foliate | `app/com.github.johnfactotum.Foliate/x86_64/stable` | `f0ccefbb5b7b52e8e1ad7ab51db8ecc3e229a540cdfef8d2bb903762393630e6` | `4ca532b545422340d60651e8d4a912192dade5dc6f0af264f051215c5e6f489c` |
| PDF4QT | `app/io.github.JakubMelka.Pdf4qt/x86_64/stable` | `2e1b6551b978e4feb96fd54d4127615e730ff6d51407122f0fde63485c2d6ddd` | `74aa672411f5f82032b7bdbc5a3e771b15616de215d16ecd54f050dbe513eca1` |
| GNOME Calendar | `app/org.gnome.Calendar/x86_64/stable` | `f2e677fbdcc96a412ac5b2d0054efc60c1e654079a7bd242d20b65b5ae306826` | `54b27758494108a51e1747428909fe8337d21ddb923894b2ff3418b881de738a` |

Ho letto le otto schermate e le quattro icone esportate dai pacchetti.

- CherryTree. Editor e barra strumenti; riquadro nero centrale nella cattura Xvfb. [Prima](evidence/cherrytree/previous.png) · [Dopo](evidence/cherrytree/current.png) · [Log](evidence/cherrytree/lifecycle.log)
- Foliate. Libreria locale vuota con pulsante Apri. [Prima](evidence/foliate/previous.png) · [Dopo](evidence/foliate/current.png) · [Log](evidence/foliate/lifecycle.log)
- PDF4QT. Editor PDF con barra strumenti e nessun documento aperto. [Prima](evidence/pdf4qt/previous.png) · [Dopo](evidence/pdf4qt/current.png) · [Log](evidence/pdf4qt/lifecycle.log)
- GNOME Calendar. Griglia mensile del calendario locale. [Prima](evidence/gnome-calendar/previous.png) · [Dopo](evidence/gnome-calendar/current.png) · [Log](evidence/gnome-calendar/lifecycle.log)

Solo la revisione precedente di Foliate impiega GNOME 47 e Freedesktop 24.08, segnalati come fuori supporto e rimossi dopo la prova. La versione 3.3.0 testata usa GNOME 49, senza segnalazione di fine supporto nel remote al momento della verifica. [Storico Flatpak](history.log)

Il runner è compilato da `e01639825fbca3a8d24ff78a378e7f0d04eb4c63` più le quattro ricette proposte per questo lotto. Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `ec5453fc0fa06f659d66cb501a178394f12a9ddc43f9114403c536f665965bb3`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.KoR57Rrd`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Manifest SHA256](SHA256SUMS)
