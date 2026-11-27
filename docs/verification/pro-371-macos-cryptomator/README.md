# Cryptomator su macOS Apple Silicon

Prova del 27 novembre 2026. Il NativeBackend ha installato 1.19.6, aggiornato a 1.19.7 e rimosso nel Cestino del profilo temporaneo. I dati sintetici sono rimasti intatti. Discovery a profilo vuoto ha risolto 1.19.7 via canale di fallback ufficiale GitHub Pages. [Ciclo nativo](cryptomator-lifecycle.log) · [Discovery](cryptomator-discover.log)

| Release ufficiale | Dimensione DMG arm64 | SHA-256 |
|---|---:|---|
| [1.19.6](https://github.com/cryptomator/cryptomator/releases/tag/1.19.6) | 58.714.109 byte | `fdfc8ef50b10baca1078b7a9edf404e330363c8a3a40fda29f86dd5d3a0a0b02` |
| [1.19.7](https://github.com/cryptomator/cryptomator/releases/tag/1.19.7) | 58.716.219 byte | `c4818d38cb3f9919bfe276c812714596cc50da4d01a82ac2f22bf2e64b441677` |

Entrambi contengono `org.cryptomator`, firmato Developer ID Application Skymatic GmbH, Team `YZQJQUHA3L`. Montaggio in sola lettura e verifica di bundle, versione, architettura, firma deep/strict e Gatekeeper. Richiesto macOS 11 o successivo. Finestra, vault e integrazione FUSE da provare separatamente. [Controlli 1.19.6](cryptomator-1.19.6-package.log) · [Controlli 1.19.7](cryptomator-1.19.7-package.log) · [Documentazione del produttore](https://docs.cryptomator.org/desktop/setup/)
