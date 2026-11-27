# Installazione locale Windows x64
27 novembre 2026, Windows 11 Enterprise Evaluation su AX42. Versione launcher 0.20.6, sorgente `cfaa0027be7636175638a55b63b0b75438e1ad21` da `400beb2`. Compilazione con Rust 1.95.0 MSVC, Visual Studio Community 2022 17.14.45, Windows SDK 26100 e Tauri CLI 2.12.6.

Dal prompt di sviluppo x64 di Visual Studio, nella radice del repository:

```bat
cargo build --locked --target x86_64-pc-windows-msvc -p shiftanything-tauri-proof --features windows-lifecycle
cd apps\shiftanything-tauri-proof
cargo tauri bundle --ci --no-sign --debug --target x86_64-pc-windows-msvc --bundles nsis --features windows-lifecycle --config tauri.nsis-qa.json
```

L'override locale usa ShiftAnything per i nomi Windows e Sh*ftAnything nel titolo della finestra di runtime. L'installer `ShiftAnything_0.20.6_x64-setup.exe` di 36.598.568 byte aveva SHA-256 `2cda912e160002574b09b4ed4cae620fd830eda08fd027377907d45141e41ecb` e firma Authenticode assente. Nessuna pubblicazione.

Nel profilo dell'Administrator di Audit Mode, senza installazioni o dati launcher iniziali, l'installer ha scritto la registrazione corrente e la scorciatoia. Il percorso scelto è stato `C:\sq\installed-launcher`, con profilo nativo sotto `%LOCALAPPDATA%\ShiftAnything\desktop`. Avvio dal wizard riuscito con catalogo 530 app e icona nativa. Il profilo debug apre anche una console, poi minimizzata per la schermata. La rimozione ha eliminato eseguibili e registrazione e conservato il dato sintetico. [Installer](installer.png) · [Catalogo](catalog.png) · [Rimozione](removed.png)

Questa prova copre il pacchetto del launcher. I cicli delle app del catalogo, l'autoaggiornamento Windows, la firma e SmartScreen restano verifiche distinte.
