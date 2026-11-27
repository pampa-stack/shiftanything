# Stats e Whispering su macOS Apple Silicon

Verifica del 27 novembre 2026 sul codice 7007986 con la patch scratch `stats-whispering-native.patch`. Solo le ricette Apple Silicon sono abilitate. Il Mac ha mantenuto la riserva interna minima di 2,5 GiB; i DMG di controllo e gli artefatti sono passati dal T7.

| App | Versioni installata → aggiornata | Bundle | Team ID | Minimo effettivo ARM64 |
|---|---|---|---|---|
| Stats | 3.1.0 → 3.1.1 | `eu.exelban.Stats` | `RP2S87B72W` (Serhiy Mytrovtsiy) | 12.0 (plist e binario, universal) |
| Whispering | 7.9.0 → 7.11.0 | `com.bradenwong.whispering` | `3C7J97QA5Z` (Braden Wong) | 11.0 (binario ARM64, plist generico 10.15) |

- Stats 3.1.0, 8.257.600 byte, SHA-256 `a5f16b6606c8bd1fc847439a3b4a507153837167cb333514954882c2bc31e0e7`. [Release ufficiale](https://github.com/exelban/stats/releases/tag/v3.1.0).
- Stats 3.1.1, 8.386.915 byte, SHA-256 `2fbe64690251e5c92102a2091052c76f440561397fcb7e0246821c331386bdd9`. [Release ufficiale](https://github.com/exelban/stats/releases/tag/v3.1.1).
- Whispering 7.9.0, 15.206.641 byte, SHA-256 `6a0e45d47ecdb2ada6afa8309914dd73c03db896fd153272e0891976867851c7`. [Release ufficiale](https://github.com/EpicenterHQ/epicenter/releases/tag/v7.9.0).
- Whispering 7.11.0, 15.606.745 byte, SHA-256 `7c15883a5562e594633e2636db5abbc8197b8ae93c4714711bf802b531eaf2d0`. La release intermedia 7.10.0 non offre il DMG ARM. [Release ufficiale](https://github.com/EpicenterHQ/epicenter/releases/tag/v7.11.0).

Entrambi i cicli NativeBackend hanno scaricato per HTTPS il DMG della release scelta e quello dell'ultimo aggiornamento. Il manifest e il bundle nell'Applications del profilo riportavano inizialmente la versione precedente, poi quella nuova. La rimozione ha tolto l'app dal manifest e spostato il bundle nella Trash temporanea. Stessa firma Developer ID, notarizzazione e accettazione Gatekeeper nelle tre fasi. Il marker dei dati è rimasto identico dopo installazione, aggiornamento e rimozione. Su ogni DMG montato in sola lettura abbiamo verificato i digest GitHub ufficiali e letto l'architettura dal binario.

La quota REST anonima GitHub era esaurita. Solo la lista delle versioni è stata fornita dalla cache di prova, popolata con la risposta REST ufficiale già acquisita tramite gh autenticato. NativeBackend ha verificato la release corrente tramite il redirect pubblico e scaricato i quattro DMG dai link ufficiali. Nessun token è stato trasferito alla prova. Apertura GUI, supporto Intel e installazione di eventuali helper amministrativi di Stats restano da verificare. [Opzioni del monitor Stats](https://github.com/exelban/stats#readme).

- Log dei flussi `stats-lifecycle-seeded.log` e `epicenter-whispering-lifecycle-seeded.log`.
- `cargo fmt --all --check`, `git diff --check`, core 128 PASS/1 ignorato e clippy core/native tutti i target PASS. Suite native in profilo pulito 555 PASS/7 ignorati e un fallimento del fixture MCP PhotoCraft con uv a freddo. Stesso fixture PASS alla ripetizione isolata con dipendenze uv già disponibili (14,36 s). Vedi `core-test.log`, `clippy.log`, `native-test.log` e `native-test-mcp-rerun.log`.
