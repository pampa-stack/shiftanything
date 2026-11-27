# Meetily Community e Rizzo PII su macOS arm64

Il 27 novembre 2026, il motore nativo di Sh*ftAnything ha scaricato i DMG ufficiali tramite HTTPS, verificato i digest, installato la versione precedente, scaricato e installato quella successiva e spostato il bundle aggiornato nel cestino di un profilo temporaneo. Il marker sintetico del profilo è rimasto invariato. Un controllo indipendente con `codesign` e Gatekeeper ha accettato firma e notarizzazione dopo ogni fase. I profili di prova sono stati ripuliti.

| App | Versione precedente → successiva | Bundle | Developer ID |
| --- | --- | --- | --- |
| Meetily Community | 0.4.1 → 0.5.0 | `com.meetily.ai` | ZACKRIYA SOLUTIONS (`554AZZ38TB`) |
| Rizzo PII standard | 2.1.0 → 2.2.0 | `com.rizzoai.pii-anonymizer` | Simone Rizzo (`5J76G3383Y`) |

| Pacchetto ufficiale | Byte | SHA-256 |
| --- | ---: | --- |
| [Meetily 0.4.1 Apple Silicon](https://github.com/Zackriya-Solutions/meetily/releases/tag/v0.4.1) | 47.951.342 | `16f84b1769619ba3da93bc43e9c1fa3320ce2b00cc9e99225a3d21302770eaf5` |
| [Meetily 0.5.0 Apple Silicon](https://github.com/Zackriya-Solutions/meetily/releases/tag/v0.5.0) | 48.053.942 | `c7ac63bf80c4085d95388e6c569c95f9a7bcb1b539f3fe964f4522c63eac2ef3` |
| [Rizzo PII 2.1.0 Apple Silicon](https://github.com/Rizzo-AI-Academy/rizzo-pii/releases/tag/v2.1.0) | 132.561.673 | `f94fd21db84e9e355b9bd873c50686ead334e38bebed39ae6daf7f54c336dfe7` |
| [Rizzo PII 2.2.0 Apple Silicon](https://github.com/Rizzo-AI-Academy/rizzo-pii/releases/tag/v2.2.0) | 197.183.674 | `eb732c316dc5a60c84afe5a921162c7abb52575b3fb6e0aaf0c7902642c0e808` |

La prova forza nel backend i metadati delle due release ufficiali, così da verificare anche la versione precedente. I file scaricati, il montaggio, l’installazione, l’aggiornamento e il cestino usano il motore reale. La selezione delle release è coperta dal test `community_dmg_recipes_accept_documented_apple_silicon_packages`. La ricetta seleziona Meetily Community e il DMG standard di Rizzo PII, mentre respinge i pacchetti Pro e offline.

La prova del motore copre installazione, aggiornamento e rimozione. Un test precedente ha eseguito il solo sidecar Meetily `llama-helper`, con `ping/pong` e chiusura pulita in un profilo isolato. Le interfacce grafiche, l’uso del microfono e le funzioni di anonimizzazione restano da verificare. Il minimo documentato di Meetily e la versione minima Mach-O del binario Rizzo PII arm64 sono macOS 11. [Requisiti Meetily](https://meetily.ai/downloads/mac)

Log ripetibili nella stessa directory `meetily-native.log`, `rizzo-native.log`, `rizzo-architecture.log` e `cargo-meetily-rizzo-tests.log`.
