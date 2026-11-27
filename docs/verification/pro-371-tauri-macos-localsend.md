# LocalSend in Tauri su macOS Apple Silicon

La ricetta usa il DMG universale dal rilascio stabile `localsend/localsend`. Il motore controlla il digest, la firma Apple, il Team ID `3W7H4PYMCV`, l'identificatore `org.localsend.localsendApp` e la versione. L'abilitazione riguarda macOS arm64, il sistema usato per questa prova.

| Versione | Byte DMG | SHA-256 | Sorgente |
| --- | ---: | --- | --- |
| 1.18.2 | 52.700.247 | `126860d56f6f49b11845f601aac51de27a49b16d2b48102415da91e0e37e5155` | [Release ufficiale](https://github.com/localsend/localsend/releases/tag/v1.18.2) |
| 1.19.0 | 52.878.051 | `5645789cc976298b893267c025f0945d8673baed3bb302a201eb8b96991e43d4` | [Release ufficiale](https://github.com/localsend/localsend/releases/tag/v1.19.0) |

Entrambi i DMG sono stati montati in sola lettura e ciascuna app ha passato `codesign --verify --deep --strict --all-architectures` e Gatekeeper. Il bundle contiene un eseguibile arm64 e x86_64. `CFBundleShortVersionString` corrisponde alla versione attesa, con macOS 11.0 dichiarato nel bundle.

La prova usa le primitive native del launcher e due profili temporanei. Nel primo, il clic **Installa** della scheda e la successiva conferma hanno scaricato i 52.878.051 byte ufficiali. Il backend ha rilevato la versione 1.19.0 in `home/Applications/LocalSend.app`. Una seconda verifica `codesign` ha passato la copia installata.

Nel secondo profilo **Le mie app** ha rilevato 1.18.2 e proposto 1.19.0. Dopo **Aggiorna** e il consenso in finestra, il motore ha scaricato e installato 1.19.0, ricalcolato lo stato e mantenuto il file sintetico di profilo. La versione è stata riletta dal bundle aggiornato e `codesign` ha verificato la copia.

**Rimuovi** con consenso ha spostato la versione 1.19.0 nel cestino temporaneo. L'inventario ha riportato `installed: false` e il file sintetico era ancora intatto. Il client MCP su stdio ha letto l'inventario e ha rifiutato le azioni incompatibili con lo stato installato o rimosso.

**Apri** ha mostrato il consenso Tauri con richiesta `open` per `localsend`. La prova ha scelto **Rifiuta** e osservato `ui-2: rejected`. L'avvio effettivo richiede un account macOS di prova o una VM. LaunchServices separa `HOME` e XDG nel launcher, mentre l'app firmata può accedere al proprio app group macOS e usare la LAN all'avvio. [Entitlements upstream](https://github.com/localsend/localsend/blob/v1.19.0/app/macos/Runner/Release.entitlements).

Per ripetere il test di aggiornamento con il DMG 1.18.2 già scaricato:

```sh
cargo build -p shiftanything-tauri-proof --offline
run_dir="$(mktemp -d /private/tmp/shift-localsend.XXXXXX)"
target/debug/shiftanything-tauri-proof "$run_dir/update-profile" it --lifecycle \
  --seed-localsend-previous /percorso/LocalSend-1.18.2.dmg
```

Il seed richiede un profilo inutilizzato e verifica la ricevuta del DMG prima di installare il bundle. I log di prova locali sono sotto `/private/tmp/shift-localsend-mac-packages.09t4hco_`.
