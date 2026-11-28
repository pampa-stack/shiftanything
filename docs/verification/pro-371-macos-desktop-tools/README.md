# Dyad, Finicky e Hammerspoon su Mac con chip Apple

Il 28 novembre 2026 il motore nativo ha scaricato, installato, aggiornato e rimosso i tre pacchetti in profili temporanei distinti. I dati sintetici sono rimasti integri e i bundle rimossi sono finiti nel Cestino del profilo.

| App | Versioni provate | Formato | Fonte |
| --- | --- | --- | --- |
| Dyad | 1.23.0 → 1.24.0 | ZIP arm64 | [Release ufficiali](https://github.com/dyad-sh/dyad/releases) |
| Finicky | 4.2.1 → 4.2.2 | DMG arm64 | [Release ufficiali](https://github.com/johnste/finicky/releases) |
| Hammerspoon | 1.1.0 → 1.1.1 | ZIP universale | [Release ufficiali](https://github.com/Hammerspoon/hammerspoon/releases) |

Il controllo preliminare verifica SHA-256 pubblicato, firma Developer ID con team atteso, Gatekeeper, notarizzazione e slice arm64. Il probe [macOS](../pro-371-macos-overnight) usa la cache release temporanea con le pagine GitHub ufficiali e scarica i pacchetti originali durante il test, con verifica di byte, integrità, identità e versioni. Il log riporta i tre cicli riusciti.

I due ZIP con framework usano il lettore esistente. Solo Dyad può ricreare i collegamenti Git nel suo bundle, verso i due eseguibili Git nella stessa cartella. Il lettore crea i link dopo l'estrazione dei file e verifica che si risolvano nella directory prevista. Il test ZIP verifica anche il rifiuto di destinazioni e percorsi diversi. Il collaudo riguarda la gestione pacchetti su Apple Silicon. L'avvio delle interfacce grafiche rimane da verificare.
