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
3. Il DMG passa la verifica di `release-dmg.sh` sul Pro. La prova di Gatekeeper si rifà sull'**Air**, che non ha firmato nulla: il DMG si scarica con l'attributo di quarantena (`xattr -p com.apple.quarantine` presente), poi `spctl -a -t open --context context:primary-signature`, `xcrun stapler validate`, copia in Applicazioni e apertura con l'app senza avvisi (screenshot letto).
4. Il DEB si installa su Ubuntu 24.04 x64 e l'archivio `.tar.gz` si avvia su una distribuzione diversa da Ubuntu (Fedora o Debian), con screenshot letti.
5. `latest.json` ha solo `darwin-aarch64` e `linux-x86_64-deb`, con firme che `package-tauri-updater.sh` verifica; nessuna voce Windows.
6. Aggiornamento dalla 0.20.9 alla 0.21.0 su Mac e su Linux, in due tempi, con screenshot prima e dopo letti:
   - **Prima della pubblicazione.** L'app legge `latest.json` da un indirizzo fissato in compilazione (`SHIFT_LAUNCHER_UPDATE_ENDPOINT` in `launcher_update/signed.rs`); nella 0.20.9 pubblicata è `https://github.com/pampa-stack/shiftanything/releases/latest/download/latest.json`, che non vede le bozze. L'indirizzo deve essere HTTPS e il certificato si controlla con il portachiavi del sistema (`rustls-platform-verifier`). Quindi: una build di prova della 0.20.9 da `db42c75`, uguale tranne `SHIFT_LAUNCHER_UPDATE_ENDPOINT=https://ax42.tail680f56.ts.net:8443/latest.json`, installata sull'Air e nel guest Linux. Sull'AX42 `tailscale serve` (solo dentro la rete Tailscale, mai `funnel`) serve una cartella con una copia di `latest.json` i cui indirizzi puntano agli stessi file firmati della 0.21.0, copiati byte per byte (hash confrontati con `SHA256SUMS.txt`). La build di prova non esce mai dalle macchine di prova; la build pubblicata usa l'indirizzo di GitHub, controllato con `shiftanything --check-update-channel` prima del caricamento. Finita la prova, `tailscale serve reset` sulla porta 8443.
   - **Dopo la pubblicazione.** Dalla 0.20.9 pubblicata, installata da zero sull'Air e nel guest Linux, aggiornamento dal vivo alla 0.21.0, con il tag di rollback già pronto.
7. Catture reali IT/EN dalla build 0.21 (Catalogo, SH*FTER, Trova app, Kit di lavoro, Le mie app) in WebP, con gli stessi byte nei README IT/EN e nella landing promptica.it/shiftanything (vedi «Landing»).
8. Bozza GitHub con `--target`, file scaricati di nuovo e confrontati con `SHA256SUMS.txt` prima di pubblicare.
9. Le 5 prove prima della pubblicazione: pipeline a 9 fasi; revisione di un modello di un'altra famiglia con link al commento e 0 problemi gravi aperti; `cargo fmt --all --check`, clippy `-D warnings` e test di core, native e tauri-proof verdi, con i fallimenti preesistenti elencati a parte; prova dal vivo (download anonimo dalla landing su desktop e telefono, aggiornamento su Mac e Linux); tag di rollback prima del merge e `--match-head-commit`.
10. Riepilogo a SHIFTANYTHING - MAIN con link a release, landing e screenshot.

## Landing

La landing sta nel repo `pampa-stack/promptica-it` (ramo predefinito `main`, Next.js su Vercel), non in questo. Per la 0.20.9 sono servite due PR: [#591](https://github.com/pampa-stack/promptica-it/pull/591) (quattro schermate in `public/shiftanything/0.20.9/` e `app/shiftanything/page.tsx`) e [#592](https://github.com/pampa-stack/promptica-it/pull/592) (Le mie app, con `shiftanything.module.css`). Per la 0.21 si fa lo stesso:

1. Ramo `f/shiftanything-021-screenshots` in un worktree di quel repo; WebP in `public/shiftanything/0.21.0/`, con la versione nell'indirizzo per il rinnovo della cache, e gli stessi byte dei README di questo repo.
2. `app/shiftanything/page.tsx` punta alle nuove immagini; versione, peso e download continuano a venire da GitHub.
3. PR verso `main` con il commit e il deployment Vercel di ritorno indietro scritti nella descrizione, come nella #591; anteprima Vercel controllata su desktop e telefono con screenshot letti.
4. Revisione di un modello di un'altra famiglia sulla PR della landing, con link al commento e 0 problemi gravi.
5. La PR della landing si unisce solo dopo la pubblicazione della release 0.21.0 e solo con le sue 5 prove; il download anonimo dalla landing in produzione chiude il criterio 9.

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
