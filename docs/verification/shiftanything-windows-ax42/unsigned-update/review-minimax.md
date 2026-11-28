I'll start by examining the repository structure and the PR diff.
## Esito

Nessun difetto **Critical**. La catena di firma dell'aggiornamento è solida (verificata a fondo, vedi in fondo). Restano **1 High, 5 Medium, 7 Low**. Il rischio principale non è la firma: è la catena di rilascio Windows (chiave di firma, prerequisiti, copertura CI).

---

## HIGH

### H1 — Il JAR di jsign non è verificato e riceve la password del keystore
`packaging/windows/jsign-sign.ps1:10-14`

**Scenario.** `SHIFT_WINDOWS_SIGN_JSIGN` punta a un percorso arbitrario; `java.exe -jar` lo esegue con l'ambiente ereditato, quindi un JAR sostituito (o un file JAR plantato sul disco della macchina di build, o un semplice cambio della variabile) legge `SHIFT_WINDOWS_SIGN_STOREPASS` e la chiave. L'indirezione `env:` protegge solo `argv`/`ps`, non il processo figlio. `RELEASE.md:20` chiede solo una verifica manuale dello SHA-256, non eseguita dallo script.

**Correzione.** Aggiungere `SHIFT_WINDOWS_SIGN_JSIGN_SHA256` (pin del JAR ufficiale) e verificare l'hash prima di `java -jar`; in alternativa non far ereditare il segreto al processo (password via file/tmpfs o letta da un helper che non espone l'ambiente).

---

## MEDIUM

### M1 — I prerequisiti documentati non bastano: manca `signtool.exe` (Windows SDK)
`packaging/RELEASE.md:6` e `:22`

**Scenario.** `tauri build` chiama `should_sign()` → `verify()` → `signtool verify /pa <exe>` per ogni .exe (`tauri-bundler/src/bundle/windows/sign.rs`); senza l'SDK la build abortisce con `SignToolNotFound`. `verify-release-signature.ps1` richiede `signtool.exe` (`Get-Command -CommandType Application`) e fallisce con un errore non riconducibile alla causa. Il documento cita solo "Rust MSVC, Visual Studio Build Tools e Tauri CLI" e, per il passaggio 4, solo Java+jsign.

**Correzione.** Elencare l'SDK Windows (o `signtool.exe` sul PATH) fra i prerequisiti, in entrambi i passaggi.

### M2 — Il comportamento SmartScreen descritto non è quello verificato
`packaging/RELEASE.md:13` vs `docs/verification/shiftanything-windows-ax42/unsigned-update/README.md`

**Scenario.** RELEASE.md afferma come fatto «Al primo avvio SmartScreen mostra "Windows ha protetto il PC"», mentre la verifica del PR dice l'opposto: la schermata non è comparsa, `ShellSmartScreenLevel` era `Warn` e Smart App Control era solo "in valutazione" (`VerifiedAndReputablePolicyState` = 2). Le note di rilascio finiscono così per insegnare all'utente una procedura che sulla macchina di prova non si è mai vista, mentre sui PC con Smart App Control attivo il setup è bloccato senza rimedio.

**Correzione.** Riscrivere come "può mostrare" e tenere la caveat Smart App Control nella stessa frase; usare la formulazione già pronta nelle righe IT/EN citate nella stessa riga.

### M3 — Il codice Windows nuovo non è compilato né testato da nessuna CI
`.github/workflows/ci.yml:41-42`, `apps/shiftanything-tauri-proof/build.rs:8-13`

**Scenario.** `cfg(signed_updates)` su Windows dipende da `windows-lifecycle`; nessun job CI lo abilita (`cargo clippy --workspace --all-targets` e `cargo test --workspace` girano senza feature). Quindi `installed_path()`, `finish()`, il ramo Windows di `install()` e il `Staged` con i suoi campi non vengono mai compilati da clippy né testati: un errore di compilazione o un lint su quel codice emerge solo sulla macchina di rilascio. I test `windows_update_must_be_an_nsis_setup` e `registry_values_are_read_by_exact_name` girano solo su Linux/macOS (via `cfg(test)`).

**Correzione.** Nel job Windows aggiungere `cargo clippy -p shiftanything-tauri-proof --all-targets --features windows-lifecycle -- -D warnings` e `cargo test -p shiftanything-tauri-proof --features windows-lifecycle`.

### M4 — Il controllo chiave↔manifest resta opzionale, e la PR aggiunge un terzo asset firmato
`packaging/package-tauri-updater.sh:151` e `:172`, `packaging/RELEASE.md:7`

**Scenario.** `SHIFT_LAUNCHER_UPDATE_PUBKEY` è confrontato con la chiave delle firme solo "if set" (`:172`). La chiave che firma sul Mac può non essere quella compilata nel launcher (Windows la prende da `SHIFT_LAUNCHER_UPDATE_PUBKEY` in `release-nsis.ps1:22-24`). Se l'operatore non la imposta sul Mac, `latest.json` esce firmato con una chiave diversa e **tutti** gli utenti (tre piattaforme) ricevono «Download interrupted or invalid signature» al primo aggiornamento: il fallimento emerge sul campo, dopo la pubblicazione.

**Correzione.** Rendere `SHIFT_LAUNCHER_UPDATE_PUBKEY` obbligatorio in `package-tauri-updater.sh` (fail closed) e far scrivere il key-id atteso nel `windows-receipt.json` per confrontarlo lato Mac.

### M5 — Il nome prodotto su Windows diverge dal brand e resta hardcodato in due posti
`apps/shiftanything-tauri-proof/tauri.windows.json:2`, `packaging/windows/check-setup.py:35`

**Scenario.** `tauri.windows.json` forza `productName: "ShiftAnything"` mentre `tauri.conf.json` dice "Sh\*ftAnything": su Windows l'eseguibile, la voce in Start Menu, la voce in "App e funzioni" e la chiave di disinstallazione sono "ShiftAnything", su macOS/Linux "Sh\*ftAnything". Nessun documento della PR lo dice. `check-setup.py:35` pretende esattamente `{"ShiftAnything"}`: un cambio di nome (o l'allineamento al brand con l'asterisco) blocca la release con un errore fuorviante "setup declares [...]".

**Correzione.** Derivare il nome atteso da un'unica fonte (config Cargo/tauri) invece di hardcodarlo in `check-setup.py` e in `release-nsis.ps1:63`, e documentare la scelta in RELEASE.md.

---

## LOW

**L1 — `on_before_exit(|| {})` non fa ciò che dice il commento e disattiva la pulizia di uscita di Tauri** (`apps/shiftanything-tauri-proof/src/launcher_update/signed.rs:71-73`). L'hook è eseguito **prima** di `ShellExecuteW` (`tauri-plugin-updater-2.12.0/src/updater.rs:849-852`), non prima dell'uscita; l'uscita (`std::process::exit(0)`, `updater.rs:882`) avviene comunque dopo un avvio riuscito e non è impedibile. Se `ShellExecuteW` fallisce, `install_inner` restituisce Err senza uscire: il launcher resta vivo senza bisogno dell'hook. L'unico effetto reale è丢掉 l'hook predefinito `app_handle.cleanup_before_exit()` (su Windows nasconde le finestre). *Correzione: rimuovere l'override e il commento.*

**L2 — La cartella di output è creata prima della validazione Authenticode** (`packaging/windows/release-nsis.ps1:33` prima delle righe 37 e 41). `-Authenticode off` con variabili di firma impostate → lo script solleva l'errore ma ha già creato la cartella; l'operatore sistema l'ambiente e rilancia → «Choose a new output directory» → deve cancellare a mano. *Correzione: spostare `New-Item` dopo il blocco di validazione (righe 36-48).*

**L3 — La versione è letta dal Cargo.toml radice con una regex su tutto il file** (`packaging/windows/release-nsis.ps1:16`). Oggi la prima occorrenza di `^version = "X.Y.Z"$` è `[workspace.package]` (riga 6), ma un `[workspace.dependencies.x]` con `version = "1.2.3"` spostato più in alto, o un semplice riordino del file, fa prendere la versione di una dipendenza: il confronto con `tauri.conf.json` (riga 18) fallisce con un messaggio fuorviante e la ricevota registra la versione sbagliata. *Correzione: `cargo metadata --no-deps --format-version 1` o un parser TOML.*

**L4 — `Set-Content -Encoding utf8` con Windows PowerShell 5.1 scrive il BOM** (`packaging/windows/release-nsis.ps1:46` e `:87`). `RELEASE.md:6` prescribe `powershell -File ...` (5.1), mentre la CI del repo usa `pwsh` (7, senza BOM). Il config Tauri non ne risente (il parser json5 considera U+FEFF spazio bianco), ma `windows-receipt.json` risulta illeggibile per `json.load()` e `jq` ("Expecting value: line 1 column 1"). *Correzione: `[IO.File]::WriteAllText($path, $text, [Text.UTF8Encoding]::new($false))` o eseguire lo script con `pwsh`.*

**L5 — `CARGO_TARGET_DIR` relativo non gestito** (`packaging/windows/release-nsis.ps1:57-59`). Se è relativo, `$release` viene risolto dopo `Pop-Location` (radice del repo, non la cartella dell'app) e setup/eseguibile non vengono trovati. *Correzione: `Resolve-Path` il target dir o vietare i valori relativi.*

**L6 — `check-setup.py:16` allinea il valore VS_VERSIONINFO sull'offset assoluto del file**, mentre il formato PE richiede l'allineamento rispetto all'inizio della struttura String: se i dati della risorsa non sono allineati a 4 byte, il parser legge a un offset sbagliato e riporta ProductName/ProductVersion errati (o assenti) bloccando la release con un messaggio falso. Fall closed, quindi basso impatto. *Correzione: leggere la directory delle risorse PE e l'allineamento relativo.*

**L7 — `Write-Host` al posto di `Write-Output`** (`packaging/windows/release-nsis.ps1:73,76,88`): gli altri script di `packaging/` usano `Write-Output` per le righe PASS; `Write-Host` non è output di pipeline (righe non redirizzabili/parsabili) e `PSAvoidUsingWriteHost` è segnalato da PSScriptAnalyzer. *Correzione: `Write-Output`.*

**L8 — L'uninstaller viene firmato via `!uninstfinalize`, di cui NSIS ignora il codice di uscita** (`RELEASE.md:22` afferma che l'uninstaller viene firmato; `verify-release-signature.ps1` controlla solo setup ed eseguibile). In modalità `required` l'uninstaller può restare non firmato senza che la build fallisca. *Correzione: firmare e verificare l'uninstaller per conto proprio, o correggere la frase.*

**L9 — Manca `--keypass`** (`packaging/windows/jsign-sign.ps1:12-14`): con un keystore PKCS#12 in cui la password della chiave ≠ password dello store, jsign fallisce e la build si ferma con un errore jsign. *Correzione: accettare `SHIFT_WINDOWS_SIGN_KEYPASS` opzionale.*

---

## Verificato e senza difetti (risposte ai punti chiesti)

- **Firma dell'aggiornamento non aggirabile.** Il plugin verifica la minisign sui byte scaricati (`update.download()` → `verify_signature`, `tauri-plugin-updater-2.12.0/src/updater.rs:740-746`), con `pubkey` e `requireSignedVersion` iniettati in `signed::configure()`; `verify_signature` fallisce chiuso con chiave vuota o firma non valida, e `require_signed_version` impedisce di annunciare una versione nuova con la firma di una release vecchia. Il downgrade è bloccato anche da `allow_downgrades = false`.
- **Nessun download da host diverso dal manifest.** `trusted_download` (`signed.rs:43-45`) confronta host e porta del manifest: pinning del primo hop. Il redirect di Gitrix (`objects.githubusercontent.com`) non è coperto dal confronto, ma è irrelevante perché la minisign è il vero cancello.
- **Registro Windows corretto.** Confrontato con il template NSIS di Tauri v2.12.5: `UNINSTKEY = Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCTNAME}`; `currentUser` ⇒ `RequestExecutionLevel user` + `SetShellVarContext current` ⇒ HKCU; `MainBinaryName` = "ShiftAnything.exe" (solo nome, `folder.join` corretto); `InstallLocation` scritto **con virgolette letterali** (gestite da `trim_matches('"')`); `DisplayVersion` = versione senza virgolette. `canonicalize` da entrambi i lati ⇒ confronto coerente.
- **Il flusso NSIS è compatibile con i flag del plugin** (`/P /UPDATE /R /ARGS`): `/P` ⇒ `PassiveMode`, `.onInstSuccess` rilancia l'app perché `PassiveMode = 1`; in modalità passiva la macro `CheckIfAppIsRunning` salta il MessageBox e chiude il vecchio processo via Restart Manager.
- **`latest.json` corretto.** Le tre chiavi (`darwin-aarch64`, `linux-x86_64-deb`, `windows-x86_64`) corrispondono alle costanti `TARGET` dell'app; la forma `{"version", "platforms":{...}}` è quella attesa dal plugin; l'URL del setup usa lo stesso tag `v{version}` degli altri asset; la firma contiene `version:` nel trusted comment e tutte e tre le chiavi devono coincidere.
- **`-- --locked` raggiunge cargo** (la CLI appende gli argomenti dopo `--` al comando cargo, `tauri-cli/src/interface/rust/desktop.rs:261`), quindi il lockfile è bloccato come inteso.
- **`%1` usato correttamente**: il bundler sostituisce l'argomento *esattamente* uguale a `%1` con il percorso (`sign.rs`, `sign_command_custom`), quindi i percorsi con spazi reggono; `jsign-sign.ps1` riceve `-File` come argomento separato.
- **Nessun segreto in argv né nei log**: gli script nominano solo variabili (`SHIFT_WINDOWS_SIGN_STOREPASS` non compare mai come valore), il ricevuto contiene solo versione/commit/size/sha256, e la chiave di firma degli aggiornamenti non viene mai compilata nel binario Windows (il bundle la firma sul Mac).
===ERR
ndows/jsign-sign.ps1` e controlla setup ed eseguibile con `verify-release-signature.ps1`. Con `-Authenticode off` lo script si ferma se trova una di quelle variabili, così una configurazione a metà non produce in silenzio un setup non firmato.
    23	5. La firma dell'aggiornamento si fa dopo, sul Mac, sul setup già firmato Authenticode: prima Authenticode, poi SHA-256, poi la firma Tauri.
[0m
