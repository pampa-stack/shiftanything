# Sh*ftAnything: prova desktop Tauri separata

Proof reversibile richiesto esplicitamente da Pampa tramite Regia il 27 novembre 2026, dal checkpoint prodotto `7a2b6ce`. Eccezione deliberata alla raccomandazione Rust-only/egui di AGENTS.md: HTML/CSS/JavaScript locali nella finestra Tauri, motore Rust esistente. Il prodotto egui e PR3 restano separati. Non è una release né un secondo launcher da consegnare.

## Avvio riproducibile macOS

```sh
PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/Users/pampa/Documents/shiftanything/target CARGO_BUILD_JOBS=2 cargo build -p shiftanything-tauri-proof --offline
/Users/pampa/Documents/shiftanything/target/debug/shiftanything-tauri-proof /tmp/shift-proof-nuovo-it it
```

Il percorso deve essere assoluto e nuovo. Per EN usare un altro percorso nuovo e `en`. La finestra nasce 1440×900 senza attivare altre applicazioni. HOME, Core Foundation, XDG, installazioni e manifest sono indirizzati al profilo temporaneo. Nessuna configurazione client viene scritta. Il primo runtime isolato è solo macOS; nessuna prova Windows/Linux dichiarata.

Otto metodi IPC: `catalog_snapshot`, `native_snapshot`, `record_visual_metrics`, `request_native_action`, `decide_native_action`, `set_favorite`, `check_native_updates`, `open_social_link`. Senza `--lifecycle` rimane la prova di sola lettura. Con `--lifecycle`, Rust rileva versioni e release reali; installazione/aggiornamento/apertura/rimozione/annullamento accettano solo ID curati, operazioni chiuse e una conferma. Lo stato nativo viene rivalidato alla conferma; Update non diventa Install. Nessun percorso, comando shell, argomento di lancio o URL viene accettato dal bridge. I report hanno destinazioni fisse nel profilo. Navigazioni esterne e nuove finestre sono rifiutate. CSP e risorse interamente locali, senza dev server. I sei link social/sito nel footer usano open_social_link con un enum chiuso di destinazioni fisse e il browser di sistema; nessun URL è accettato dal chiamante. Gli altri link mostrano il proprio URL nel footer senza aprirlo. Il catalogo esportato deve avere esattamente gli stessi ID del catalogo Rust.

```sh
/Users/pampa/Documents/shiftanything/target/debug/shiftanything-tauri-proof /tmp/shift-lifecycle-nuovo it --lifecycle
```

La scheda mostra le azioni native e l'esito osservato. Handy non viene aperta se esiste il socket single-instance personale. Per preparare **solo** l'aggiornamento con il DMG ufficiale precedente già verificato:

```sh
/Users/pampa/Documents/shiftanything/target/debug/shiftanything-tauri-proof /tmp/shift-update-nuovo it --lifecycle --seed-handy-previous /tmp/shift-handy-update-proof/Handy_0.10.1_aarch64.dmg
```

Il seed è disponibile solo da CLI, richiede un profilo inutilizzato e il digest esatto 0.10.1, verifica firma e versione con le primitive native. Non avvia Handy. Gli aggiornamenti successivi richiedono la conferma nella finestra.

MCP riproducibile sul profilo in esecuzione (non approva azioni):

```sh
/Users/pampa/.local/bin/uv run --offline tools/verify_tauri_mcp.py /tmp/shift-lifecycle-nuovo
/Users/pampa/.local/bin/uv run --offline tools/verify_tauri_mcp.py /tmp/shift-update-nuovo --operation update
```

Lo script materializza il server auditato nella sola cartella automation del profilo; nessun file client viene modificato. Con `--operation` resta in attesa di Approva/Rifiuta nella finestra.

## Prova grafica

`visual-it.json` / `visual-en.json` nel profilo contengono rettangoli reali del DOM WebKit, stato caricamento immagini e navigazione. Controllo:

```sh
python3 tools/verify_tauri_metrics.py /tmp/shift-proof-nuovo-it/visual-it.json
```

Report senza filtro: 336 schede; larghezza 408.328125 e altezza uniforme 400 px (tutte le righe), preview 134 px, header e toolbar 46 px ciascuno, rail 214 px. Sei loghi e sei prime anteprime reali, 19 slot locali totali. Descrizioni: due righe riservate e troncamento; testo completo in scheda. La nuova altezza uniforme prevale sulle altezze variabili del riferimento Doop. Gli altri 330 record conservano slot vuoti: media non ancora inclusi in questo proof. Azioni e pagine non collegate sono disabilitate esplicitamente. Preferiti salvati per ID nel file atomico Rust `favorites.json`, isolato per profilo; nessuna azione di installazione. Per riaprire un profilo creato dalla prova aggiungere `--resume` (marker verificato). Sidebar in sei famiglie comprimibili, inizialmente chiuse; filtri specifici preservati.

Esportazione opzionale dal riferimento Regia in sola lettura (nessuna API Doop):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tools/export_tauri_proof.py /Users/pampa/Progetti/Regia/orchestratore/specs/launcher-scale-anything /tmp/shift-media-render-clean-20261126
```

L'esportazione usa font e marchi già autorizzati nel repo; immagini e sorgenti design sono registrate con hash/URL in `media-provenance.json`. Icona shell: stessa icona autorizzata, conversione RGBA senza modifica del disegno. Licenze font in `frontend/fonts/OFL-*.txt`; dipendenze in `licenses/`, `ATTRIBUTION.md` e `NOTICE` alla radice. Nessuna lettura di credenziali.
