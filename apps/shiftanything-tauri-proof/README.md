# Sh*ftAnything desktop

La finestra Tauri usa HTML, CSS e JavaScript locali con il motore nativo Rust. Il catalogo include preferiti, Trova app, SH*FTER, Kit di lavoro, Le mie app, impostazioni e configurazione MCP. Il catalogo e il confronto funzionano con dati locali. Le installazioni e gli aggiornamenti usano la ricetta disponibile per il sistema e l’architettura rilevati.

## Avvio locale

```sh
cargo build --locked -p shiftanything-tauri-proof
shift_bin="${CARGO_TARGET_DIR:-target}/debug/shiftanything-tauri-proof"
"$shift_bin"
```

L’avvio normale usa il profilo di Sh*ftAnything e conserva lingua e preferiti. macOS e Linux abilitano il motore di installazione. Le operazioni Windows sono dietro la feature `windows-lifecycle` e richiedono una prova su sistema Windows completo. Le prove eseguite per piattaforma sono in [docs/verification](../../docs/verification).

Il bridge accetta ID di app e operazioni definite dal motore. Rust ricontrolla stato e compatibilità alla conferma. Le fonti del catalogo e i social si aprono nel browser di sistema dopo un controllo sulle destinazioni ammesse. La finestra usa le risorse incluse nella build e le immagini ufficiali presenti nella cache o recuperate dal backend.

## Prova in un profilo temporaneo

```sh
shift_scratch=$(mktemp -d)
"$shift_bin" "$shift_scratch/profile" it --lifecycle
```

Usare un percorso assoluto nuovo con `it` o `en`. `--lifecycle` abilita le azioni sulla copia temporanea. `--resume` riapre un profilo con il marcatore della prova. Per provare l’apertura di app terze usare un account dedicato o una VM, perché alcune app ignorano il profilo passato dal launcher. Handy controlla il suo socket single-instance prima dell’avvio. Per preparare la prova di aggiornamento con il DMG ufficiale precedente:

```sh
"$shift_bin" "$shift_scratch/update" it --lifecycle --seed-handy-previous /path/to/Handy_0.10.1_aarch64.dmg
```

Il seed è disponibile solo da CLI, richiede un profilo inutilizzato e il digest esatto 0.10.1, verifica firma e versione con le primitive native. Non avvia Handy. Gli aggiornamenti successivi richiedono la conferma nella finestra.

MCP riproducibile sul profilo in esecuzione (non approva azioni):

```sh
uv run --offline tools/verify_tauri_mcp.py "$shift_scratch/profile"
uv run --offline tools/verify_tauri_mcp.py "$shift_scratch/update" --operation update
```

Lo script materializza il server auditato nella sola cartella automation del profilo; nessun file client viene modificato. Con `--operation` resta in attesa di Approva/Rifiuta nella finestra.

## Prova grafica

`visual-it.json` / `visual-en.json` nel profilo contengono rettangoli reali del DOM WebKit, stato caricamento immagini e navigazione. Controllo:

```sh
python3 tools/verify_tauri_metrics.py "$shift_scratch/profile/visual-it.json"
```

Il controllo verifica dimensioni uniformi su più righe, intestazioni maiuscole e media iniziali caricati a 1440×900. Aggiungere `--capture-window` all’avvio del profilo temporaneo per acquisire i pixel a 1440×900, con la sola area web e senza icona nel Dock. Le misurazioni sono disponibili nei profili di prova. Il catalogo esportato deve contenere gli stessi ID del catalogo Rust.

Esportazione opzionale dal riferimento Regia in sola lettura (nessuna API Doop):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tools/export_tauri_proof.py /Users/pampa/Progetti/Regia/orchestratore/specs/launcher-scale-anything /tmp/shift-media-render-clean-20261126
```

L'esportazione usa font e marchi già autorizzati nel repo; immagini e sorgenti design sono registrate con hash/URL in `media-provenance.json`. Icona shell: stessa icona autorizzata, conversione RGBA senza modifica del disegno. Licenze font in `frontend/fonts/OFL-*.txt`; dipendenze in `licenses/`, `ATTRIBUTION.md` e `NOTICE` alla radice. Nessuna lettura di credenziali.
