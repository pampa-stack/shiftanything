# Annullamento di un’installazione Flathub dal launcher Linux

Prova del 27 novembre 2026, 05:21 UTC, VPS Strumenti, Ubuntu 26.04 x86_64. Build Tauri `1701d47`, SHA-256 binario `2040a350a52dcceaaa5dc6db81549071f8a0fe17a46b6d71a0f637fe3fd1abcf` verificato dopo il trasferimento. 491 schede. Stessi file di annullamento del commit `a3306ad`, verificati con `git diff --exit-code`.

Ref `app/org.gimp.GIMP/x86_64/stable` pubblicato dal [team GIMP](https://www.gimp.org/downloads/) su [Flathub](https://flathub.org/apps/org.gimp.GIMP). Il motore ha creato il remote utente nel nuovo profilo. Il suo [output](evidence/flatpak-result.txt) mostra `https://dl.flathub.org/repo/`, `gpg-verify=true`, `gpg-verify-summary=true` e SHA-256 del keyring `c504fa5dc891df6cfcc10021dd9addf08459007f1787f40b9c6fd3c7e58416ea`.

| Azione | Riscontro |
| --- | --- |
| Click Installa e approvazione `ui-1` | PID 2276916, processo discendente del launcher, con argomenti `flatpak install --user --noninteractive --assumeyes flathub app/org.gimp.GIMP/x86_64/stable`. [Installazione in corso](evidence/03-actual-installation.png) |
| Primo click Annulla operazione, quindi Rifiuta `ui-2` | Esito `rejected`. Lo stesso PID 2276916 è ancora attivo, l’installazione resta in corso e `ui-1:gimp` non ha esito finale. [Consenso](evidence/04-reject-cancellation.png) · [Installazione ancora in corso](evidence/05-still-installing-after-refusal.png) |
| Secondo click Annulla operazione, quindi Approva `ui-3` | Esiti distinti `ui-1:gimp: cancelled` e `ui-3:gimp: completed: operation cancelled`. Il PID di Flatpak è terminato, il launcher non ha più job attivi e il ref utente è assente. [Inventario](evidence/07-no-installation-running.json) · [Esito visibile nella scheda](evidence/08-cancelled-in-gimp-details.png) |
| Controllo del profilo | SHA-256 del file sintetico di GIMP invariato, `528b6410dec54c8d6528609ffdad0d6d6a98aebff7e8f9de5829fbf3b6de3ae7`. [Log Flatpak](evidence/flatpak-result.txt) |

Gli otto PNG provengono dalla finestra Tauri 1440×900, avviata su Xvfb, e sono stati letti singolarmente. L’annullamento ha interessato una singola installazione. Le approvazioni e il rifiuto arrivano da click nella finestra. La preservazione dati riguarda il file sintetico del profilo. La prova lascia 93 MiB di dati di test nel proprio profilo, inclusi cache e log.

Sul server la prova è in `/var/tmp/shift-cancel-tauri.ZtjoIuXG`. Il profilo, WebKit e D-Bus girano in bwrap con filesystem host in sola lettura, HOME dedicata, X11 privato, rete attiva per Flathub. Nessun accesso a profili reali. Con il binario verificato e `env.sh`, `tauri-gui.sh`, `verify_cancel.py` in una cartella nuova `/var/tmp/shift-cancel-tauri.*`, eseguire `python3 -u verify_cancel.py SHA256`. L’harness esige profilo assente, verifica SHA e controlla il comando esatto via `/proc`, limitandosi ai discendenti del processo creato. Evidenze in `evidence`, [log dell’harness](verify-cancel.log) e [log nativo](evidence/tauri-run.log).
