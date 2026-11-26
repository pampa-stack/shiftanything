# Preferiti nel profilo Rust — 27 novembre 2026

Click reali in finestra macOS, profilo `/tmp/shift-tauri-favorites-20261127`.
Salva Handy in IT → `favorites.json` contiene `["handy"]`; riavvio con `en --resume` → cuore attivo e filtro FAVORITES mostra solo Handy; rimozione dal cuore → `[]`.
Screenshot 1440×900 e albero accessibilità letti. `pending=null`, `progress={}`, modalità lifecycle disabilitata: nessuna richiesta di installazione.

Comandi eseguiti: `cargo test -p shiftanything-tauri-proof --offline` (2 PASS), `cargo clippy -p shiftanything-tauri-proof --all-targets --offline -- -D warnings` (PASS), `cargo build -p shiftanything-tauri-proof --offline` (PASS), `cargo fmt --all --check` (PASS).

Il test Rust verifica anche isolamento fra profili e rifiuto ID sconosciuto. La ripresa richiede il marker del profilo creato dall’app. La persistenza usa un file atomico a percorso fisso, senza dipendere da localStorage. Nessuna prova di navigazione tastiera completa in questo checkpoint.
