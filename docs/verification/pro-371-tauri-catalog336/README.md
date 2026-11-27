# Catalogo Tauri 336 — 27 novembre 2026

Finestra macOS reale a 1440×900, click/accessibilità in background; PNG IT/EN letti. Le sei famiglie della sidebar partono chiuse. Click Sistema e infrastruttura → Backup e ripristino: 8 schede, filtro specifico preservato. Tutti i 336 ID coincidono con Rust. Nessuna ricetta aggiunta dai metadati.

`python3 tools/verify_tauri_metrics.py docs/verification/pro-371-tauri-catalog336/visual-{it,en}.json docs/verification/pro-371-tauri-catalog336/visual-backup-it.json`: PASS, 336 IT + 336 EN + 8 Backup IT. Card 408,328×400 su tutte le righe, preview 134, header/toolbar 46, rail 214; etichette principali maiuscole; sei loghi e sei anteprime locali caricate.

Core/catalogo e contratto MCP: vedere ../pro-371-catalog336.md. Test proof 2 PASS, clippy all-targets `-D warnings` PASS, build offline PASS. Correzione dopo prova reale: schede senza immagine interrompevano il report geometrico; accessi naturalWidth ora tollerano l’assenza e riportano false. Misura ripetuta con successo. Un primo report mancava anche prima del caricamento del webview: nessuna misura anticipata dichiarata riuscita. Un indice AX stale è stato riletto prima di agire.

Export riproducibile mantiene adattamento Preferiti Rust, senza reintrodurre localStorage. Tre select con frecce CSS e sorgente accanto alle stelle. Font e asset locali conservati con manifest di provenienza aggiornato.

Limiti: prime sei app con media inclusi, altri 330 con slot non inclusi nel proof. Solo catalogo/griglia, filtro Backup e precedente checkpoint Preferiti verificati in questa finestra. Le pagine Gestione, SH*FTER, Trova app, Kit e impostazioni/MCP restano disabilitate nel proof Tauri; rimangono nel prodotto egui. Nessuna parità globale dichiarata, nessun nuovo ciclo di installazione, prova UI Linux o Windows. Ciclo ufficiale Handy precedente in ../pro-371-tauri-lifecycle/README.md: install/update/remove, Open bloccato per istanza personale.
