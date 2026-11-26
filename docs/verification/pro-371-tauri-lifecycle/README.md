# Ciclo nativo nella finestra Tauri — 27 novembre 2026

Prova macOS Apple Silicon in profili temporanei, con clic background Orca nella finestra reale. Nessuna configurazione client, installazione personale o istanza Handy esistente modificata. Il backend, il download, la verifica SHA-256/firma, il rilevamento e la rimozione sono le primitive Rust del prodotto.

## Eseguito

- Clic Installa → Rifiuta: `ui-1=rejected`, nessun job nativo.
- Installa → Approva: primo tentativo fallito onestamente per cartella Applications assente. Preparazione corretta nel codice. Tentativo successivo: DMG ufficiale Handy 0.10.2, installazione completata, rilevamento versione 0.10.2, pulsante Apri. [Stato](installed-state.json), [finestra 1440×900](installed-it.png), [albero accessibile](installed-ax.txt).
- Apri su Handy installata: rifiutato prima del comando Launch perché esiste il socket single-instance personale. Il messaggio compare nel footer; non è una prova di apertura riuscita.
- Scheda → Rimuovi → Approva: completata, copia nel profilo assente, rilevamento non installata, file sintetico preservato. [Stato](removed-state.json), [log con errore iniziale e recupero](native-cycle.log).
- Nuovo profilo con precedente DMG ufficiale 0.10.1: digest esatto, firma e versione verificati dalle primitive native; nessuna apertura. NativeBackend rileva 0.10.1 e aggiornamento 0.10.2 disponibile.
- Client stdio MCP reale `request_action(update)` → finestra Tauri → clic Approva → `InstallExpected(installed=true)` → download e aggiornamento ufficiale 0.10.2 completati. NativeBackend rileva 0.10.2, aggiornamento non più disponibile. Dato sintetico `synthetic-data-keep` preservato. [Consenso 1440×900](update-approval-it.png), [AX](update-approval-ax.txt), [stato](updated-state.json), [log MCP](mcp-update.log).
- Dopo update: lo stesso client rifiuta Install su già installata e Update/UpdateAll senza nuova versione. Remove MCP → consenso nella finestra → rimozione completata, app assente e dato sintetico preservato. [Esiti MCP](mcp-remove.log), [stato finale](update-removed-state.json), [log nativo](native-update.log).
- MCP su app non installata: Open, Update, UpdateAll, Remove e Cancel rifiutati. App sconosciuta e task sconosciuto rifiutati. [Log](mcp-negative.log).

Il completamento è provato dagli esiti per app, separati da `approved; native operation requested (not completed)`. Nessuna approvazione automatica nel client di prova. Queue e validazione restano quelle del protocollo esistente.

## Difetto corretto durante la prova

GitHub API ha riportato quota residua zero. Il fallback quota-free raggiungeva il tag ufficiale Handy, ma, in assenza di `SHA256SUMS.txt`, produceva una release con il solo nome del manifest. La prima ipotesi sul feed suite era errata ed è stata ritirata.

La ricevuta già verificata del pacchetto è ora condivisa fra parser API e fallback: solo `cjpais/Handy`, `v0.10.2`, `Handy_0.10.2_aarch64.dmg`, 14.979.688 byte, SHA-256 `b8b893fbf9e8262abb0e363cb0ac6715cc69eac2fd6ffd8c0785278083a0e3ca`. La cache incompleta con il solo manifest non impedisce il recupero. Non si estende ad altre versioni, architetture o repository. Download e firma nativa rimangono obbligatori. [Release ufficiale](https://github.com/cjpais/Handy/releases/tag/v0.10.2). Credito: CJ Pais; Team ID della ricetta `UWFLB4GC25`.

## Comandi e controlli

Avvio e seed riproducibili in [README della prova](../../../apps/shiftanything-tauri-proof/README.md). Client riutilizzabile: `uv run --offline tools/verify_tauri_mcp.py PROFILO [--operation update|remove|install]`. Richiede la finestra attiva in background e una decisione esplicita nella sua UI.

Comandi eseguiti sul worktree Tauri, Cargo seriale, target condiviso autorizzato e due job:

```sh
cargo test -p artcraft-launcher-core -p artcraft-launcher-native -p shiftanything-tauri-proof --offline
cargo clippy -p artcraft-launcher-core -p artcraft-launcher-native -p shiftanything-tauri-proof --all-targets --offline -- -D warnings
cargo build -p shiftanything-tauri-proof --offline
python3 tools/verify_tauri_metrics.py /tmp/shift-tauri-update-20261127/visual-it.json
```

Esiti: core 116 PASS/1 ignorato preesistente; native 549 PASS/7 ignorati preesistenti; Tauri 1 PASS; clippy e build PASS. Una precedente compilazione del seed falliva perché usava un helper privato: sostituito con la validazione pubblica della factory nativa, poi tutti i controlli sopra passati. Log completi locali `/tmp/shift-tauri-lifecycle-{tests,clippy}.log`, `/tmp/shift-tauri-lifecycle-build-final.log`.

Geometria finale: 298 schede, tutte le righe 408.328×400 px, sei copertine/loghi reali, viewport 1440×900. [Misure](lifecycle-visual-it.json). Screenshot letti insieme agli alberi accessibili. Consenso ora centrato; il primo runtime lo collocava in alto a sinistra, corretto prima della prova update.

## Limiti aperti

Apertura ufficiale Handy non eseguita per conflitto con istanza personale. Annullamento di un download in corso, bulk update e transizioni UI EN non ancora provati in questo checkpoint. Le pagine oltre al catalogo/dettaglio e la persistenza Settings Rust dei preferiti non sono completate nella prova Tauri. Non è una dichiarazione di parità completa o di copertura di tutto il catalogo. Nessuna prova Windows/Linux del frontend e nessuna release pubblicata. Le quantità Regia 18 Mac ARM64/19 Linux x64/18 Windows x64 indicano pacchetti selezionabili, non cicli eseguiti qui. La revisione GUI/dashboard del catalogo attende i dati curati di Regia.
