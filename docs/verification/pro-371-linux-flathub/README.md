# Cicli Flathub dal motore Sh*ftAnything

Checkpoint del motore `ec16158767d070b0a258ff7b733574679b0cf137`. Il binario del banco usa il crate `artcraft-launcher-native` compilato su VPS1, poi eseguito su VPS Strumenti con Ubuntu 26.04 LTS Linux x86_64 e Flatpak 1.16.6. [Build Linux](linux-build-corrected.log)

Quattro installazioni partono da `Command::InstallExpected` del motore e si concludono su un ref grafico stable Flathub per utente. La prima parte da un profilo senza remote e verifica il bootstrap automatico. Tutte e quattro eseguono l'avvio grafico dal motore prima e dopo l'aggiornamento. La rimozione dal motore lascia il marker dati di prova. [Calcolatrice](native-cycle.log) · [Altri tre cicli](native-lot.log) · [Versioni, commit e fonti](VERIFICA.md)

- Calcolatrice passa da 51.0 a 51.1.
- Text Editor resta a 51.0 e cambia digest. Il motore mostra la nuova revisione anche a numero versione invariato.
- Loupe passa da 51.rc, etichetta mostrata da AppStream per il commit storico del canale stable, a 51.2.
- Papers passa da 51.0 a 51.1.

Per ciascuna app il banco riattiva la revisione storica ufficiale nel profilo di prova e controlla che `can_open` sia attivo e che aggiornamento e rimozione siano rifiutati come copia sostituita all'esterno. Il banco poi imposta nel solo manifest scratch l'identità di una vecchia installazione gestita. Il nuovo processo del motore la rileva e compie l'aggiornamento reale al digest stabile. Il comando di apertura passa a Xvfb privato. Gli override Flatpak di prova isolano rete e filesystem host. [Referti GUI e hash](native-png-sha256.txt)

Un alias per utente chiamato flathub con GPG disattivato viene rifiutato, senza riscrivere la configurazione e senza distribuire app. La lettura del digest completo usa `flatpak info --show-commit`. Il formato elenco di Flatpak 1.16.6 rende il digest abbreviato anche con `active:full`. [Prova remota non verificata](native-untrusted.log) · [Output del primo rilevamento](native-retry-preflight.log)

La prova riguarda i comandi NativeBackend. Il percorso Tauri e l'approvazione MCP si verificano separatamente. Il componente Clippy è assente nella toolchain VPS1, mentre compilazione e ciclo Linux sono passati.

La verifica precedente con Flatpak CLI è autonoma. Questo banco esercita il NativeBackend, compresi persistenza, comandi e avvio. In tutti i quattro casi il digest storico rimane lo stesso dopo Apri e cambia soltanto dopo Aggiorna. Il report dei digest evidenzia che il messaggio iniziale del motore “up to date” era fuorviante dopo un aggiornamento effettivo. La correzione testuale distingue una revisione aggiornata da una già attuale. [Confronto digest dopo Apri e Aggiorna](open-update-state.log) · [Prima prova solo CLI](VERIFICA.md) · [Pulizia e marker](native-cleanup.log)
