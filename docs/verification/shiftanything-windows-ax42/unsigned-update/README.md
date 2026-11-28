# Setup Windows non firmato e aggiornamento firmato

28 novembre 2026, Windows 11 Enterprise Evaluation x64 (build 26100) nella VM di prova su AX42, con rete NAT. Sorgente `46b90cf24317b2da3c964484485606c8725a60e2` del ramo `f/windows-non-firmato`. Toolchain del guest: Rust 1.95.0 MSVC, Visual Studio Community 2022 17.14.45, Windows SDK 26100, Tauri CLI 2.12.6, NSIS 3.13. Nessuna release pubblicata, nessuna firma Authenticode, nessuna spesa.

## Build

`packaging/windows/release-nsis.ps1 -Authenticode off` ha compilato due setup dallo stesso commit. La seconda copia porta la versione a 0.21.0 solo in `Cargo.toml`, `tauri.conf.json` e `Cargo.lock`, per avere un aggiornamento da installare. Entrambe usano un canale di aggiornamento di prova e una chiave updater usa-e-getta, generata per questa prova e mai entrata nel repository né in un manifest pubblico.

| Setup | Byte | SHA-256 | Authenticode |
| --- | --- | --- | --- |
| `ShiftAnything-0.20.9-windows-x64-setup.exe` | 46.949.970 | `49eefbb156e9ca21998de32ba2eb207f1ae62044010ebd5422dca98316cb9a09` | NotSigned |
| `ShiftAnything-0.21.0-windows-x64-setup.exe` | 46.949.411 | `67524865210891731f48a154159eb84e3516fdbaaff1f5ad9b9ac65f7ef0b3c2` | NotSigned |

Lo script ha controllato prodotto e versione di setup ed eseguibile e ha scritto `UNSIGNED: Authenticode is off`. Prima della build, nel guest: `artcraft-launcher-core` 181 test passati (1 ignorato), `artcraft-launcher-native` 593 passati (4 ignorati), `shiftanything-tauri-proof` con `windows-lifecycle` 25 passati.

Sul Mac, `package-tauri-updater.sh` ha preso il DMG e il DEB 0.20.9 pubblicati e il setup 0.20.9 e ha prodotto un `latest.json` con `darwin-aarch64`, `linux-x86_64-deb` e `windows-x86_64`, firme verificate e `SHA256SUMS.txt` controllato. Anche qui la chiave era quella usa-e-getta e l'output è rimasto locale. Lo script rifiuta un DEB passato come setup e un setup con versione diversa da quella richiesta.

## Download con Edge

Il setup 0.20.9 era servito da un tunnel Cloudflare gratuito (`https://strikes-stuart-quantitative-fusion.trycloudflare.com`), con una cartella che conteneva solo i file di prova. Il tunnel è stato chiuso alle 19:42 UTC dopo l'ultimo download, e da fuori ha risposto 530. Il nome del file è in minuscolo perché la tastiera virtuale della VM non trasmette le maiuscole nella barra degli indirizzi.

Edge segnala il file: «isn't commonly downloaded», poi «Microsoft Defender SmartScreen couldn't verify if this file is safe», «Publisher: Unknown». L'utente sceglie Keep, poi Keep anyway, e il setup si apre. [Avviso di Edge](01-edge-download-warning.png) · [Conferma di SmartScreen in Edge](02-edge-smartscreen-keep.png) · [Setup aperto](03-setup-opened.png)

La schermata blu «Windows ha protetto il PC» non è comparsa in questa VM. Il file salvato da Edge non aveva il flusso `Zone.Identifier` (Mark of the Web), né in Audit Mode né dopo l'uscita in OOBE. Una copia con il Mark of the Web aggiunto a mano e aperta da Esplora file è partita anch'essa senza avviso. Nella VM Smart App Control è in valutazione (`VerifiedAndReputablePolicyState` = 2), e durante la prova il criterio `ShellSmartScreenLevel` era impostato su `Warn`, il valore predefinito. Su un PC con Smart App Control attivo un setup senza firma viene bloccato senza «Esegui comunque»: è un limite noto dei pacchetti non firmati, che la firma Authenticode risolverebbe. La riga dei README resta valida per i PC con SmartScreen classico, ma questa VM non l'ha dimostrata.

## Installazione, aggiornamento e rimozione

Utente standard `ShiftQa`, nessuna elevazione. Il setup ha installato in `%LOCALAPPDATA%\ShiftAnything` e registrato `HKCU\...\Uninstall\ShiftAnything` con `DisplayVersion` 0.20.9, `MainBinaryName` `ShiftAnything.exe` e `InstallLocation` uguale alla cartella dell'eseguibile, cioè i valori che il launcher controlla prima di aggiornarsi. Catalogo aperto con 591 app. [Fine installazione](04-install-complete.png) · [Catalogo](05-catalog.png)

Con PhotoCraft nei preferiti e un file sintetico in `%LOCALAPPDATA%\ShiftAnything\desktop`, il launcher ha trovato 0.21.0 nel `latest.json` di prova. Installa ha scaricato il setup, verificato firma e versione firmata, avviato NSIS in modalità passiva `/UPDATE` e riaperto il launcher. Dopo: `DisplayVersion` 0.21.0, processo `ShiftAnything.exe` 0.21.0 avviato dall'utente standard, preferiti intatti. [Aggiornamento disponibile](06-update-available.png) · [Setup passivo](07-update-passive-setup.png) · [Launcher riaperto](08-relaunched.png)

Prova negativa: un `latest.json` che annuncia 0.21.1 con la firma della 0.21.0 viene rifiutato con «Download interrupted or invalid signature». Il launcher resta su 0.21.0. [Versione non firmata rifiutata](09-forged-version-rejected.png)

La rimozione con l'opzione predefinita («Delete the application data» non spuntata) ha tolto eseguibile, uninstaller, collegamento e chiave di registro. `favorites.json` e il file sintetico sono rimasti, con lo stesso SHA-256 di prima. [Rimozione](10-uninstall-keep-data.png) · [Rimozione completata](11-uninstall-complete.png)

## Stato della VM

Per la prova SmartScreen la VM è uscita da Audit Mode con `sysprep /oobe` e un account amministratore locale di prova. Prima è stato salvato lo snapshot interno `audit-mode-2026-11-28` del disco, con copia di `vars.fd` e dello stato TPM in `backup-audit-mode-2026-11-28`, per tornare allo stato precedente.
