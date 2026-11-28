# Piano: beta 0.21 di Sh*ftAnything

Fase 3/9. Segue [spec.md](spec.md). Nessuna migrazione di dati; il ritorno indietro è il tag di rollback sul ramo predefinito e la release 0.20.9 che resta pubblicata.

## Pezzo → chi lo fa già

| Pezzo | Chi lo fa già (in questo repo) |
|---|---|
| Note e conteggio | `packaging/release-notes.py` |
| DMG firmato, notarizzato, controllato | `packaging/macos/release-dmg.sh` (con `--check`) |
| Firme updater, `latest.json`, `.tar.gz` Linux | `packaging/package-tauri-updater.sh`, `packaging/linux/portable-tar.sh` |
| DEB Ubuntu | `packaging/linux/local-deb.sh` |
| Indirizzo di aggiornamento nel binario | `--check-update-channel` (`startup.rs`) |
| Servire file in HTTPS nella rete privata | `tailscale serve` sull'AX42 (certificato `ax42.tail680f56.ts.net`) |
| Guest Linux di prova | `/var/tmp/shift-beta021-guest.gvBiii0o` sull'AX42 (`start.sh`) |
| Revisione di un'altra famiglia | agent `codex-reviewer` |

Il nostro codice resta colla: nessuno script nuovo, salvo quello che i passi sotto dimostrano mancare.

## Macchine

| Macchina | Ruolo | Spazio |
|---|---|---|
| Air (`ssh air`) | build `release` Mac arm64; prova Gatekeeper con quarantena; prove di aggiornamento | 23 GB liberi |
| Pro | firma Developer ID, notarizzazione, `package-tauri-updater.sh` (vuole macOS arm64 e `APPLE_TEAM_ID`) | 12 GB liberi: nessuna build completa qui |
| AX42 | build Linux (nel guest Ubuntu 24.04), DEB, `.tar.gz`, `tailscale serve` per la prova di aggiornamento | 309 GB |

## Passi

1. **Versione.** Commit sul ramo della PR #7: 0.21.0 in `Cargo.toml` (`[workspace.package]`), `Cargo.lock` e `apps/shiftanything-tauri-proof/tauri.conf.json`; `novita.md` scritto da `git log db42c75..HEAD`. Controlli fmt, clippy, test di core, native, tauri-proof nel guest. Questo SHA si chiama `R`.
2. **Note.** `python3 packaging/release-notes.py R --previous db42c75 --changes novita.md > release-notes.md`; deve uscire 0.
3. **Linux.** Nel guest, da `R`: build `release` di `shiftanything-tauri-proof` con `SHIFT_LAUNCHER_UPDATE_ENDPOINT` di GitHub e `SHIFT_LAUNCHER_UPDATE_PUBKEY` della 0.20.9 (letta dal binario 0.20.9, è pubblica). `--check-update-channel` deve stampare `linux-x86_64-deb https://github.com/pampa-stack/shiftanything/releases/latest/download/latest.json`. DEB con `local-deb.sh`. Lo script legge sempre `${CARGO_TARGET_DIR}/debug/shiftanything-tauri-proof` (riga 9), mentre il binario della 0.20.9 (82 MB, non strippato) sembra `release`: prima si controlla il DEB della 0.20.9 (`dpkg-deb -I`, dimensione, simboli); poi un commit a parte, dichiarato nella PR, aggiunge allo script la scelta del profilo (`release` o `debug`, predefinito invariato) con `shellcheck` verde.
4. **Mac.** Sull'Air, da `R`, build `release` arm64 con le stesse due variabili; `--check-update-channel` stampa `darwin-aarch64` e l'indirizzo GitHub. Il binario va sul Pro in `CARGO_TARGET_DIR=$HOME/shift-021-target`, al percorso `release/shiftanything-tauri-proof` che `local-app.sh` (righe 19-25) legge senza compilare; SHA-256 del binario uguale sull'Air e sul Pro prima di andare avanti. Sul Pro `release-dmg.sh --check`, poi `release-dmg.sh` (firma, notarizzazione, Gatekeeper). Dopo, `shift-021-target` si cancella.
5. **Updater.** Sul Pro `package-tauri-updater.sh 0.21.0 DMG HASH DEB HASH pampa-stack/shiftanything OUT`, con `TAURI_SIGNING_PRIVATE_KEY_PATH` su un file in una cartella `mktemp -d` con `umask 077`, scritto da `op-regia` senza mostrarne il contenuto e cancellato da un `trap` anche in caso di errore; se la chiave ha una password, ci si ferma prima (lo script toglie `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` alla riga 6). `SHIFT_LAUNCHER_UPDATE_PUBKEY` impostata con la chiave pubblica della 0.20.9, così lo script (righe 140-162) rifiuta firme con un'altra chiave: senza quella variabile il controllo non avviene. Esce con `latest.json` (solo `darwin-aarch64` e `linux-x86_64-deb`), `.sig`, `.tar.gz` Linux, `SHA256SUMS.txt`.
6. **Prova di aggiornamento prima della pubblicazione.** Build di prova della 0.20.9 da `db42c75` con `SHIFT_LAUNCHER_UPDATE_ENDPOINT=https://ax42.tail680f56.ts.net:8443/latest.json` (stessa chiave pubblica), su Air e guest. Sull'AX42 una cartella con i file del passo 5 copiati e una copia di `latest.json` con gli indirizzi verso `ax42.tail680f56.ts.net:8443`; `tailscale serve --bg --https=8443 <cartella>`. La cartella servita contiene solo i file del passo 5 e la copia di `latest.json`, con hash controllati prima di avviare `tailscale serve` (mai `funnel`, mai `reset`; `tailscale serve status` prima e dopo). Installare la 0.20.9 di prova, aggiornare, schermate prima e dopo, `--version` 0.21.0. Su Linux l'aggiornamento passa da `pkexec apt-get` (`signed.rs` righe 206-216): nel guest usa e getta una regola polkit concede `pkexec` all'utente `qa` senza agente grafico; la regola si scrive nel rapporto. Poi `tailscale serve --https=8443 off` e cancellazione delle build di prova. Il guest raggiunge il nome ts.net attraverso l'AX42; se non lo risolve, una riga in `/etc/hosts` del guest. La prova dimostra il flusso di aggiornamento con una 0.20.9 ricompilata, non il binario pubblicato: quello si prova al passo 13.
7. **Gatekeeper sull'Air.** DMG del passo 4 copiato sull'Air e marcato come scaricato da Safari (`xattr -w com.apple.quarantine "0083;<ora esadecimale>;Safari;"`): `gh` e `curl` non mettono la quarantena. `xattr -p` deve mostrarla prima di `spctl`, `stapler validate`, apertura e screenshot; senza attributo la prova non vale.
8. **Linux su un'altra distribuzione.** `.tar.gz` avviato in un contenitore o guest Fedora pulito sull'AX42 con Xvfb, screenshot; i pacchetti Fedora che servono (webkit2gtk4.1, gtk3) si scrivono nel README portatile. Il `.tar.gz` non si aggiorna da solo: `installed_path` vuole il DEB installato in `/usr/bin`.
9. **Catture.** Dalla build 0.21 sull'Air: Catalogo, SH*FTER, Trova app, Kit di lavoro, Le mie app in IT ed EN, WebP. Commit nei README di questo repo; PR nella landing (spec, «Landing»).
10. **PR pronta (fase 6).** Punta della PR #7 = `R` più catture. Se le catture cambiano l'albero dopo le build, le build valgono solo se nessun file compilato è cambiato (`git diff --stat R HEAD` solo su `README*` e `docs/screenshots/`).
11. **Revisione (fase 7).** `codex-reviewer` sul diff `origin/f/pro-371-next-verification...HEAD`, commento sulla PR, 0 problemi gravi.
12. **Controllo prima del merge.** `requireSignedVersion` (`signed.rs` riga 54) deve esistere nel sorgente di `tauri-plugin-updater` 2.12.0 nel registro Cargo; se non esiste, ci si ferma e si apre un ticket.
13. **Online (fase 9), solo con le 5 prove.** Tag di rollback `rollback/pre-0.21.0` sulla punta del ramo predefinito; `gh pr merge 7 --merge --match-head-commit <punta>`; il commit di merge `M` deve avere lo stesso albero di `R` (`git rev-parse R^{tree} M^{tree}`), altrimenti si ricostruisce da `M`. Solo ora la bozza: `gh release create v0.21.0 --draft --target M` con i file del passo 5 e `release-notes.md` (che dice «Codice usato per la build: R», vero perché l'albero è uguale); download dei file dalla bozza e confronto con `SHA256SUMS.txt`; pubblicazione come latest. Poi aggiornamento dal vivo dalla 0.20.9 pubblicata su Air e guest; poi la PR della landing.

## Metriche e soglie per l'online

- `release-notes.py`: uscita 0, almeno 20 verifiche.
- Checksum: 8 file su 8 uguali tra cartella locale, bozza e `SHA256SUMS.txt`.
- Chiave dell'updater: stesso ID chiave della 0.20.9 (controllo dello script con `SHIFT_LAUNCHER_UPDATE_PUBKEY`).
- `latest.json`: esattamente 2 piattaforme, `version` 0.21.0, firme verificate dallo script.
- Aggiornamento: 2 macchine su 2 passano da 0.20.9 a 0.21.0, sia prima sia dopo la pubblicazione.
- Ritorno indietro se l'aggiornamento dal vivo fallisce dopo la pubblicazione: rimettere la 0.20.9 come latest (`gh release edit v0.20.9 --latest`) e la 0.21.0 in bozza; il ramo predefinito torna al tag di rollback solo con l'ok di MAIN (è un'operazione distruttiva).

## Spesa

Nessuna: macchine già pagate, notarizzazione inclusa nell'account Apple, nessun servizio nuovo.
