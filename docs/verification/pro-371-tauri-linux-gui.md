# Ciclo desktop Tauri su Linux x86_64

Il 27 novembre 2026 a partire dalle 02:05 UTC, la finestra Tauri `11f453b1c9f4c36784178155f5a387a0867f3ff1` ha installato, aperto, aggiornato, riaperto e rimosso AniCraft da Ubuntu 24.04 x86_64 su VPS2. La prova ha usato i controlli della finestra in italiano, con input X11 e screenshot 1440 × 900. [Catalogo avviato](pro-371-tauri-linux-gui/launcher-ready.png)

| Azione nella finestra | Risultato osservato | Screenshot |
| --- | --- | --- |
| Installa → Rifiuta | Richiesta `ui-1` rifiutata, nessuna richiesta HTTP dell’AppImage e nessun eseguibile installato. | [Conferma](pro-371-tauri-linux-gui/install-approval.png) |
| Installa → Approva | Scaricata e installata `0.2.0`. Hash dell’AppImage installata, voce desktop e manifest verificati. | [AniCraft 0.2.0 in Le mie app](pro-371-tauri-linux-gui/manage-before-update.png) |
| Apri → Approva | Finestra AniCraft con editor e timeline. | [AniCraft 0.2.0 in esecuzione](pro-371-tauri-linux-gui/anicraft-020.png) |
| Cerca aggiornamenti → Aggiorna → Approva | Rilevata `0.2.1`, scaricata la release e aggiornati AppImage e voce desktop. La vecchia AppImage è stata rimossa. Hash e versione verificati. | [Aggiornamento disponibile](pro-371-tauri-linux-gui/update-detected.png), [aggiornamento completato](pro-371-tauri-linux-gui/update-complete.png) |
| Apri → Approva | Finestra AniCraft che riporta `0.2.1`. | [AniCraft 0.2.1 in esecuzione](pro-371-tauri-linux-gui/anicraft-021.png) |
| Rimuovi → Approva | AppImage, voce desktop e record del manifest rimossi. Hash della fixture nel profilo XDG di AniCraft invariato dopo aggiornamento e rimozione. | [Le mie app vuota](pro-371-tauri-linux-gui/remove-complete.png) |

## Pacchetti ed esecuzione

| AppImage ufficiale | Byte | SHA-256 |
| --- | ---: | --- |
| [AniCraft 0.2.0](https://github.com/storytold/anicraft/releases/download/v0.2.0/anicraft-0.2.0-linux-x86_64.AppImage) | 16062968 | `e01b1d334deb0b0249840e0dab3ea3baadb0568bfa0503c2fcd29f6265235a14` |
| [AniCraft 0.2.1](https://github.com/storytold/anicraft/releases/download/v0.2.1/anicraft-0.2.1-linux-x86_64.AppImage) | 19024376 | `352456384d1e82f373d836dcbcd9f1473b1603c5c9c7b185d763218470091b31` |

Il sysroot utente ha estratto 75 pacchetti `.deb` ufficiali da Ubuntu e riusato i runtime già presenti dove i pacchetti di sviluppo contenevano solo collegamenti. `pkg-config` ha rilevato GTK 3.24.41 e WebKitGTK 2.52.6. La build `cargo build --locked -j2 -p shiftanything-tauri-proof --features artcraft-launcher-native/local-mirror` ha prodotto un ELF x86-64 con SHA-256 `558533ee888a13b87dd49d54fc635d823632edad7f9838e8ec5d25dbc1b14d02`.

Il motore nativo ha scaricato i due pacchetti ufficiali, già confrontati con i digest dell’editore, da un mirror HTTP loopback configurato per la prova. Il feed del mirror è passato da `0.2.0` a `0.2.1` prima del clic su Cerca aggiornamenti. Bubblewrap teneva rete e processi in namespace privati, con HOME e XDG temporanei. Xvfb, D-Bus e il mirror sono stati fermati a fine prova. Le dipendenze di sistema e i profili esistenti non sono stati modificati. Il precedente ciclo nativo su VPS1 copre il download HTTPS dai repository ufficiali. [Prove delle ricette Linux](pro-371-tauri-linux-recipes.md)

Gli originali sono in `vps2:/mnt/dati/lavori/shift-tauri-linux-gui.m0Wn3750/`. `prepare-sysroot.sh`, `gui-envelope.sh`, `fixture/run.sh`, `gui-action.py`, `set-mirror-release.py` e `assert-lifecycle.py` sono gli script del banco. `build-final.log`, `fixture/desktop.log`, `fixture/mirror.log`, `fixture/input.log`, `install-proof.log`, `update-proof.log`, `remove-proof.log` e `final-state.log` riportano build, operazioni, richieste HTTP, click e stato finale.

La scheda Linux mostrava ancora la dimensione editoriale di macOS, `35,7 MB`, mentre installava il pacchetto Linux da `16062968` byte. Logo e anteprima di AniCraft nella scheda erano vuoti nel sandbox senza accesso a Internet. La gestione in italiano mostrava stringhe di risultato del motore in inglese. La prova riguarda un’app in un profilo privato su Ubuntu x86_64 e usa il mirror locale per il cambio di release. Restano da verificare il feed pubblico firmato nella finestra Tauri, il packaging di distribuzione e la parità con il design Doop.
