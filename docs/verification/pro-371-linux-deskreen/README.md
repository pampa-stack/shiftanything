# Deskreen Community Edition su Ubuntu x86-64

Il 27 novembre 2026 una VM Ubuntu 24.04 su AX42 ha installato il DEB ufficiale 3.2.15, aperto la finestra come utente standard, aggiornato alla 3.2.16 e aperto anche questa versione. Dopo la rimozione con apt, l'eseguibile e la voce di menu erano assenti. Il file sintetico nel profilo XDG ha conservato lo stesso SHA-256 attraverso il ciclo. Non è stata avviata la condivisione dello schermo.

| Release | Byte del DEB amd64 | SHA-256 |
| --- | ---: | --- |
| [3.2.15](https://github.com/pavlobu/deskreen/releases/tag/v3.2.15) | 115664700 | `d4945afac08a82e243bf0cb90c84784b0a927184251b00efa93d1e142ab3c5f0` |
| [3.2.16](https://github.com/pavlobu/deskreen/releases/tag/v3.2.16) | 115728016 | `f27f0c70740941c5faadf8cbcff0ab35ace8026f8471f8fa7bc67ab907c09787` |

La prova ha controllato nome, architettura, versione, hash, script del pacchetto e voce desktop. L'eseguibile dichiarato dalla voce desktop è stato lanciato con Xvfb e D-Bus separati, senza opzioni aggiunte. In entrambe le versioni il controllo dei processi discendenti ha trovato `--no-sandbox` nel renderer. Il [sorgente della 3.2.16](https://github.com/pavlobu/deskreen/blob/v3.2.16/src/main/index.ts#L213-L228) configura `sandbox: false` nella finestra principale. [Comportamento documentato da Electron](https://www.electronjs.org/docs/latest/tutorial/sandbox#disabling-the-sandbox-for-a-single-process)

L'AppImage 3.2.15 si era fermata nel guest con un errore di configurazione di `chrome-sandbox`. Il percorso DEB è una prova indipendente via apt. Deskreen rimane fuori dall'elenco Linux installabile nel launcher, che gestisce AppImage, archivi supportati e Flatpak. Il [ciclo macOS](../pro-371-macos-deskreen/README.md) misura distribuzione e gestione del bundle. La sandbox del renderer macOS richiede una prova distinta.
