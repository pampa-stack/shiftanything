# Gestione nativa Tauri — 27 novembre 2026

Le mie app / My apps e Aggiornamenti / Updates mostrano l’inventario reale del NativeBackend. Versione installata, aggiornamento disponibile, progresso e risultato arrivano dal protocollo Rust. Nessuna azione simulata. Check for updates invia CheckForUpdates(force=true): il messaggio conferma solo la richiesta, non inventa il completamento. Aggiorna tutte richiede update_all sui soli ID installati, compatibili, gestibili e con aggiornamento disponibile; conferma e rivalidazione native esistenti. Dettagli, apertura, aggiornamento e rimozione riusano gli stessi controlli della scheda.

## Ciclo eseguito

Profilo isolato /tmp/shift-tauri-management-it, macOS ARM64. Seed del DMG ufficiale Handy 0.10.1 col digest e firma già verificati; nessuna apertura o installazione personale toccata.

- IT: Le mie app rileva 0.10.1 e nuova 0.10.2. Aggiorna tutte → Rifiuta: risultato rejected, nessun job, versione invariata.
- Riavvio EN nello stesso profilo con --resume: inventario 0.10.1 conservato. Clic CHECK FOR UPDATES inviato al backend.
- UPDATE ALL → consenso reale → Approve: InstallExpected(installed=true), download del DMG ufficiale, SHA/firma native, aggiornamento completato a 0.10.2. Updates 0, Installed 1, UPDATE ALL disabilitato. Il file sintetico nel profilo com.pais.handy conserva synthetic-data-keep.
- Client MCP stdio reale: Install su installata, Update/UpdateAll senza nuova versione, Cancel senza job, ID e task sconosciuti rifiutati (mcp.log).
- Riavvio EN: stato 0.10.2 conservato. DETAILS apre la scheda e CLOSE torna alla gestione; REMOVE → Approve → esito completato, bundle assente e nuovo rilevamento Installed 0. Dato sintetico conservato.
- Ritorno CATALOG: toggle attivo Value 1 e tutte le 336 schede misurate 408.328×400; viewport 1440×900, header/barra46, rail214 e sei media PASS. Corretto il ripristino del toggle quando il catalogo era già la vista selezionata prima della gestione.

PNG allegati letti insieme agli alberi accessibili. Prima navigazione IT: due timeout del provider AX, processo inattivo verificato tramite sample e screenshot proprio; clic background dalle coordinate del controllo, poi AX e PNG confermano il cambio. Non viene dichiarato un blocco dell’app. Una verifica iniziale del file sintetico usava la directory Handy errata; il percorso effettivo com.pais.handy è stato riletto prima di dichiarare preservazione.

## Controlli e limiti

cargo test -p shiftanything-tauri-proof --offline: 2 PASS. cargo clippy -p shiftanything-tauri-proof --all-targets --offline -- -D warnings: PASS. Build offline, node --check native.js/management.js, fmt e diff check PASS. Cargo seriale, target condiviso autorizzato, due job. Client: uv run --offline tools/verify_tauri_mcp.py PROFILO.

UPDATE ALL è stato eseguito con UNA sola app idonea, non dimostra bulk multi-app, errore parziale o annullamento in corso. OPEN resta protetto dal socket personale Handy e non è provato come successo. Nessuna seconda ricetta Mac abilitata. Altre pagine disattivate nella prova Tauri, Trova app ancora solo dati. Nessuna dichiarazione di parità completa, nuovi conteggi installabili, Linux/Windows Tauri o pacchetto finale. PR prodotto e proof restano separati.
