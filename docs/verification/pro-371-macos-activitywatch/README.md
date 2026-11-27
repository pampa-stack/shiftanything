# ActivityWatch su Mac Apple Silicon

Verifica del 27 novembre 2026. La ricetta seleziona l'edizione classica stabile per Apple Silicon. Usa il DMG ufficiale con tag, byte e SHA-256 corrispondenti e richiede macOS 12 o successivo, come dichiarato da ActivityWatch. Le edizioni Tauri sperimentale e Research hanno pacchetti distinti. [Requisiti e installazione del produttore](https://docs.activitywatch.net/en/latest/getting-started.html)

| Release ufficiale | DMG ARM64 | Dimensione (byte) | SHA-256 |
|---|---|---:|---|
| [v0.14.2](https://github.com/ActivityWatch/activitywatch/releases/tag/v0.14.2) | `activitywatch-v0.14.2-macos-arm64.dmg` | 65.835.856 | `8341d56b5b43a60ca04ea2a3cc5ffbfb5e661cdb6430145d9ceb0f49dca63d65` |
| [v0.14.3](https://github.com/ActivityWatch/activitywatch/releases/tag/v0.14.3) | `activitywatch-v0.14.3-macos-arm64.dmg` | 67.250.584 | `5462f3b90f9e1c0352061888d5f041029abb2ccfbe05e42c6dc63d91ff26bd4a` |

Entrambi i DMG sono stati montati in sola lettura e contengono `ActivityWatch.app`, bundle `net.activitywatch.ActivityWatch` e binario ARM64 `Contents/MacOS/aw-qt`. Il firmatario è Developer ID Application Erik Bjareholt, Team `XM9GC3SUL2`. Verificati SHA-256, byte, bundle, architettura, versione, firma strict/deep, notarizzazione e Gatekeeper. Il binario dichiara minOS 11.0 e la ricetta applica il minimo macOS 12 documentato per il pacchetto completo. Nessun avvio dell'app o dei componenti di monitoraggio sul desktop utente.

Build nativa, formattazione, 128 test core (1 ignorato preesistente) e clippy all-targets core/native superati. La prova di discovery con cache vuota riconosce v0.14.3 dal canale pubblico con API GitHub anonima. Il primo tentativo ha installato v0.14.2 e poi si è fermato al limite di spazio prima dell'aggiornamento. Il profilo temporaneo è stato ripulito.

Al secondo giro il NativeBackend ha scaricato 0.14.2 dal rilascio ufficiale, installato nel profilo temporaneo, scaricato 0.14.3 dal canale stabile e aggiornato. Il manifest nativo registra il pacchetto e il percorso corretti. La rimozione ha spostato il bundle nel Cestino isolato e tolto la ricevuta dal manifest. Il file sintetico del profilo conserva lo stesso SHA-256 in ogni passaggio. La selezione della versione storica e l'aggiornamento hanno consultato le release reali senza precompilare la cache. Il log include la pulizia del profilo.

[Ciclo del motore](activitywatch-native.log) · [Discovery GitHub](activitywatch-discover.log) · [Firma v0.14.2](package-v0.14.2.log) · [Firma v0.14.3](package-v0.14.3.log)
