# Specifica: beta 0.21 di Sh*ftAnything

Fase 2/9 della pipeline. Mandato: `.git/handoffs/2026-11-28-mandato-beta-021.md` della cartella principale. Procedura già scritta da seguire, non da riscrivere: [packaging/RELEASE.md](../../packaging/RELEASE.md), [packaging/macos/README.md](../../packaging/macos/README.md), `packaging/package-tauri-updater.sh`, `packaging/linux/portable-tar.sh`.

## Cosa esce

Sh*ftAnything 0.21.0 beta per **Mac Apple Silicon** e **Linux x64**, come la 0.20.9. Windows resta fuori (decisione di Pampa del 28 novembre): niente setup Windows e nessuna voce `windows-x86_64` nel `latest.json` pubblicato. Windows entrerà con lo stesso numero di versione degli altri alla prima uscita dopo 50 app Windows verificate.

File della release, uguali per nome e ruolo a quelli della 0.20.9:

| File | Da dove |
|---|---|
| `ShiftAnything-0.21.0-macos-arm64.dmg` | `release-dmg.sh` sul Pro: firma Developer ID, notarizzazione, Gatekeeper |
| `ShiftAnything-0.21.0-macos-arm64.app.tar.gz` e `.sig` | `package-tauri-updater.sh` |
| `ShiftAnything-0.21.0-linux-amd64.deb` e `.sig` | build Tauri sull'AX42 |
| `ShiftAnything-0.21.0-linux-amd64.tar.gz` | `portable-tar.sh` dallo stesso DEB |
| `latest.json` | `package-tauri-updater.sh`: solo `darwin-aarch64` e `linux-x86_64-deb` |
| `SHA256SUMS.txt` | hash di tutti i file sopra |

## Da quale commit

Decisione di SHIFTANYTHING - MAIN del 28 novembre (opzione A corretta):

1. Il ramo predefinito del repo, `f/pro-371-next-verification`, è stato unito nel ramo della PR #7 con un merge normale (`71fc43f`, stesso albero di `8a53891`). La base della PR #7 ora è `f/pro-371-next-verification`.
2. Il commit di versione (0.21.0 in `Cargo.toml` e `apps/shiftanything-tauri-proof/tauri.conf.json`, stesso commit) va sul ramo della PR #7. Build, prove e revisione si fanno su quello SHA.
3. Con le 5 prove complete: tag di rollback sulla punta del ramo predefinito, merge della PR #7 con `--match-head-commit`, poi tag `v0.21.0` sul commit di merge con `--target <sha del merge>`. I pacchetti costruiti dallo SHA della PR valgono per il merge solo se l'albero del merge è identico a quello della punta della PR (controllo con `git rev-parse <sha>^{tree}`); se l'albero cambia, si ricostruisce dal merge.
4. `main` e `f/shiftanything-next-beta` non si toccano; le PR #3 e #4 restano come sono.

## Note di rilascio

`python3 packaging/release-notes.py "$commit" --previous db42c75 --changes novita.md`. Oggi dà 21 verifiche nuove (Mac +6, Linux x64 +15). `novita.md` elenca, in parole semplici, le 7 app nuove su Linux del 28 novembre (Bruno, Cutter, Trayscale, OpenTabletDriver, Angry IP Scanner, Termix, OpenWhispr) e le altre funzioni completate dopo la 0.20.9 sul ramo; si ricava da `git log db42c75..$commit`, non da memoria.

## Dove si costruisce

- Mac: il certificato Developer ID Application è solo nel portachiavi del Pro, che ha circa 12 GB liberi. La build `release` si fa sull'Air (23 GB liberi); sul Pro arriva solo il binario, con `CARGO_TARGET_DIR` che punta alla sua cartella, per `release-dmg.sh --check` e `release-dmg.sh`. Se lo script vuole compilare sul Pro, prima `df -h /` deve lasciare almeno 8 GB liberi a fine build, e la cartella `target` si cancella dopo.
- Linux: DEB e `.tar.gz` dallo stesso commit sull'AX42, non sul Mac.
- Firma aggiornamenti: `TAURI_SIGNING_PRIVATE_KEY_PATH` punta alla chiave Tauri; la chiave non si legge né si stampa, si prende da 1Password con `op-regia`.

## Criteri di finito

Ogni criterio ha la sua prova scritta nel rapporto finale.

1. `release-notes.py` sul commit di release esce 0 con almeno 20 verifiche nuove rispetto a `db42c75`.
2. `Cargo.toml` e `tauri.conf.json` dicono 0.21.0 nello stesso commit; `--version` del binario Mac e di quello Linux dice 0.21.0.
3. Il DMG passa `spctl -a -t open --context context:primary-signature`, `stapler validate` e la verifica di `release-dmg.sh`; l'app copiata dal DMG si apre su un Mac Apple Silicon (screenshot letto).
4. Il DEB si installa su Ubuntu 24.04 x64 e l'archivio `.tar.gz` si avvia su una distribuzione diversa da Ubuntu (Fedora o Debian), con screenshot letti.
5. `latest.json` ha solo `darwin-aarch64` e `linux-x86_64-deb`, con firme che `package-tauri-updater.sh` verifica; nessuna voce Windows.
6. Aggiornamento dal vivo dalla 0.20.9 alla 0.21.0 su Mac e su Linux, dalla bozza o dalla release, con screenshot prima e dopo letti.
7. Catture reali IT/EN dalla build 0.21 (Catalogo, SH*FTER, Trova app, Kit di lavoro, Le mie app) in WebP, con gli stessi byte nei README IT/EN e nella landing promptica.it/shiftanything.
8. Bozza GitHub con `--target`, file scaricati di nuovo e confrontati con `SHA256SUMS.txt` prima di pubblicare.
9. Le 5 prove prima della pubblicazione: pipeline a 9 fasi; revisione di un modello di un'altra famiglia con link al commento e 0 problemi gravi aperti; `cargo fmt --all --check`, clippy `-D warnings` e test di core, native e tauri-proof verdi, con i fallimenti preesistenti elencati a parte; prova dal vivo (download anonimo dalla landing su desktop e telefono, aggiornamento su Mac e Linux); tag di rollback prima del merge e `--match-head-commit`.
10. Riepilogo a SHIFTANYTHING - MAIN con link a release, landing e screenshot.

## Fuori da questo lavoro

- Windows (decisione di Pampa del 28 novembre).
- Umi-OCR su Linux: ticket [#11](https://github.com/pampa-stack/shiftanything/issues/11).
- Unificare i rami in un solo ramo principale: ticket #12.
- Catalogo firmato remoto: ticket #10.

## Rischi noti

- Notarizzazione Apple: lo script aspetta fino a 45 minuti e salva gli ID per riprendere con `notarytool info`.
- Spazio sul Pro: 12 GB liberi; una build `release` completa sul Pro non ci sta con margine.
- Il ramo predefinito può ricevere altre PR prima del merge: in quel caso si riunisce, si rifanno controlli e build, e si rifà il controllo dell'albero del punto 3 di «Da quale commit».
- Gate: la pubblicazione della release e la landing sono azioni verso l'esterno; partono solo con le 5 prove.
