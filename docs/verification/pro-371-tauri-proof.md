# PRO-371 — checkpoint grafico Tauri macOS

27 novembre 2026. Ramo isolato `f/pro-371-tauri-proof`, nato da `7a2b6ce`; prodotto egui/PR3 invariati. Richiesta esplicita operatore: provare HTML/CSS Doop dentro Tauri prima di ulteriore pixel matching egui. Non è parità completa e non è una release.

## Eseguito

- Build offline `cargo build -p shiftanything-tauri-proof`: PASS dopo allineamento Tauri 2.12.5/runtime 2.12 e icona shell RGBA. I tentativi iniziali 2.11.5 con runtime 2.12 fallivano dentro Tauri (Monitor/Menu); icona originale RGB rifiutata da generate_context. Cause corrette, nessun file registry modificato.
- `cargo clippy -p shiftanything-tauri-proof --all-targets --offline -- -D warnings`: PASS, log `/tmp/shift-tauri-clippy.log`.
- `cargo test -p shiftanything-tauri-proof --offline`: PASS, 1 test del confine di navigazione; log `/tmp/shift-tauri-tests.log`.
- Finestra reale macOS WebKit 1440×900, ScreenCaptureKit tramite Orca in background. PNG e alberi accessibilità letti in IT/EN; nessun ripristino/attivazione della finestra. Prove conservate in `docs/verification/pro-371-tauri/` e `/tmp/shift-tauri-visual-proof-20261127/`.
- Report reali: 298 card IT/EN, 408.328125×400 px, incluse righe inferiori e titoli lunghi/media mancanti. Filtro Audio EN: 42 card della stessa dimensione. Preview 134 px, header 46, toolbar 46, rail 214, margine esterno zero. Etichette principali tutte maiuscole, due righe descrizione riservate. Frecce CSS dei tre select visibili, GitHub testo accanto alle stelle, primi sei loghi/anteprime reali locali.
- Catalogo IPC confronta 298 ID reali Rust; piattaforma MacOs/Arm64 e 20 risposte di rilevamento NativeBackend nel profilo isolato. Footer verificato nel PNG. Un errore iniziale di ordine script/footer impediva il riepilogo: corretto con `defer`, senza simulare lo stato.
- Click cuore PhotoCraft: AXPress in background, stato accessibile passato da 0 a 1. In questa fase è localStorage WebKit, non Settings Rust; persistenza dopo riavvio da verificare. Ricerca: AX set-value non confermato e AX text insertion non confermato; nessun risultato di ricerca dichiarato. Filtro categoria tramite click AX e stato/PNG verificati.

## Linux: solo prerequisiti

SSH diretto a VPS1 non risolveva il nome. Tramite ambiente Orca VPS1, terminale dedicato nel repo privato `/mnt/dati/lavori/shiftanything`: x86_64, prompt `ubuntu@vps-builder`; `pkg-config` assente, pacchetti `libwebkit2gtk-4.1-dev` e `pkgconf` assenti; 14 GiB liberi. Nessuna dipendenza installata e nessuna build Tauri Linux dichiarata. Non duplicata la track LocalSend.

## Limiti aperti

Solo catalogo/griglia confrontato con `/tmp/shift-doop-finished-grid-{it,en}.png`. Altezza 400 uniforme intenzionale, nuovo brief prevalente sulle righe variabili Doop. I pulsanti install/update/open/remove e pagine non collegate sono disabilitati; niente simulazioni. Installazioni ufficiali, conferma, progresso e preferiti Rust saranno collegati dopo questo checkpoint. Altri 292 record hanno slot media ancora vuoti in questo proof. Nessuna prova Windows; nessun deploy, release, merge o mutazione delle installazioni personali.

Riproduzione: README dell'app, export deterministico da fonti approvate, manifest URL/hash; `tools/verify_tauri_metrics.py` controlla report della finestra reale senza installare app.
