# Lotto di verifiche per la beta 0.21

Obiettivo: portare `python3 packaging/release-notes.py HEAD --previous db42c75` da 14/20 a 20/20 o più. Il 28 novembre 2026 il conteggio su `e636831` è 14/20 (Mac con chip Apple +6, Linux x64 +8). Servono almeno 6 cicli chiusi in più, meglio 8.

Il manifest sale solo con la prova del ciclo nello stesso commit: log, PNG letti, hash. I cicli chiusi sono nella sezione [Cicli eseguiti](#cicli-eseguiti).

Le ricette esistono già come codice spento in `crates/core/src/pending.rs`, fuori da ogni tabella viva (`flathub::RECIPES`, elenco AppImage verificate, `COMMUNITY_RECIPES`, formati predefiniti di Linux). Un test fallisce se un candidato compare in una tabella viva o nel manifest Linux prima del tempo.

## Perché serve un lotto nuovo

Tutte le 217 ricette Flatpak del codice (`crates/core/src/flathub.rs`) sono già nel manifest Linux x64. Le schede non ancora verificate sono 316. Quelle con oltre 1.000 stelle e un pacchetto Linux ufficiale o Flathub sono nel file [lot.tsv](lot.tsv), con le stelle lette da `crates/core/curated.json`. Le altre voci Flathub incontrate non servono: Sioyek e Speech Note usano un runtime fuori supporto e restano con il link ufficiale, Sunshine è un servizio senza finestra, GanttProject ha un solo rilascio con AppImage e quindi non si può provare l'aggiornamento.

## Lotto scelto: 8 cicli

| Scheda | Pacchetto | Codice spento già scritto | Per accenderla, dopo il ciclo |
|---|---|---|---|
| Bruno, Cutter, Trayscale, Angry IP Scanner, OpenTabletDriver, OpenUtau | Flatpak su Flathub, firma Flathub | `pending::FLATPAK`: ID Flathub legato all'indirizzo del repository del catalogo | una riga in `RECIPES` e una in `approved_catalog_identity` di `flathub.rs`, via da `pending::FLATPAK`, riga in `candidates.tsv`, riga nel manifest |
| Umi-OCR | archivio ufficiale `.tar.xz` | `PackageKind::TarXz`, `install/archive_tree.rs`, ricevute 2.1.4 e 2.1.5, cancello `pending::umi_ocr_asset`, `curated::UMI_OCR` | ricetta in `COMMUNITY_RECIPES`, scelta del pacchetto in `platform.rs`, rilevamento della cartella installata, riga nel manifest |
| Termix | AppImage ufficiale 26.11.2 → 26.11.3 | ricevute e cancello `pending::termix_appimage_asset` | `TERMIX` in `qualified_linux_appimage_recipe` e il cancello in `qualified_linux_appimage_asset` di `release.rs`, riga nel manifest |

Riserve, nell'ordine: eSearch, OpenWhispr, Restic Browser. Dettagli di pacchetto, dimensioni e stelle sono in [lot.tsv](lot.tsv).

Note per i sei Flatpak:
- Le due revisioni firmate di ogni app si ricavano nel guest con `flatpak remote-info --log flathub <id>`; `run.sh` vuole due commit da 64 caratteri diversi tra loro.
- Bruno concede l'accesso alla cartella home: nel profilo QA va revocato e sostituito dalla sola cartella sintetica, come per calibre. Flathub indica la licenza come proprietaria, la scheda dice MIT: da confrontare con il file `LICENSE` del repository prima di scrivere il testo del README.
- Trayscale e OpenTabletDriver hanno bisogno di un servizio (Tailscale, demone della tavoletta) che nel guest manca: la prova copre l'apertura della finestra con il messaggio di servizio assente, non la funzione.

## Umi-OCR su Linux x64

Il 28 novembre ho scaricato l'archivio ufficiale della v2.1.5: 277.369.884 byte (uguali alla scheda), SHA-256 `7ff32e5dc818d67171820545100433c86e7e71155f9bad0851fad2ced9588f9b`. GitHub non pubblica un digest per questo file, quindi l'hash va fissato nella ricetta come per AiderDesk.

Il guest `shift-launcher-qa` non era raggiungibile da VPS2. Ho rifatto il preflight in un guest equivalente: contenitore Docker Ubuntu 24.04 senza rete, senza capability e con utente 1000 (non è il contenitore con permessi estesi che la Regia ha escluso). Dentro ci sono Xvfb, xauth, xdotool, x11-apps, x11-utils e xfwm4, gli stessi pacchetti grafici del guest QA; Flatpak e OSTree non servono per questo archivio. Il [preflight](umi-ocr/preflight.sh) usa [Dockerfile.base](umi-ocr/Dockerfile.base) e [Dockerfile.xcb](umi-ocr/Dockerfile.xcb).

- `ldd` su `UmiOCR-data/.embeddable/lib/python3.10/site-packages/PySide2/Qt/plugins/platforms/libqxcb.so` segnala 7 librerie mancanti: `libxcb-icccm.so.4`, `libxcb-image.so.0`, `libxcb-keysyms.so.1`, `libxcb-render-util.so.0`, `libxcb-xinerama.so.0`, `libxcb-xkb.so.1`, `libxkbcommon-x11.so.0`. [Log](umi-ocr/ldd.log)
- I pacchetti Ubuntu 24.04 che le portano sono `libxcb-icccm4`, `libxcb-image0`, `libxcb-keysyms1`, `libxcb-render-util0`, `libxcb-xinerama0`, `libxcb-xkb1` e `libxkbcommon-x11-0` ([versioni](umi-ocr/packages.txt)). Con questi `ldd` non segnala più nulla.
- Con i 7 pacchetti la finestra Umi-OCR v2.1.5 compare dopo 3 secondi sotto Xvfb con xfwm4. Il PNG letto mostra la schermata di benvenuto con «Current version 2.1.5». [PNG](umi-ocr/preflight-window.png) · [Log di apertura](umi-ocr/open-window.log)
- Il programma apre anche un server locale su `127.0.0.1:1224` e scrive `UmiOCR-data/.settings`, `.pre_settings` e `logs/` dentro la cartella di installazione.

Questo non è il ciclo: manca installare con il launcher, aggiornare, riaprire e rimuovere.

### Ricetta (spenta fino al ciclo)

Scritta il 28 novembre, con 7 test in `archive_tree.rs` e 6 in `pending.rs`:

- `PackageKind::TarXz` per Linux. L'installer chiama `xz -dc` e legge il flusso con la libreria `tar` già presente, quindi nessuna dipendenza nuova.
- `archive_tree::install` accetta solo file, cartelle e link verso un nome vicino; rifiuta `..`, percorsi assoluti, hard link, dispositivi, più cartelle in cima e uno script d'avvio mancante o che è un link. Limiti: 50.000 voci e 2 GiB (l'archivio Umi-OCR ha 6.007 voci e 1,1 GB). Un download troncato o rifiutato lascia intatta la copia installata.
- L'aggiornamento porta nella copia nuova `UmiOCR-data/.settings` e `.pre_settings`; la rimozione cancella tutto il resto e lascia quei due file.
- Il cancello prima del download (`install_at_with_profile`) lascia passare solo l'archivio Umi-OCR con dimensione e SHA-256 delle ricevute, su Linux x86-64.
- Avvio: lo script `umi-ocr.sh` nella cartella estratta, con `DISPLAY` impostato. Servono i 7 pacchetti Ubuntu elencati sopra (`pending::UMI_OCR_UBUNTU_PACKAGES`).
- Archivi con ricevuta: v2.1.4 (277.414.412 byte, SHA-256 `0dae3a93eb4fc8da23097944f403d143430ec27a91ce3fa563ac5d363b69246e`) e v2.1.5. Le versioni dalla 2.1.3 in giù non hanno un archivio Linux.

Manca per accenderla: la scelta del pacchetto (nessun asset diventa `TarXz` oggi), il rilevamento della cartella installata e la voce di menu, che il ciclo reale deve provare.

## Come si eseguono i cicli

1. Guest Ubuntu 24.04 x64 come in [after210](../pro-371-linux-after210/README.md#riproduzione). Almeno 9 GiB liberi nel profilo.
2. Flatpak: aggiungere le sei righe a una copia di `candidates.tsv` con i due commit, poi `dbus-run-session -- bash ~/after210-run.sh <slug> <display>`.
3. Termix e Umi-OCR: stesso schema del ciclo NAPS2 in [naps2/run.sh](../pro-371-linux-after210/naps2/run.sh).
4. Per ogni ciclo: log, PNG prima e dopo (letti), hash, file sintetico conservato.
5. Solo dopo, nello stesso commit, la riga in `verified-platforms.json`.
6. Alla fine `python3 packaging/release-notes.py HEAD --previous db42c75` deve dare 20/20 o più.

## Stato dell'ambiente

VPS2 non ha Flatpak, sudo né namespace utente, il guest QA vive sull'AX42 e l'Air rifiuta il login da qui. I cicli aspettano l'accesso a quegli host.

Controlli su VPS2 con il codice spento: `cargo fmt --all --check` verde; `cargo clippy -D warnings` su core e native verde, dopo la correzione del lint `byte_char_slices` in `flatpak.rs`; test di core 187 passati e 1 ignorato, di native 653 passati e 4 ignorati (gli ignorati c'erano già). `tauri-proof` non si compila su VPS2 perché mancano le librerie di sviluppo GTK e non c'è sudo: va controllato su una macchina che le ha.

## Cicli eseguiti

Guest Ubuntu 24.04 x64 usa e getta su AX42 (`/var/tmp/shift-beta021-guest.*`, 8 vCPU, 12 GiB), creato con lo stesso schema di after210. Il backend è quello di questo ramo con la ricetta accesa, compilato nel guest. Le aperture usano il solo loopback.

### Bruno su Linux x64

Il 28 novembre NativeBackend ha installato Bruno da Flathub, portato il pacchetto alla revisione firmata `acc13d37…`, aggiornato alla revisione firmata `1bca9c49…` e rimosso l'app. Le due revisioni sono entrambe v4.3.0: la più recente è la ricostruzione che annulla la v4.4.0 ritirata, quindi l'aggiornamento cambia il pacchetto e non il numero di versione. Il file sintetico è rimasto e la voce di menu è sparita. Il profilo QA toglie a Bruno l'accesso alla cartella home (`!host:reset`): la collezione di esempio creata alla prima apertura sta in una home temporanea del sandbox, per questo dopo l'aggiornamento compare «Failed to open». La licenza del repository è MIT (`license.md`); Flathub indica «proprietaria» perché il pacchetto scarica il binario ufficiale come extra-data. [Flathub](https://flathub.org/apps/com.usebruno.Bruno) · [Prima](bruno/previous.png) · [Dopo](bruno/current.png) · [Log](bruno/lifecycle.log)

### Cutter 2.4.1 → 2.5.0 su Linux x64

Il 28 novembre NativeBackend ha installato Cutter da Flathub, portato il pacchetto alla revisione firmata 2.4.1 (runtime KDE 6.9), aggiornato alla 2.5.0 (runtime KDE 6.11), riaperto e rimosso l'app. Entrambe le revisioni hanno una firma Flathub valida. La 2.4.1 mostra la finestra di benvenuto con il numero di versione, la 2.5.0 la finestra «Open File». Il file sintetico è rimasto e la voce di menu è sparita. La prova non apre un binario da analizzare. [Flathub](https://flathub.org/apps/re.rizin.cutter) · [Prima](cutter/previous.png) · [Dopo](cutter/current.png) · [Log](cutter/lifecycle.log)

### Trayscale 0.20.0 → 0.21.0 su Linux x64

Il 28 novembre NativeBackend ha installato Trayscale da Flathub, portato il pacchetto alla revisione firmata v0.20.0, aggiornato alla v0.21.0, riaperto e rimosso l'app. Entrambe le revisioni hanno una firma Flathub valida e usano il runtime GNOME 51. Nel guest non gira il servizio Tailscale: la v0.20.0 mostra l'elenco vuoto «No matching peers», la v0.21.0 il messaggio «Tailscale is not connected». La prova copre il ciclo del pacchetto e l'apertura della finestra, non la connessione. Il file sintetico è rimasto e la voce di menu è sparita. [Flathub](https://flathub.org/apps/dev.deedles.Trayscale) · [Prima](trayscale/previous.png) · [Dopo](trayscale/current.png) · [Log](trayscale/lifecycle.log)

### OpenTabletDriver 0.6.7 → 0.6.8.1 su Linux x64

Il 28 novembre NativeBackend ha installato OpenTabletDriver da Flathub, portato il pacchetto alla revisione firmata v0.6.7, aggiornato alla v0.6.8.1, riaperto e rimosso l'app. Entrambe le revisioni hanno una firma Flathub valida e usano il runtime Freedesktop 26.08. Il titolo della finestra riporta la versione e sopra compare la guida iniziale. Nel guest non ci sono tavolette né il demone del driver: la prova copre il ciclo del pacchetto e l'apertura dell'interfaccia, non il driver. Il primo giro senza compositing ha chiuso il ciclo con l'interno della finestra nero nella cattura, quindi la prova registrata usa `SHIFT_QA_COMPOSITOR=on dbus-run-session -- bash ~/after210-run.sh opentabletdriver 91`. Il file sintetico è rimasto e la voce di menu è sparita. [Flathub](https://flathub.org/apps/net.opentabletdriver.OpenTabletDriver) · [Prima](opentabletdriver/previous.png) · [Dopo](opentabletdriver/current.png) · [Log](opentabletdriver/lifecycle.log)

### Angry IP Scanner 3.9.3 → 3.10.0 su Linux x64

Il 28 novembre NativeBackend ha installato Angry IP Scanner da Flathub, portato il pacchetto alla revisione firmata 3.9.3 (runtime Freedesktop 25.08), aggiornato alla 3.10.0 (runtime 26.08), riaperto e rimosso l'app. Entrambe le revisioni hanno una firma Flathub valida. In tutte e due le aperture compare la finestra principale «IP Range - Angry IP Scanner». Il sandbox di prova ha solo l'interfaccia di loopback: la 3.9.3 lo segnala con un avviso `NullPointerException` su `localInterface`, la 3.10.0 apre la finestra «Getting Started», che la cattura con compositing disegna in modo incompleto. Nessuna scansione è stata avviata. La prova registrata usa `SHIFT_QA_COMPOSITOR=on`; il primo giro senza compositing aveva chiuso il ciclo con le finestre di dialogo nere nella cattura. Il file sintetico è rimasto e la voce di menu è sparita. [Flathub](https://flathub.org/apps/org.angryip.ipscan) · [Prima](angryip/previous.png) · [Dopo](angryip/current.png) · [Log](angryip/lifecycle.log)
