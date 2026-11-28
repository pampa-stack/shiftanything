# Release macOS di Sh*ftAnything

Usa un Mac con Xcode e un certificato **Developer ID Application** valido, con la chiave privata nel portachiavi. Prepara e convalida un profilo locale `notarytool` del team usando la procedura Apple `store-credentials`. `MACOS_SIGN_IDENTITY` è il fingerprint SHA-1 di 40 caratteri del certificato, `APPLE_TEAM_ID` è l'ID del team e `MACOS_NOTARY_PROFILE` è il nome del profilo. Gli eventuali `MACOS_SIGN_KEYCHAIN` e `MACOS_NOTARY_KEYCHAIN` sono percorsi assoluti dei portachiavi, con lo stesso perimetro usato per configurare l'identità e il profilo. [Firma Developer ID](https://developer.apple.com/developer-id/) · [Procedura notarytool](https://developer.apple.com/documentation/security/customizing-the-notarization-workflow)

Dalla radice del repo, con le tre variabili impostate:

```bash
packaging/macos/release-dmg.sh --check
cargo build --locked --release -p shiftanything-tauri-proof
packaging/macos/release-dmg.sh /percorso/nuova-cartella-release
```

`--check` controlla identità, team, presenza della voce del profilo nel portachiavi e Gatekeeper. Il controllo è locale e la validità delle credenziali del profilo si verifica quando lo si configura con Apple. Il confezionamento riusa la struttura `.app` Tauri di `local-app.sh` e usa il binario `release` di `CARGO_TARGET_DIR` se impostato. Il nome del DMG indica le architetture effettive del binario. Per un DMG ARM64 compila su Apple Silicon senza cambiare il target Cargo. Il Mac incaricato deve poter eseguire il binario per il controllo di versione.

Lo script firma app e DMG con timestamp, abilita il runtime hardened nell'app e applica i ticket dopo le risposte `Accepted`. Verifica il DMG con Gatekeeper, monta l'immagine in sola lettura e controlla i permessi nel mount e nella copia. Sulla copia ripete Gatekeeper ed esegue `--version`. Solo dopo consegna localmente il DMG e il suo hash. I file `notary-app-submission.json` e `notary-dmg-submission.json` salvano gli ID prima dell’attesa. Se Apple non risponde entro 45 minuti, lo script conserva anche gli artefatti firmati nella cartella di lavoro indicata. Usa `notarytool info` con l’ID salvato per controllare la richiesta già inviata. [Distribuzione macOS](https://developer.apple.com/documentation/xcode/packaging-mac-software-for-distribution)

Il volume Finder mostra Sh*ftAnything e il collegamento ad Applications. Per rigenerare la grafica e il layout usa `python3 packaging/macos/dmg/generate.py --shiftanything` con le dipendenze indicate nella guida [DMG](dmg/README.md). La verifica di rilascio rifiuta file estranei nel volume.
