# Prova del pacchetto Release per Windows x64

Il 27 novembre 2026 abbiamo compilato Sh*ftAnything 0.20.6 in un guest Windows 11 Enterprise Evaluation x64. Il sorgente `b9ebab7cb3399b74ed8452d93223f279a3f799b2` include `3a6503b`. Toolchain verificata con Rust 1.95.0 MSVC, Visual Studio Community 2022 17.14.45, Windows SDK 26100, Tauri CLI 2.12.6 e NSIS 3.13.

Dal prompt di sviluppo x64 di Visual Studio, nella radice del repository, eseguire:

```bat
cargo build --locked --release --target x86_64-pc-windows-msvc -p shiftanything-tauri-proof --features windows-lifecycle
cd apps\shiftanything-tauri-proof
cargo tauri bundle --ci --no-sign --target x86_64-pc-windows-msvc --bundles nsis --features windows-lifecycle --config tauri.nsis-qa.json
```

`ShiftAnything_0.20.6_x64-setup.exe` misura 37.540.770 byte. SHA-256 `248240c0d109aad5feb28d871ebc669ef3a694a233408a39efdeb9fa0f41ced6`, confermato dopo l'esportazione dallo stesso guest. Authenticode riporta `NotSigned`. Il binario interno dichiara la versione 0.20.6 e il sottosistema PE `2` (Windows GUI). [Esito build e bundle](release-scan.png)

Nel profilo di prova Administrator in Audit Mode, il wizard ha installato il launcher per l'utente corrente in `C:\sq\installed-release`. Il pulsante Finish ha avviato la finestra nativa Sh*ftAnything con icona nella barra, comandi finestra e 531 app. La prima cattura durante l'inizializzazione mostra la WebView bianca prima che si carichi il catalogo. [Fine installazione](nsis-finish.png) · [Catalogo](release-catalog.png)

La ricerca OpenGame restituisce una scheda. View App apre il dettaglio con copertina, licenza Apache-2.0, MCP e guida all'installazione self-hosted. [Ricerca](opengame-search.png) · [Dettaglio](opengame-view.png)

L'uninstaller ha eliminato la registrazione e gli eseguibili del launcher. Il file sintetico sotto `%LOCALAPPDATA%\ShiftAnything\desktop` è stato conservato con l'opzione predefinita. [Esito rimozione](release-removed.png)

Il test è locale. La distribuzione pubblica, SmartScreen, l'esecuzione sotto un utente standard e l'autoaggiornamento Windows richiedono verifiche dedicate.
