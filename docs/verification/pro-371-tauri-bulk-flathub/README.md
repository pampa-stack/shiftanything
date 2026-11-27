# Aggiorna tutte con due app Flathub reali

Prova del 27 novembre 2026 su Ubuntu 26.04 x86-64, VPS Strumenti. Finestra WebKitGTK del DEB locale del checkpoint `f569e40` in Xvfb 1440 × 900. Profilo launcher e Flatpak temporanei, host in sola lettura ed AppData sintetici. Il [digest](launcher.sha256) riguarda l'eseguibile estratto dal DEB.

1. Installate GNOME Calculator e GNOME Text Editor dalla scheda e approvate le azioni nella finestra. [Log d'installazione](initial-install.log)
2. Preparato lo stato iniziale retrocedendo, solo nel profilo di prova, alle revisioni firmate e aggiornando il relativo inventario di proprietà. Entrambe le righe comparivano in Aggiornamenti. [Preparazione](prepare.log) · [Screenshot prima](bulk-before.png)
3. Rifiutato il primo Aggiorna tutte. Entrambi i digest ufficiali precedenti, firma e dati sono rimasti invariati. [Verifica del rifiuto](rejected.log) · [Conferma con due nomi](bulk-consent.png)
4. Approvato il secondo Aggiorna tutte una volta. NativeBackend ha concluso entrambi gli aggiornamenti. Confrontati digest Flatpak reali, inventario, firme GPG e dati sintetici. Text Editor conserva la versione pubblica 51.0 e cambia revisione. [Log nativo](tauri-run.log) · [Verifica dopo](update-verify.log) · [Screenshot dopo](bulk-after.png)
5. Rimosse entrambe le app dalla stessa finestra con approvazioni distinte. Entrambe rimosse, entrambi i marcatori del profilo conservati. [Verifica rimozione](removed.log) · [Screenshot rimozione](bulk-removed.png)

La prova usa installer e repository Flathub reali. La retrocessione prepara le condizioni iniziali. La prova non attribuisce copertura a Windows, Open o Cancel. Le revisioni sono quelle individuate al momento della prova.
