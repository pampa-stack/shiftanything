# KopiaUI su macOS Apple Silicon

Il 27 novembre 2026 da Sh*ftAnything in italiano sono stati approvati il download ufficiale di KopiaUI 0.23.1, il primo avvio e la rimozione. Il bundle è finito nel cestino del profilo di prova con i dati sul posto. [Conferma installazione](install-confirmation.png) · [Conferma rimozione](remove-confirmation.png)

L'opzione di primo avvio usa Launch Services. AppKit ha visto il processo e gli elementi del Dock includevano KopiaUI in esecuzione dal bundle temporaneo. La finestra di KopiaUI è rimasta in caricamento. Il processo Go rispondeva su loopback e gli errori del renderer non riportavano la causa. Il timeout degli aggiornamenti era un evento distinto. [Finestra KopiaUI](kopia-running.png) · [Prova Dock](dock.log)

L'avvio ha prodotto una preferenza fuori dalla HOME temporanea. Abbiamo chiuso il processo di prova e spostato nella quarantena del profilo soltanto il file creato durante il test. Il successivo controllo ha trovato assenti gli stessi 16 percorsi inventariati prima del test.

L'aggiornamento 0.23.0 → 0.23.1 è passato in una prova separata con backend Rust, download da GitHub, SHA-256, firma Kopia LLC `7299Z9ZVTK`, bundle `io.kopia.ui` e dati preservati. Il CLI incluso ha eseguito un backup cifrato e un ripristino. [Log del ciclo nativo](native-cycle.log) · [Release 0.23.0](https://github.com/kopia/kopia/releases/tag/v0.23.0) · [Release 0.23.1](https://github.com/kopia/kopia/releases/tag/v0.23.1)
