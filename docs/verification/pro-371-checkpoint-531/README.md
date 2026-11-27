# Sh*ftAnything · checkpoint del 27 novembre 2026

Il catalogo contiene 531 app e dashboard. I pacchetti includono 506 loghi e 36 altri asset con provenienza registrata. Il filtro Installabili qui usa lo stato del motore nativo per sistema e architettura. SH*FTER confronta il catalogo con 13 profili documentati. Le coperture indicano le funzionalità documentate selezionate. [Fonti dei media](../../../apps/shiftanything-tauri-proof/media-provenance.json) · [Prova filtro](../pro-371-desktop-installable/README.md) · [Prova SH*FTER](../pro-371-shifter-full/README.md)

## Cicli reali con il motore nativo

- **macOS Apple Silicon** — 36 ID unici con installazione, aggiornamento e rimozione documentati. I controlli su dimensione, checksum, firma, notarizzazione e destinazione hanno operato sui pacchetti ufficiali. Le prove di apertura su Mac sono documentate per singola app. AppFlowy e Actual Budget hanno richiesto un retry di Gatekeeper. Il limite di valutazione per i due pacchetti sale a 180 s, con tutti i controlli mantenuti. [AppFlowy](../pro-371-macos-appflowy/README.md) · [Actual Budget](../pro-371-macos-actual/README.md)
- **Linux x86-64** — 116 app uniche con installazione, aggiornamento e rimozione documentati. Il conteggio include 81 app Flathub e 35 AppImage. Le ultime prove hanno usato firme Flathub o checksum ufficiali, profili temporanei, apertura in Xvfb, icona del pacchetto e verifica che i dati sintetici restassero dopo la rimozione. [Ultime sette Flathub](../pro-371-flathub-lot10/README.md) · [RustDesk](../pro-371-linux-rustdesk/README.md) · [ActivityWatch](../pro-371-linux-activitywatch/README.md)
- **Limiti emersi** — Mumble richiede un controllo delle impostazioni dopo l’aggiornamento. Il wizard PrusaSlicer copriva parte dell’editor nel banco. FamiStudio richiede un sequencer ALSA indisponibile nel banco. Xournal++ è disponibile su Linux e il pacchetto Mac ufficiale provato ha firma ad hoc e fallisce la verifica profonda. Queste quattro prove incomplete sono fuori dal conteggio delle rispettive piattaforme.

## Verifiche del launcher

Entrambi i pacchetti finali sono stati aperti in un profilo temporaneo, il DMG dalla copia verificata del bundle arm64 e il DEB dal binario estratto su Ubuntu x86-64. L'avvio standard macOS è stato esercitato anche con il DMG intermedio in una home temporanea. Entrambe le build finali usano `65689f6ba964ea80139acaf407e0ed6c37c875f3`. Il test MCP stdio copre 531 schede, 13 profili, filtri e scelte manuali, e usa fixture per coda, annullamento e stati negativi. Le prove precedenti con backend reale includono Handy con conferma dalla finestra e due versioni dei pacchetti ufficiali. [MCP e Handy](../pro-371-tauri-lifecycle/README.md) · [Aggiornamento multiplo](../pro-371-tauri-bulk-flathub/README.md) · [Annullamento](../pro-371-tauri-cancel-flathub/README.md)

Il controllo giornaliero dei rilasci è nel ramo di lavoro. Aggiorna versione e metadati quando le fonti corrispondono e produce proposte per le variazioni di funzionalità di SH*FTER. Le affermazioni nuove richiedono una fonte verificabile. [Workflow](../../../.github/workflows/catalog-release-watch.yml)

## Finestra desktop e pacchetti 0.20.3

- **macOS Apple Silicon** — catalogo, filtro Installabili qui, Photoshop e Loom in SH*FTER e ricerca di app per dettatura, salvataggio di PhotoCraft nei Preferiti e apertura di Kit di lavoro. La suite finale conta 716 test passati e 8 ignorati preesistenti. [Catalogo](macos-catalog-it.png) · [Filtro](macos-installable-it.png) · [Loom](macos-loom-it.png) · [Trova app](macos-trova-app-it.png) · [Preferiti](macos-favorite-it.png) · [Kit di lavoro](macos-kits-it.png)
- **Linux x86-64** — catalogo, filtro Installabili qui, ripristino filtri e Loom in SH*FTER. La suite finale conta 765 test passati e 5 ignorati preesistenti. Il banco DEB ha dato accesso alle release via rete e mostrato 31 app installabili in quel profilo. Le 81 prove Flathub sono state eseguite nei banchi che esponevano il relativo runtime. [Catalogo](linux-catalog-it.png) · [Filtro](linux-installable-it.png) · [Loom](linux-loom-it.png)
- Entrambi i cataloghi desktop misurano 531 schede di dimensioni uguali a 1440 × 900. La finestra mostra gli stessi punteggi Loom in entrambi i profili. Il contratto MCP verifica anche le fonti. Il filtro native-ready espone lo stato del profilo corrente. La [matrice delle ricette](recipe-matrix.json) riporta le configurazioni e gli esiti raccolti per architettura.
- `ShiftAnything-0.20.3-macos-arm64-tauri-local.dmg` — SHA-256 `e13e19059dd2eb226d687059ec01b60e6a9b7204ddae17993edfbd9e91c5ad30`. Verificati checksum del disco, architettura, versione, licenze e firma ad hoc profonda in lettura e dalla copia.
- `shiftanything_0.20.3_amd64-local.deb` — SHA-256 `1f129e668cc4e68f81e98a9e2ca0ec34500f784a87536d862583a3a1df2182da`. Verificati architettura, versione e avvio del binario estratto in Ubuntu e isolato con bubblewrap e Xvfb.
- Lo script di aggiornamento del catalogo ha passato 12 test. Il rendering del catalogo verifica titoli della barra in maiuscolo, dimensioni uniformi e caricamento dei media iniziali.

## Correzione successiva al pacchetto

Nella finestra macOS, il tasto Aggiornamenti lasciava visibile anche la pagina Kit di lavoro. La correzione chiude la pagina precedente quando si apre la gestione. Verificati tre passaggi in italiano e in inglese con Playwright sul frontend effettivo e il bridge Tauri simulato: Kit di lavoro e Trova app → Aggiornamenti, SH*FTER → Le mie app. I due pacchetti locali sopra precedono questa correzione.

## Limiti prima della release

Rimangono da verificare le altre schede, macOS Intel, Linux ARM64 e Windows. La macchina Windows è arrivata a Windows Setup. Il desktop e i cicli dei pacchetti Windows sono aperti. Rimangono anche la verifica pixel per pixel con Doop e la notarizzazione del DMG. AppFlowy su Linux ha richiesto libkeybinder nel banco isolato e il suo editor resta da verificare. La ricetta Linux resta fuori da questo checkpoint. La PR resta in bozza. [Windows](../pro-371-windows-setup/desktop/README.md) · [PR 4](https://github.com/pampa-stack/shiftanything/pull/4)
