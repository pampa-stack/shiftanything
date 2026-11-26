# Sh*ftAnything: prova desktop Tauri separata

Proof reversibile richiesto esplicitamente da Pampa tramite Regia il 27 novembre 2026, dal checkpoint prodotto `7a2b6ce`. Eccezione deliberata alla raccomandazione Rust-only/egui di AGENTS.md: HTML/CSS/JavaScript locali nella finestra Tauri, motore Rust esistente. Il prodotto egui e PR3 restano separati. Non è una release né un secondo launcher da consegnare.

## Avvio riproducibile macOS

```sh
PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/Users/pampa/Documents/shiftanything/target CARGO_BUILD_JOBS=2 cargo build -p shiftanything-tauri-proof --offline
/Users/pampa/Documents/shiftanything/target/debug/shiftanything-tauri-proof /tmp/shift-proof-nuovo-it it
```

Il percorso deve essere assoluto e nuovo. Per EN usare un altro percorso nuovo e `en`. La finestra nasce 1440×900 senza attivare altre applicazioni. HOME, Core Foundation, XDG, installazioni e manifest sono indirizzati al profilo temporaneo. Nessuna configurazione client viene scritta. Il primo runtime isolato è solo macOS; nessuna prova Windows/Linux dichiarata.

Tre soli metodi IPC: `catalog_snapshot`, `native_snapshot`, `record_visual_metrics`. I primi due leggono il catalogo Rust e gli eventi reali `Command::Detect`/`Event::Detected` del NativeBackend isolato. Il terzo scrive un report limitato, con ID ammessi, in un file fisso del profilo; non accetta percorsi. Nessun comando install/update/remove/open, shell, filesystem generico o URL opener è esposto. Navigazioni esterne e nuove finestre sono rifiutate. CSP e risorse interamente locali, senza dev server. I link mostrano il proprio URL nel footer senza aprirlo. Il catalogo esportato deve avere esattamente gli stessi ID del catalogo Rust.

## Prova grafica

`visual-it.json` / `visual-en.json` nel profilo contengono rettangoli reali del DOM WebKit, stato caricamento immagini e navigazione. Controllo:

```sh
python3 tools/verify_tauri_metrics.py /tmp/shift-proof-nuovo-it/visual-it.json
```

Report senza filtro: 298 schede; larghezza 408.328125 e altezza uniforme 400 px (tutte le righe), preview 134 px, header e toolbar 46 px ciascuno, rail 214 px. Sei loghi e sei prime anteprime reali, 19 slot locali totali. Descrizioni: due righe riservate e troncamento; testo completo in scheda. La nuova altezza uniforme prevale sulle altezze variabili del riferimento Doop. Gli altri 292 record conservano slot vuoti: media non ancora inclusi in questo proof. Azioni e pagine non collegate sono disabilitate esplicitamente. Preferiti attualmente nel localStorage del solo profilo WebKit; non ancora collegati alle Settings Rust del prodotto.

Esportazione opzionale dal riferimento Regia in sola lettura (nessuna API Doop):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tools/export_tauri_proof.py /Users/pampa/Progetti/Regia/orchestratore/specs/launcher-scale-anything /tmp/shift-media-render-clean-20261126
```

L'esportazione usa font e marchi già autorizzati nel repo; immagini e sorgenti design sono registrate con hash/URL in `media-provenance.json`. Icona shell: stessa icona autorizzata, conversione RGBA senza modifica del disegno. Licenze font in `frontend/fonts/OFL-*.txt`; dipendenze in `licenses/`, `ATTRIBUTION.md` e `NOTICE` alla radice. Nessuna lettura di credenziali.
