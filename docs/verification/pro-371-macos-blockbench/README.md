# Blockbench 5.2.1 → 5.2.2 su macOS Apple Silicon

Prova del 27 novembre 2026, da `88caad1` con patch `blockbench-native.patch`. DMG arm64 ufficiali verificati via SHA-256 pubblicato da GitHub. Entrambi contengono `Blockbench.app` con bundle `blockbench`, versione identica al tag e macOS 12.0 come minimo nel plist e Mach-O. La firma Developer ID di Jannis Petersen (Team `5QMG7G7462`), la validazione profonda, Gatekeeper senza cache e il ticket dell'app sono positivi per entrambe le versioni. L'immagine DMG non ha un ticket autonomo. [5.2.1](https://github.com/JannisX11/blockbench/releases/tag/v5.2.1) · [5.2.2](https://github.com/JannisX11/blockbench/releases/tag/v5.2.2) · [Sorgente e licenza GPL-3.0](https://github.com/JannisX11/blockbench)

| Tag | Byte | SHA-256 |
| --- | ---: | --- |
| v5.2.1 | 128.702.946 | 20fd51a18c7a757d9cb03bb0b89c99874d2d5bdbae733817a7877b15043aa78c |
| v5.2.2 | 128.721.695 | 0d072bf863ee3e0bcaecbd0de67a5ffd29dfbf6ee2e8fe5eeb872a14331c5c03 |

- Core e compilazione Native passati. Il selettore rifiuta asset Intel, tag errato, dimensione alterata, checksum assente e prerelease.
- Installazione Native da HTTPS 23,300 s. Aggiornamento Native con verifica Gatekeeper e swap 85,129 s. Rimozione Native 0,024 s. Bundle post-update di ~318 MiB, ricevute coerenti, dati sintetici mantenuti e Cestino isolato.
- Finestra dell'app e azione Apri del launcher da provare in sessione macOS separata. Durante questa esecuzione non sono stati avviati Electron, plug-in o LaunchServices dell'utente.
