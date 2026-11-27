# Sh*ftAnything su Ubuntu 26.04 x86_64

VPS Strumenti, 27 novembre 2026. Binario Tauri da commit `468602c`, SHA-256 `3c4b811cbbd5725fa583b34dfdb3b7d6b6f160aa9a9d5b33d808ed0820db317c`. Profilo privato `/var/tmp/shift-tauri-flathub.ysriyaXg/launcher-profile`. Xvfb 1440×900 con Xauthority. Runtime GTK e Mesa estratto da pacchetti Ubuntu ufficiali, nessuna installazione globale. Montaggio Bubblewrap privato, home dell'account mascherata e sandbox WebKit attiva.

| Prova | Risultato | Evidenza |
| --- | --- | --- |
| Apertura launcher | Catalogo IT con 462 app e Linux x64 | [PNG](evidence/flathub-tauri-02-mesa.png), [EGL software Mesa](evidence/tauri-eglinfo.log) |
| Cerca e Installa → Rifiuta | Ref Flatpak, chiave e remote assenti | [Attesa consenso](evidence/flathub-tauri-04-install-confirm.png), [dopo rifiuto](evidence/flathub-tauri-05-install-rejected.png), [stato](evidence/tauri-rejected.state) |
| Installa → Approva | GNOME Calculator 51.1 da Flathub firmato per l'utente | [Durante installazione](evidence/flathub-tauri-06-install-running.png), [ref con SHA-256](evidence/tauri-installed.state), [log launcher](evidence/tauri-run.log) |
| Apri → Approva | Processo di lancio accettato. App senza finestra nel bwrap usato per la prova, kernel rifiuta secondo user namespace di Flatpak | [Consenso Apri](evidence/flathub-tauri-07-open.png), [diagnosi](evidence/tauri-flatpak-nested.log) |
| Rimuovi da MCP → Approva | Stdio reale: in coda prima del click, outcome completato dopo click. Ref rimosso, marcatore dati app preservato | [Consenso Rimuovi](evidence/flathub-tauri-09-remove-mcp.png), [log SDK](evidence/tauri-mcp-remove.log), [stato](evidence/tauri-removed.state), [catalogo finale](evidence/flathub-tauri-11-finished.png) |

Ref esatto `app/org.gnome.Calculator/x86_64/stable`, remote `https://dl.flathub.org/repo/`, revisione `00516a24d410ad62ed3e955fa5deb5af221ab6b181ed5549103f3670d10d2fca`. Apertura reale delle quattro app e aggiornamento a revisione successiva verificati separatamente con il motore nativo. [Referto nativo](README.md)

Il client riproducibile `tools/verify_tauri_mcp.py` della build puntava alla cartella del bridge macOS. Il client aggiornato sceglie la cartella dal sistema rilevato nel profilo isolato. Le interrogazioni stdio dello stesso client hanno passato i casi negativi su Linux e macOS. Il server MCP è lo stesso della prova. La risposta MCP contiene `outcomes["<request-id>:gnome-calculator"] = "completed: Flathub app removed"` anche quando `state` conserva la stringa di approvazione iniziale. La prova Tauri copre rifiuto, installazione e rimozione con pacchetto e repository reali.

## Aggiornamento di Calcolatrice e test MCP Linux

Nello stesso profilo temporaneo, dopo una nuova installazione gestita, Flatpak ha selezionato la revisione ufficiale storica 51.0 `0032886e3396c2e0cb107d453d600301c8090241d3fef31ca3f688510dba0e43` con GPG verificato. Il banco ha aggiornato il solo manifest scratch per rappresentare una vecchia installazione gestita. Il digest è rimasto 51.0 con Aggiorna in attesa di approvazione. Dopo il click in Tauri, Flatpak ha distribuito 51.1 `00516a24d410ad62ed3e955fa5deb5af221ab6b181ed5549103f3670d10d2fca`, anch'essa verificata GPG. L'interfaccia riportava 51.1.0, zero aggiornamenti e operazione completata. Il marker dati è sopravvissuto anche al successivo Rimuovi. La build 468602c riporta ancora la stringa motore “up to date” poi corretta in a3f9b02. [Prima e dopo](evidence/tauri-update-before.state) · [In attesa](evidence/tauri-update-pending.state) · [Nuovo digest](evidence/tauri-update-after.state) · [Vecchio GPG](evidence/tauri-update-old-ostree-gpg.log) · [Nuovo GPG](evidence/tauri-update-new-ostree-gpg.log) · [Log update](evidence/tauri-run-update-cycle.log) · [Rimozione](evidence/tauri-update-removed.state)

Tre screenshot letti: [Calcolatrice 51.0 e nuova revisione](evidence/flathub-tauri-update-03-manage.png) · [Consenso Aggiorna](evidence/flathub-tauri-update-04-consent.png) · [Calcolatrice aggiornata a 51.1](evidence/flathub-tauri-update-05-installed.png) · [Checksum screenshot](evidence/tauri-update-png.sha256)

Lo script di verifica MCP aggiornato è passato identico al file del worktree, senza `--operation`. Esegue lo stdio reale Linux, rileva la scheda disinstallata e rifiuta le operazioni incompatibili e gli ID sconosciuti. [Log MCP Linux](evidence/tauri-mcp-inventory.log)
