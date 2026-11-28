# AltTab, LinearMouse e Loop su Mac con chip Apple

Il 28 novembre 2026 il motore del launcher ha scaricato, installato, aggiornato e rimosso tutte e tre le app in profili temporanei distinti. Ogni profilo conserva il file utente di prova e contiene solo il bundle rimosso nel proprio Cestino.

| App | Versione iniziale → nuova | Pacchetto | Fonte |
| --- | --- | --- | --- |
| AltTab | 11.10.1 → 11.10.2 | ZIP | [Release ufficiali](https://github.com/lwouis/alt-tab-macos/releases) |
| LinearMouse | 0.12.0 → 0.13.0 | DMG | [Release ufficiali](https://github.com/linearmouse/linearmouse/releases) |
| Loop | 1.4.1 → 1.4.2 | ZIP | [Release ufficiali](https://github.com/mrkai77/Loop/releases) |

Il controllo preliminare ha verificato SHA-256 pubblicato da GitHub, firma Developer ID, Gatekeeper e binario arm64 in entrambe le versioni. Il limite dell'API GitHub ha richiesto di popolare la cache della cronologia con i metadati delle pagine ufficiali. Il ciclo Rust verifica il download reale, il numero di byte, la versione installata e la ricevuta. Il probe e i comandi per ripetere il ciclo sono in [pro-371-macos-overnight](../pro-371-macos-overnight).

Loop richiede di scartare i tre file AppleDouble identificati nei due ZIP. Il bundle continua a passare la verifica firma e identità. Boring Notch resta fuori dalla matrice: le due versioni provate non hanno superato la verifica della firma di distribuzione.

I cicli riguardano i pacchetti Apple Silicon. L'avvio grafico e i permessi di Accessibilità di queste utility restano da provare.
