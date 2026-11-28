I'll start by reading the specs and then verify them against the real code.
Now let me read the real scripts and code.
Ho letto spec, plan, critica e i sorgenti reali (script di packaging, `launcher_update/signed.rs`, `startup.rs`, `launcher_update.rs`, frontend, e il sorgente di `tauri-plugin-updater` 2.12.0 in `~/.cargo/registry/src/.../tauri-plugin-updater-2.12.0`).

## Verifiche chieste, esito

- **`requireSignedVersion` esiste in 2.12.0**: `tauri-plugin-updater-2.12.0/src/config.rs:135` (`require_signed_version`, con `#[serde(rename_all = "camelCase")]` e alias `require-signed-version`). Il passo 12 del piano è risolto; la riga giusta in `signed.rs` è **56**, non 54.
- **Versione corrente dell'updater**: `updater.rs:199` prende `app.package_info().version` → cioè **`tauri.conf.json`**, non `Cargo.toml`. `release-notes.py:71` idem. `Cargo.toml` serve solo a `local-app.sh:45` e `local-deb.sh:11` (Info.plist e versione DEB). Le tre copie devono essere 0.21.0 o lo stesso SHA.
- **TLS**: `reqwest 0.13` con feature `rustls-no-provider` → `rustls-platform-verifier` (root di sistema). La pretesa della spec è vera.
- **Firma**: il plugin fa `Signature::decode` sul file `.sig` a 4 righe intero (`updater.rs:1541`) e verifica la versione nel `trusted comment` (`updater.rs:740-745`); `package-tauri-updater.sh:154` controlla la stessa cosa lato produttore. Coerente.

---

## CRITICAL

**[CRITICAL] La prova di aggiornamento macOS non può partire: la 0.20.9 di prova deve essere firmata con l'identità Developer ID**
`apps/shiftanything-tauri-proof/src/launcher_update/signed.rs:144-153` — `apply()` legge `TeamIdentifier=` dalla **app installata** e pretende che la nuova app soddisfi la stessa requirement. `packaging/macos/local-app.sh:64` firma ad-hoc (`codesign --force --sign -`), quindi `codesign -dv --verbose=4` non stampa `TeamIdentifier=` → `Err("installation_unsupported")`: l'aggiornamento non parte mai. Il piano (passo 6) non dice che la 0.20.9 di prova va firmata con Developer ID (il certificato sta solo nel portachiavi del Pro).
*Correzione:* al passo 6 aggiungere: assemblare la 0.20.9 di prova sul Pro con `local-app.sh --tauri --release`, firmarla `codesign --force --sign "$MACOS_SIGN_IDENTITY" --options runtime --timestamp`, copiare la `.app` sull'Air, installarla in una cartella scrivibile dall'utente. Nel rapporto: output di `codesign -dv --verbose=4` della 0.20.9 di prova con `TeamIdentifier` presente, e `lipo -archs` = `arm64`.

## HIGH

**[HIGH] Albero del merge confrontato con `R` invece che con la punta della PR: ricostruzione dopo il merge**
`specs/beta-021/spec.md:26` impone `albero(merge) == albero(punta PR)`; `plan.md:30` definisce `R` = commit di versione, `plan.md:39` mette la punta PR = `R` + catture, `plan.md:42` confronta `M^{tree}` con `R^{tree}`. Le tre cose non possono stare insieme: con le catture dentro la PR il check fallisce per costruzione e il piano prevede di rifare DMG, notarizzazione, firme e DEB **dopo** il merge.
*Correzione:* fissare l'ordine 1) commit versione → 2) build candidata → 3) catture → 4) commit catture (punta PR = `R`) → 5) **pacchetti e firme costruiti da `R`** → 6) merge e check `git rev-parse M^{tree}` vs `git rev-parse R^{tree}`. Le note devono citare `R` = punta PR.

**[HIGH] Il piano non costruisce né installa il DEB della 0.20.9 di prova: la prova Linux rischia di girare sul binario pubblicato**
`signed.rs:171-186` pretende `current_exe()` == `/usr/bin/shiftanything`, `dpkg-query -S` == `shiftanything: /usr/bin/shiftanything` e versione installata **esattamente** uguale a `CARGO_PKG_VERSION` del binario in esecuzione. Se si installa il DEB pubblicato 0.20.9 e si lancia la build di prova (endpoint AX42), versione e binario non coincidono → `installation_unsupported`; se si installa la build di prova senza dirlo, l'endpoint è GitHub e la prova non testa nulla.
*Correzione:* passo 6 = tree di prova = `db42c75` + **solo** il fix di `local-deb.sh` (versione rimasta a 0.20.9 in `Cargo.toml` **e** `tauri.conf.json`), build `release` col profilo `release`, DEB via `local-deb.sh`, installato nel guest con `dpkg -i`. Verificare `dpkg-query -W shiftanything` = `0.20.9` e `--check-update-channel` = URL AX42 **prima** di toccare l'update.

**[HIGH] Le variabili obbligatorie di `release-dmg.sh` non compaiono nel piano**
`packaging/macos/release-dmg.sh:33-35` esce subito se mancano `MACOS_SIGN_IDENTITY` (SHA-1, 40 hex), `APPLE_TEAM_ID` (10 char) o `MACOS_NOTARY_PROFILE`; `:49-58` pretende che l'identità Developer ID e il profilo `notarytool` esistano **nel portachiavi del Pro** (`store-credentials`); `:61` pretende Gatekeeper assessments attivo. Il piano (passo 4) dice solo "`release-dmg.sh --check`, poi `release-dmg.sh`".
*Correzione:* nominare le tre variabili + la chiave (il profilo `notarytool` esiste già? verificare con `security find-generic-password -l com.apple.gke.notary.tool -a "com.apple.gke.notary.tool.saved-creds.$PROFILE"`); `--check` è solo preflight, non verifica il DMG.

**[HIGH] Il Pro deve avere l'albero `R`, non solo il binario**
`release-dmg.sh:210` chiama `local-app.sh`, che legge `$ROOT/Cargo.toml` (`:45`) e `$ROOT/assets/shiftanything/icon.png`, `LICENSE-*`, `NOTICE`, `ATTRIBUTION.md` (`:31-34`). Con il checkout del Pro a 0.20.9, l'`Info.plist` dice 0.20.9 → il DMG si chiama `ShiftAnything-0.20.9-macos-arm64.dmg` e contiene il binario 0.21.0; `package-tauri-updater.sh:97` poi fallisce **dopo** la notarizzazione.
*Correzione:* step 4 = sul Pro, checkout di `R` (o copia di `Cargo.toml`, `assets/shiftanything/`, `LICENSE-*`, `NOTICE`, `ATTRIBUTION.md` da `R`); prima di `release-dmg.sh`, controllare `plutil -extract CFBundleShortVersionString` = 0.21.0 e `ShiftAnything --version` = `shiftanything 0.21.0`.

**[HIGH] Il DMG non è nella lista di upload del passo 13**
`plan.md:42` carica «i file del passo 5» = `package-tauri-updater.sh:174-177` = 7 file (mac `.app.tar.gz` + `.sig`, `.deb` + `.sig`, `.tar.gz` Linux, `SHA256SUMS.txt`, `latest.json`). **Il DMG non c'è**: sta nella cartella di `release-dmg.sh` (`release-dmg.sh:286`). La release pubblicata resterebbe senza DMG e il download anonimo dalla landing (criterio 9) fallirebbe.
*Correzione:* elenco di upload esplicito = DMG (da `release-dmg.sh`) + i 7 file del passo 5 + `release-notes.md`, con i nomi esatti che `package-tauri-updater.sh:44-46` scrive negli URL di `latest.json:163`.

**[HIGH] Due `SHA256SUMS.txt` con lo stesso nome e copertura incompleta**
`release-dmg.sh:284` scrive un `SHA256SUMS.txt` con **solo** il DMG; `package-tauri-updater.sh:170` ne scrive uno con **6** file (mac tar.gz, mac .sig, deb, deb .sig, tar.gz Linux, latest.json) — DMG e il file stesso esclusi. I due non possono coesistere nella stessa release e la metrica «8 file su 8 uguali… e `SHA256SUMS.txt`» (`plan.md:47`) è irraggiungibile con il file reale.
*Correzione:* caricare **un solo** `SHA256SUMS.txt`, generato dopo aver riunito tutti gli 8 file (DMG incluso) nella cartella di staging, e confrontare quello; oppure due nomi distinti (`SHA256SUMS.txt` + `SHA256SUMS-dmg.txt`) e la metrica corrected a «6 su 6 + DMG con hash proprio».

**[HIGH] Il guest Linux probabilmente non raggiunge il listener di `tailscale serve`**
`tailscale serve` pubblica sul **IP tailnet** del nodo (100.x), non sull'interfaccia LAN; la riga in `/etc/hosts` (`plan.md:35`) risolve il nome ma non fa raggiungere il listener, e senza Tailscale nel guest manca anche MagicDNS.
*Correzione:* prima della prova, sul AX42 `ss -ltnp | grep 8443` per vedere l'indirizzo di bind; se è solo l'IP tailnet, o si installa Tailscale nel guest, o si serve in locale con TLS e una CA aggiunta al trust store del guest (`/usr/local/share/ca-certificates` + `update-ca-certificates`, `SSL_CERT_FILE` è impostato dal plugin, `updater.rs:441-449`). Verificare prima che il certificato sia emittibile: `tailscale cert ax42.tail680f56.ts.net` sul AX42.

**[HIGH] Il piano non dice chi/come si preme «Installa»**
`apps/shiftanything-tauri-proof/frontend/native.js:595-597` scatena `launcher_update_check` / `launcher_update_install` solo dal bottone della UI; non esiste un CLI per l'installazione (`startup.rs:22-29` ha solo `--version` e `--check-update-channel`). Nel guest con Xvfb nessuno clicca.
*Correzione:* definire il trigger: click manuale con screenshot sull'Air, e nel guest un click sintetico (xdotool) o un percorso documentato; riportare lo stato `launcher_update_state` prima/dopo.

**[HIGH] Nessuna verifica che `releases/latest` restituisca il manifest 0.21.0**
L'endpoint compilato nella 0.20.9 pubblicata è `…/releases/latest/download/latest.json`: se la 0.21.0 non diventa la "latest", la prova post-pubblicazione (passo 13) non trova aggiornamenti e il rollback (`plan.md:51`) non serve a nulla.
*Correzione:* dopo la pubblicazione, in anonimo e senza token: `curl -sSL https://github.com/pampa-stack/shiftanything/releases/latest/download/latest.json` → `version` = 0.21.0 e due sole piattaforme. Aggiungerlo come gate del passo 13.

## MEDIUM

**[MEDIUM] `requireSignedVersion` può essere ignorato in silenzio** — `tauri-plugin-updater-2.12.0/src/config.rs:170-176`: la `Config` personalizzata non ha `deny_unknown_fields` → una chiave scritta male (es. `require_signed_version`) verrebbe **accettata e ignorata**, disattivando il legame versione/firma senza errore. *Correzione:* test automatico che deserializzi `{pubkey, requireSignedVersion:true}` nella `Config` del plugin e asserisca `require_signed_version == true`; spostare il controllo del passo 12 al passo 1.

**[MEDIUM] La copia di `latest.json` servita non può corrispondere a `SHA256SUMS.txt`** — `plan.md:35` dice «hash controllati» sulla cartella servita, ma lì `latest.json` è **riscritto** con gli URL AX42 → l'hash non è più quello di `SHA256SUMS.txt:170`. *Correzione:* confrontare gli hash degli **artifact** (6 file) con `SHA256SUMS.txt`, e validare `latest.json` come JSON (`version` = 0.21.0, due piattaforme, `signature` identica byte per byte a quella di `package-tauri-updater.sh`), non per hash.

**[MEDIUM] Il rollback del `tailscale serve` è fragile** — la configurazione di `tailscale serve` è **persistenza** di `tailscaled`: se la sessione muore prima di `tailscale serve --https=8443 off` (`plan.md:35`), il AX42 continua a servire gli artefatti 0.21.0 a tutto il tailnet. *Correzione:* `trap`/passo esplicito di spegnimento e `tailscale serve status` nel rapporto finale; alternativa: servire su una porta non standard e toglierla subito.

**[MEDIUM] La versione deve coincidere in tre file, senza controllo automatico** — `Cargo.toml` (`local-app.sh:45`, `local-deb.sh:11`), `tauri.conf.json` (`release-notes.py:71`, `updater.rs:199`), versione nel `latest.json`. Una disuguaglianza tra `Cargo.toml` e `tauri.conf.json` dà: DMG rifiutato da `package-tauri-updater.sh:97`, oppure l'app 0.21.0 che si offre aggiornamento a sé stessa (`updater.rs:578`). *Correzione:* un check nel passo 1 che confronti i tre valori.

**[MEDIUM] Il build Linux non può essere "aarch64 + x86_64"** — `package-tauri-updater.sh:99` pretende `lipo -archs` = `arm64` esatto; un build universale lo rifiuta e `release-dmg.sh:223` nominerebbe il DMG `macos-universal.dmg`, rompendo i nomi attesi da `latest.json` e dalla landing. *Correzione:* nell' Air `cargo build --release -p shiftanything-tauri-proof` senza target universale; verificare `lipo -archs` = `arm64` prima di `release-dmg.sh`.

**[MEDIUM] `pkexec` è solo un `Recommends`, e nel guest potrebbe mancare** — `local-deb.sh:51`; `signed.rs:175` pretende `/usr/bin/pkexec` come file, altrimenti `installation_unsupported`. *Correzione:* installare `policykit-1` esplicitamente nel guest e verificare `test -f /usr/bin/pkexec` prima della prova.

**[MEDIUM] `apt-get --only-upgrade install -- <file.deb>` va provato a secco** — `signed.rs:206-211`. *Correzione:* prima della prova reale, `apt-get -s --only-upgrade install -- <deb>` nel guest e verificare che l'upgrade 0.20.9 → 0.21.0 risolva senza scaricare nulla (dipendenze già presenti: `local-deb.sh:50`).

**[MEDIUM] Nel guest servono gli indici apt e le dipendenze di compilazione** — `dpkg-deb` con `Depends` (`local-deb.sh:50`) pretende `libwebkit2gtk-4.1-0`, `libgtk-3-0t64`, `libjavascriptcoregtk-4.1-0`, `libsoup-3.0-0`; apt le risolve solo dagli indici locali. *Correzione:* nel guest `apt-get update` prima, e i dev package Tauri (`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libsoup-3.0-dev`, `pkg-config`) per la build `release` del passo 3.

**[MEDIUM] La regola polkit va scritta con l'action id giusto** — per `pkexec` l'action è `org.freedesktop.policykit.exec`, e serve un file JS in `/etc/polkit-1/rules.d/` che restituisca `polkit.Result.YES` per `subject.user == "qa"`. *Correzione:* scrivere il file esatto nel rapporto e verificare con `pkexec --version`/`pkexec true` **prima** della prova, così un errore di policy non si confonde con un errore di aggiornamento.

**[MEDIUM] La CLI Tauri deve supportare `--app-version`** — `package-tauri-updater.sh:39-40` fallisce se `tauri signer sign --help` non contiene `--app-version`; è il presupposto del legame versione/firma (`package-tauri-updater.sh:125,154`). *Correzione:* verificare sul Pro prima del passo 5 (`cargo tauri signer sign --help | grep -- --app-version`).

**[MEDIUM] Come si estrae in modo deterministico la pubkey dal binario 0.20.9** — `signed.rs:9` la compila dentro il binario come stringa base64; `strings` può fondere letterali adiacenti nel rodata. *Correzione:* estrarre con `strings -a -n 64 <binario> | grep -oE 'RW[A-Za-z0-9+/]{60,}'`, base64-decodificare, verificare con `minisign-verify` che il key id (byte 2..10 del record da 42 byte) sia **uguale** a quello delle firme `.sig` (record da 74 byte, byte 2..10) — lo stesso controllo che fa `package-tauri-updater.sh:147-162`.

**[MEDIUM] Il passo 13 non prevede il `--resume` se Apple non risponde in 45 minuti** — `release-dmg.sh:131,140` esce con **75** e conserva stato e ID. *Correzione:* nel piano, al passo 4: se l'uscita è 75, `release-dmg.sh --resume <cartella>` (l'id è in `notary-*-submission.json`), e non rilanciare da capo.

**[MEDIUM] `gh release download` su una bozza** — l'endpoint per tag non restituisce le bozze; il percorso robusto è l'id della release. *Correzione:* scaricare via API (`gh api repos/pampa-stack/shiftanything/releases/<id>/assets/<nome>`) o confrontare gli hash **prima** del caricamento (la cartella di staging è già la sorgente), e usare il download dalla bozza solo come conferma con il percorso API.

**[MEDIUM] La prova dell'aggiornamento richiede scrittura sulla cartella dell'app** — `signed.rs:122` (`tempfile::tempdir_in(parent)`) e `signed.rs:168` (`renameat_with(RENAME_EXCHANGE)`) scrivono nella cartella che contiene l'app. *Correzione:* installare la 0.20.9 di prova in `/Applications` **di proprietà dell'utente** o in `~/Applications`, e verificare i permessi prima.

## LOW

**[LOW] `docs/releasing.md` contraddice la procedura usata** — descrive l'ArtCraft Launcher upstream (`artcraft-launcher-*`, ramo `release`, workflow che pubblica). *Correzione:* una riga in `packaging/RELEASE.md` o nel rapporto: per Sh*ftAnything vale `packaging/RELEASE.md`; `docs/releasing.md` è ereditato.

**[LOW] Riferimento di riga sbagliato** — `plan.md:41` dice `signed.rs` riga 54 per `requireSignedVersion`; è **56** (`require_signed_version` è a `config.rs:135` del plugin).

**[LOW] Le build dovrebbero usare `--locked`** — `packaging/macos/README.md:9` prescrive `cargo build --locked --release`; senza `--locked` un `Cargo.lock` divergente passa inosservato. *Correzione:* usare `--locked` nelle build dei passi 3, 4 e 6.

**[LOW] Il piano non include il passo `wasm` di `cargo xtask ci`** — `xtask/src/main.rs:98-105` esegue fmt, clippy, test **e** wasm; la lista del criterio 9 (`spec.md:53`) li copre solo in parte. *Correzione:* eseguire `cargo xtask ci` (o aggiungere `cargo xtask wasm`) nella checklist del passo 1.

**[LOW] Il piano non confronta i file impacchettati, solo i README** — `plan.md:39` ammette la deviazione se `git diff --stat R HEAD` tocca solo `README*` e `docs/screenshots/`, ma dentro la `.app` finiscono anche `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md` (`local-app.sh:32-34`) e `assets/shiftanything/icon.png`. *Correzione:* l'elenco dei file "impacchettati" da confrontare comprende questi.

**[LOW] La chiave privata non deve transitare da stdout** — `plan.md:34` dice «scritto da `op-regia` senza mostrarne il contenuto»: la forma sicura è `op read … > "$keyfile"` con `umask 077`, non `tee`/`echo`. *Correzione:* comandare esplicitamente la forma con redirect e senza `tee`, e cancellare `keyfile` + `mktemp -d` con `trap`.

**[LOW] Gli artefatti 0.21.0 sono pubblici per tutto il tailnet durante la prova** — `tailscale serve` li espone a chiunque sia nella tailnet, prima della pubblicazione. *Correzione:* accettarlo e dirlo nel rapporto, oppure usare una ACL del tailnet.

---

Nessun file toccato.
