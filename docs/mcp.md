# Sh*ftAnything: collegare un agente locale

Apri **Collega agente** nel launcher. Scegli JSON (Claude Desktop, Cursor, LM Studio), VS Code oppure TOML (Codex), quindi **Copia configurazione** e incolla nelle impostazioni MCP del client. Il launcher prepara il server e il catalogo soltanto nel proprio profilo; non modifica le configurazioni dei client e non li riavvia. Il comando richiede `uv` nel PATH del client. La prima esecuzione di uv risolve la dipendenza fissata `mcp==1.30.0`.

Il client avvia il server con trasporto stdio e con `--automation-read-root`, `--automation-write-root` e `--bridge-root` limitati alla cartella `automation` di Sh*ftAnything. Non rimuovere queste opzioni. Per usare soltanto il catalogo dal checkout:

```sh
uv run tools/mcp_catalogo.py --automation-read-root "$PWD" --automation-write-root "$PWD"
```

Senza `--bridge-root` sono disponibili soltanto `catalog_filters`, `search_apps` e `get_app`. Il server rifiuta l'avvio senza entrambe le root. Non espone shell, script, argomenti di avvio, percorsi o URL forniti dall'agente.

## Comandi reali del protocollo

Questi sono nomi di tool e relativi argomenti JSON da chiedere al client MCP, non comandi shell:

```text
catalog_filters {}
search_apps {"query":"Granola","level":"base"}
search_apps {"query":"Linear"}
search_apps {"query":"OCR"}
get_app {"app_id":"handy"}
get_inventory {}
request_action {"operation":"install","app_ids":["handy"]}
get_request_status {"request_id":"<ID restituito dalla richiesta>"}
```

Con bridge attivo, il launcher deve rimanere aperto. **Verifica connessione** controlla la presenza di uv e un tool realmente chiamato nell'ultimo minuto. Senza una chiamata recente mostra un risultato negativo; non simula una connessione. La verifica riguarda questo server e questo profilo, non certifica che un particolare client sia configurato.

`request_action` ammette `install`, `update`, `open`, `remove`, `update_all` e `cancel`, esclusivamente per ricette native curate e identità repository corrispondenti. Una scheda informativa non basta. `update_all` richiede una lista esplicita di app installate con aggiornamento disponibile. La UI rivalida lo stato all'approvazione; il backend conserva separatamente la presenza attesa per installazione e aggiornamento. Aggiornare un'app assente viene rifiutato.

Ogni richiesta va approvata o rifiutata nella UI. Le conferme native di destinazione/rimozione rimangono operative. Chiudere la conferma interrompe l'azione. La coda su disco ha capienza uno: il server pubblica un JSON già chiuso con hard link atomico esclusivo. Una richiesta occupata o un link inatteso non viene sovrascritto.

## Esiti e storico

`queued` e `approved; native operation requested (not completed)` non significano completamento. `get_request_status` restituisce l'esito nativo per ciascuna app in `outcomes`. Un avvio riuscito significa che la richiesta nativa di avvio è stata accettata; non certifica che la finestra dell'app sia rimasta aperta. Errori e annullamenti rimangono distinti. Inventario e risultati riportano anche se lo snapshot è scaduto. ID sconosciuti non diventano installazioni.

Lo storico della finestra **Collega agente** appartiene alla sessione aperta, con limite di 256 voci. Non è un registro persistente di audit. I tool di sicurezza presenti nel catalogo restano metadati: non vengono esposte funzioni di attacco.

## Prove riproducibili

```sh
uv run tools/verify_mcp.py
cargo test -p artcraft-launcher-core automation --lib
cargo test -p artcraft-launcher-native approved_update --lib
cargo test -p artcraft-launcher-ui-egui agent_approval --lib
```

Lo script stdio usa directory IPC temporanee e distingue esplicitamente i fixture dagli esiti di esecuzione nativa. Le prove complete attraverso MCP, UI e motore con un pacchetto ufficiale restano da completare: vedi [registro PRO-371](verification/pro-371.md).
