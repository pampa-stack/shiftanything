# Piano: beta 0.21 di Sh*ftAnything

Fase 3/9, rivisto dopo [critica.md](critica.md) (lettura di Claude) e [critica-minimax.md](critica-minimax.md) (MiniMax M3, 1 Critical, 9 High, 11 Medium, 7 Low). Segue [spec.md](spec.md). Nessuna migrazione di dati; il ritorno indietro è il tag di rollback sul ramo predefinito e la release 0.20.9, che resta pubblicata.

## Pezzo → chi lo fa già

| Pezzo | Chi lo fa già (in questo repo) |
|---|---|
| Note e conteggio | `packaging/release-notes.py` |
| App, DMG firmato e notarizzato | `packaging/macos/local-app.sh`, `packaging/macos/release-dmg.sh` (`--check`, `--resume`) |
| Firme updater, `latest.json`, `.tar.gz` Linux | `packaging/package-tauri-updater.sh`, `packaging/linux/portable-tar.sh` |
| DEB Ubuntu | `packaging/linux/local-deb.sh` |
| Indirizzo di aggiornamento nel binario | `shiftanything --check-update-channel` (`startup.rs`) |
| Servire file in HTTPS nella rete privata | `tailscale serve` sull'AX42 (certificato `ax42.tail680f56.ts.net`) |
| Guest Linux di prova | `/var/tmp/shift-beta021-guest.gvBiii0o` sull'AX42 (`start.sh`) |
| Revisione di un'altra famiglia | la regia la lancia (Codex locale senza quota fino al 3 dicembre) |

Codice nuovo previsto: solo la scelta del profilo in `local-deb.sh` (passo 1) e un test sulla configurazione dell'updater (passo 1).

## Macchine

| Macchina | Ruolo | Spazio |
|---|---|---|
| Pro (questa chat, worktree all'albero `R`) | firma Developer ID, notarizzazione, `package-tauri-updater.sh`; mai una build completa | 11-12 GB liberi |
| Air (`ssh air`) | build `release` arm64 (0.21.0 e 0.20.9 di prova), prova Gatekeeper, prove di aggiornamento | 23 GB liberi |
| AX42 | build Linux nel guest Ubuntu 24.04, DEB, `.tar.gz`, `tailscale serve` | 309 GB |

## Ordine e SHA

La punta della PR #7 al momento delle build si chiama `R`, e da `R` escono tutti i pacchetti. Le catture della 0.21 **non** entrano nella PR #7: vanno in una PR successiva nel ramo predefinito e nella PR della landing, dopo la pubblicazione, come per la 0.20.9 (PR #8 e #9 di questo repo, #591 e #592 della landing). Così l'albero della PR non cambia dopo le build e il controllo `M^{tree} = R^{tree}` non fallisce per costruzione (High di MiniMax). Se dopo le build entra un altro commit nella PR, si rifanno le build da quella punta.

## Passi

Ogni passo si apre con i suoi controlli preliminari: se uno fallisce, ci si ferma lì.

1. **Versione e codice.**
   - Commit di versione: 0.21.0 in `Cargo.toml` (`[workspace.package]`), `Cargo.lock` e `apps/shiftanything-tauri-proof/tauri.conf.json`. Controllo: i tre valori uguali (`local-app.sh` e `local-deb.sh` leggono `Cargo.toml`, l'updater e `release-notes.py` leggono `tauri.conf.json`).
   - Commit `local-deb.sh`: scelta del profilo (`release` o `debug`, predefinito invariato), `shellcheck` verde. Il DEB della 0.20.9 conteneva un binario `release` (nessuna sezione `.debug`), messo nel percorso `debug`.
   - Commit di test: la configurazione che `signed.rs` inietta (`pubkey`, `requireSignedVersion`) si deserializza nella `Config` di `tauri-plugin-updater` 2.12.0 con `require_signed_version == true` (la `Config` non rifiuta chiavi sconosciute, `config.rs:170-176`).
   - `novita.md` da `git log db42c75..HEAD`.
   - Controlli nel guest: `cargo fmt --all --check`, clippy `-D warnings` e test di core, native, tauri-proof, `cargo xtask wasm`. Fallimenti preesistenti elencati a parte. Dopo il push, la punta è `R`.
2. **Note.** `python3 packaging/release-notes.py R --previous db42c75 --changes novita.md > release-notes.md`; uscita 0. Le note dicono «Codice usato per la build: R».
3. **Linux 0.21.0.** Nel guest, prima: `apt-get update`, pacchetti di sviluppo Tauri presenti. Da `R`: `cargo build --locked --release -p shiftanything-tauri-proof` con `SHIFT_LAUNCHER_UPDATE_ENDPOINT=https://github.com/pampa-stack/shiftanything/releases/latest/download/latest.json` e `SHIFT_LAUNCHER_UPDATE_PUBKEY` = campo «Chiave pubblica Tauri» di 1Password (uguale a quella della 0.20.9, ID `2507AFA05790C9FB`). `--version` = `shiftanything 0.21.0`; `--check-update-channel` = `linux-x86_64-deb` e l'indirizzo GitHub. DEB con `local-deb.sh` profilo `release`.
4. **Mac 0.21.0.**
   - Sull'Air, da `R`: `cargo build --locked --release -p shiftanything-tauri-proof` (nessun target universale) con le stesse due variabili; `lipo -archs` = `arm64`; `--check-update-channel` = `darwin-aarch64` e l'indirizzo GitHub.
   - Sul Pro, prima: worktree all'albero `R` (`local-app.sh` legge da lì `Cargo.toml`, icona, licenze, `NOTICE`, `ATTRIBUTION.md`); binario copiato in `CARGO_TARGET_DIR=$HOME/shift-021-target/release/shiftanything-tauri-proof`, SHA-256 uguale a quello sull'Air; `MACOS_SIGN_IDENTITY` (SHA-1 dell'identità Developer ID Application da `security find-identity -v -p codesigning`), `APPLE_TEAM_ID`, `MACOS_NOTARY_PROFILE` (profilo `notarytool` presente nel portachiavi del Pro); `spctl --status` = `assessments enabled`.
   - `release-dmg.sh --check`, poi `release-dmg.sh <cartella nuova>`. Se esce 75 (Apple non ha risposto in 45 minuti): `release-dmg.sh --resume <cartella>`, mai da capo. Controllo: `CFBundleShortVersionString` = 0.21.0 nell'app del DMG.
   - `$HOME/shift-021-target` si cancella alla fine.
5. **Updater e file della release (Pro).**
   - Prima: `cargo tauri signer sign --help` contiene `--app-version`.
   - Chiave: `d=$(mktemp -d)` con `umask 077` e `trap` che cancella `d`; `op-regia read` del campo «Chiave privata Tauri» rediretto in un file dentro `d`, mai su stdout, mai `tee`. Il campo password non esiste: la chiave è usata senza password, come per la 0.20.9.
   - `SHIFT_LAUNCHER_UPDATE_PUBKEY` impostata, così lo script (righe 140-162) rifiuta firme fatte con un'altra chiave.
   - `package-tauri-updater.sh 0.21.0 DMG HASH_DMG DEB HASH_DEB pampa-stack/shiftanything <cartella>`.
   - Cartella di staging con i 7 file della release, come la 0.20.9: DMG, `…macos-arm64.app.tar.gz` e `.sig`, `…linux-amd64.deb` e `.sig`, `…linux-amd64.tar.gz`, `latest.json`; più **un solo** `SHA256SUMS.txt` rigenerato su tutti e 7 (i due script ne scrivono uno ciascuno, incompleto).
   - `latest.json`: `version` 0.21.0, solo `darwin-aarch64` e `linux-x86_64-deb`.
6. **Prova di aggiornamento prima della pubblicazione.**
   - **Server.** Sull'AX42 una cartella con gli artefatti del passo 5 (hash confrontati con `SHA256SUMS.txt`) e una copia di `latest.json` con gli indirizzi verso `https://ax42.tail680f56.ts.net:8443/`, validata come JSON con firme identiche byte per byte all'originale (il suo hash cambia per forza). `tailscale serve status` prima; `tailscale serve --bg --https=8443 <cartella>`; `ss -ltnp | grep 8443` per vedere su quale indirizzo ascolta. Gli artefatti restano visibili alla sola tailnet per la durata della prova: lo si scrive nel rapporto.
   - **Raggiungibilità dal guest.** Il guest esce attraverso l'AX42 (rete utente di qemu): con una riga in `/etc/hosts` che porta il nome ts.net all'IP tailnet dell'AX42, `curl https://ax42.tail680f56.ts.net:8443/latest.json` dal guest deve rispondere con certificato valido. Se non risponde, al suo posto un server HTTPS locale con una CA di prova aggiunta solo al guest (`update-ca-certificates`), e si scrive quale strada è stata usata.
   - **0.20.9 di prova, Mac.** Sull'Air da `db42c75`, build `release` con `SHIFT_LAUNCHER_UPDATE_ENDPOINT=https://ax42.tail680f56.ts.net:8443/latest.json` e la stessa chiave pubblica. Sul Pro `local-app.sh` e firma `codesign --force --options runtime --timestamp --sign "$MACOS_SIGN_IDENTITY"`: `apply()` legge il `TeamIdentifier` dell'app installata (`signed.rs:144-153`) e senza firma Developer ID rifiuta l'aggiornamento (Critical di MiniMax). Installata sull'Air in `~/Applications` (cartella scrivibile, `signed.rs:122` e `168`). Controlli: `codesign -dv --verbose=4` mostra `TeamIdentifier`, `lipo -archs` = `arm64`, `--check-update-channel` = indirizzo AX42.
   - **0.20.9 di prova, Linux.** Nel guest, albero `db42c75` più il solo cambio di `local-deb.sh` del passo 1, versione 0.20.9, build `release` con l'indirizzo AX42, DEB con `local-deb.sh`, installato con `dpkg -i`. Controlli: `dpkg-query -W shiftanything` = 0.20.9, `/usr/bin/shiftanything --check-update-channel` = indirizzo AX42, `test -f /usr/bin/pkexec` (pacchetto `pkexec` installato esplicitamente), regola `/etc/polkit-1/rules.d/49-shift-qa.rules` che concede `org.freedesktop.policykit.exec` all'utente `qa`, provata con `pkexec true`; `apt-get -s --only-upgrade install -- <deb 0.21.0>` risolve senza scaricare nulla.
   - **Chi preme Aggiorna.** L'installazione parte solo dal pulsante della finestra (`frontend/native.js` 595-597, nessun comando da terminale). Nel guest: `xdotool` sul pulsante, con screenshot prima e dopo. Sull'Air: clic con AppleScript (System Events) dalla sessione dell'utente se l'accesso è concesso; altrimenti la regia fa il clic sull'Air. Si riporta lo stato dell'aggiornamento prima e dopo.
   - **Esito.** Su 2 macchine su 2: schermate prima e dopo lette, `--version` 0.21.0; su Mac anche `spctl` dell'app aggiornata = `Notarized Developer ID`.
   - **Chiusura.** `tailscale serve --https=8443 off` dentro un `trap`, `tailscale serve status` nel rapporto; build di prova cancellate.
   - La prova dimostra il flusso con una 0.20.9 ricompilata, non il binario pubblicato: quello si prova al passo 11.
7. **Gatekeeper sull'Air.** DMG copiato sull'Air e marcato come scaricato da Safari (`xattr -w com.apple.quarantine "0083;<ora esadecimale>;Safari;"`): `gh` e `curl` non mettono la quarantena. `xattr -p` la mostra, poi `spctl -a -t open --context context:primary-signature`, `xcrun stapler validate`, copia in Applicazioni, apertura senza avvisi, screenshot letto.
8. **Linux su un'altra distribuzione.** `.tar.gz` in un contenitore o guest Fedora pulito sull'AX42 con Xvfb, screenshot letto; i pacchetti Fedora necessari (webkit2gtk4.1, gtk3) vanno nel README portatile. Il `.tar.gz` non si aggiorna da solo: `installed_path` vuole il DEB in `/usr/bin`.
9. **PR pronta e revisione (fasi 6 e 7).** Punta della PR #7 = `R`. Revisione di un modello di un'altra famiglia sul diff `origin/f/pro-371-next-verification...R`, commento sulla PR con link, 0 problemi gravi.
10. **Merge (fase 9, solo con le 5 prove).** Tag `rollback/pre-0.21.0` sulla punta del ramo predefinito; `gh pr merge 7 --merge --match-head-commit R`; `git rev-parse R^{tree} M^{tree}` uguali, altrimenti si ricostruisce da `M` e si torna al passo 3.
11. **Release.**
    - `gh release create v0.21.0 --draft --target M --notes-file release-notes.md` con i 7 file e `SHA256SUMS.txt`.
    - Hash dei file caricati confrontati con la cartella di staging tramite l'API (`gh api …/releases/<id>/assets`), non con il download per tag, che non vede le bozze.
    - Pubblicazione come latest.
    - Gate dopo la pubblicazione: in anonimo `curl -sSL https://github.com/pampa-stack/shiftanything/releases/latest/download/latest.json` dà `version` 0.21.0 e 2 piattaforme.
    - Aggiornamento dal vivo dalla 0.20.9 pubblicata, installata da zero sull'Air e nel guest, con il trigger del passo 6.
12. **Catture e landing (dopo la release).** Catture IT/EN dalla 0.21.0 pubblicata; PR delle catture nel ramo predefinito di questo repo; PR nella landing come da [spec.md](spec.md#landing), ciascuna con le sue 5 prove.

## Punti della critica non accolti

- **Catture dentro la PR #7** (High, ordine R/catture/pacchetti): MiniMax propone build candidata, catture, poi pacchetti da `R`. Accolto il problema, cambiata la soluzione: le catture escono dalla PR #7 e vanno dopo la release, come per la 0.20.9. Una sola serie di build, nessuna notarizzazione ripetuta.
- **`docs/releasing.md` contraddice la procedura** (Low): è il documento dell'ArtCraft Launcher a monte; per Sh*ftAnything vale `packaging/RELEASE.md`. Lo si scrive nel rapporto, senza toccare un file ereditato in questa PR.

## Metriche e soglie per l'online

- `release-notes.py`: uscita 0, almeno 20 verifiche.
- Versione: `Cargo.toml`, `tauri.conf.json`, `Info.plist`, `dpkg-deb -f … Version`, `latest.json` tutti 0.21.0.
- Checksum: 7 file su 7 uguali tra staging, release caricata e `SHA256SUMS.txt`.
- `latest.json`: 2 piattaforme, firme con ID chiave `2507AFA05790C9FB`.
- Aggiornamento: 2 macchine su 2 da 0.20.9 a 0.21.0, prima (passo 6) e dopo (passo 11) la pubblicazione.
- `releases/latest/download/latest.json` anonimo = 0.21.0.
- Ritorno indietro se il passo 11 fallisce dopo la pubblicazione: `gh release edit v0.20.9 --latest` e la 0.21.0 in bozza; il ramo predefinito torna al tag di rollback solo con l'ok di MAIN (operazione distruttiva).

## Spesa

Nessuna: macchine già pagate, notarizzazione inclusa nell'account Apple, nessun servizio nuovo.
