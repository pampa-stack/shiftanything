# OBS Studio in Tauri su macOS Apple Silicon

La ricetta seleziona il DMG stabile `OBS-Studio-<versione>-macOS-Apple.dmg` pubblicato da OBS Project. Il launcher verifica checksum, firma, versione e identità del bundle. La ricetta è abilitata su macOS 13 o successivo con architettura arm64.

| Versione | Byte DMG | SHA-256 | Sorgente |
| --- | ---: | --- | --- |
| 33.0.2 | 209.592.273 | `f9cd96511c5efdd3ffd9a0068e3bbc276c61fc451d1004e3eea5658a9cb70488` | [Release ufficiale](https://github.com/obsproject/obs-studio/releases/tag/33.0.2) |
| 33.0.3 | 209.590.030 | `92212873b6f84c54285203cac3f846590ac00dfad76b312cf266100ff5edf0c8` | [Release ufficiale](https://github.com/obsproject/obs-studio/releases/tag/33.0.3) |

I DMG corrispondono agli hash pubblicati e sono stati montati in sola lettura. Entrambe le app hanno superato `codesign --verify --deep --strict --all-architectures` e Gatekeeper. I bundle contengono un eseguibile arm64, identificatore `com.obsproject.obs-studio` e firma `Wizards of OBS LLC`, Team ID `2MMRE5MTB8`.

In un profilo temporaneo vergine, **Installa** dalla scheda OBS Studio e **Approva** hanno scaricato il DMG ufficiale e installato 33.0.3. Il backend ha rilevato la versione, riletta anche dal bundle e verificata con `codesign`.

Nel secondo profilo, **Le mie app** ha mostrato 33.0.2 installata e 33.0.3 disponibile. **Aggiorna** e **Approva** hanno completato l'aggiornamento riusando il DMG della cache temporanea, con ulteriore verifica del checksum e della firma. Entrambi i file sintetici del profilo, incluso `obs-studio/basic/profiles/ShiftProof/basic.ini`, sono rimasti intatti.

L'eseguibile installato ha restituito la versione corretta anche con `--version` per 33.0.2 e 33.0.3. La sandbox ha rifiutato un tentativo di connessione di prova e una scrittura esterna al profilo. `--version` termina prima di aprire la finestra e prima dei controlli sui permessi di cattura. [Entrypoint OBS Studio](https://github.com/obsproject/obs-studio/blob/33.0.3/frontend/obs-main.cpp#L1063).

**Apri** ha mostrato la richiesta nel launcher. Il rifiuto è stato registrato dal backend. L'avvio grafico tramite LaunchServices resta da provare in un account di test o in VM per isolare i permessi macOS di fotocamera, audio, schermo e input.

**Rimuovi** e **Approva** hanno spostato `OBS.app` 33.0.3 nel cestino temporaneo. L'inventario ha rilevato l'assenza dell'app. Entrambi i file di prova sono rimasti invariati.

Per preparare un profilo di prova con il DMG ufficiale 33.0.2 già scaricato:

```sh
cargo build --offline -p shiftanything-tauri-proof
run_dir="$(mktemp -d /private/tmp/shift-obs.XXXXXX)"
target/debug/shiftanything-tauri-proof "$run_dir/update-profile" it --lifecycle \
  --seed-obs-previous /percorso/OBS-Studio-33.0.2-macOS-Apple.dmg
```

Il seed verifica i byte, l'identità e la firma dell'app e richiede un profilo mai usato. I log di prova sono sotto `/private/tmp/shift-obs-mac-packages.5JAjbbq9`.

Build, `cargo fmt --all --check` e Clippy sono passati. Test Rust su target isolato con cache clonata, 118 core, 549 native, 3 Tauri, 8 ignorati preesistenti.
