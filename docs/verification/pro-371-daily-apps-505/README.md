# Cinque app installate, aggiornate e rimosse

Prove del 27 novembre 2026 su macOS Apple Silicon 26.4 e Ubuntu 26.04 x86-64. Il motore ha usato pacchetti ufficiali e profili temporanei. Tutti i cicli hanno preservato il file sintetico del profilo.

| App | Sistema | Versione precedente e corrente | Prova |
| --- | --- | --- | --- |
| draw.io Desktop | Mac arm64, macOS 13+ | 32.6.10 → 32.7.4 | [Log](drawio-native-cycle.log) |
| Zed | Mac arm64, macOS 12+ | 1.29.3 → 1.30.2 | [Log](zed-native-cycle.log) |
| Pinta | Linux x64, Flathub | 3.1.2, GNOME 49 → GNOME 50 | [Log](pinta.cycle.log) · [Editor](native-pinta-current.png) |
| KeePassXC | Linux x64, Flathub | 2.7.12 → 2.8.0 | [Log](keepassxc.cycle.log) · [Finestra](native-keepassxc-current.png) |
| Xournal++ | Linux x64, Flathub | 1.3.8, due revisioni | [Log](xournal.cycle.log) · [Editor](native-xournal-current.png) |

Su macOS sono stati verificati hash SHA-256, architettura, bundle, Team ID, firma Developer ID, notarizzazione e accettazione da Gatekeeper su entrambe le versioni per ciascuna app. Il motore ha installato la versione precedente, aggiornato alla corrente e spostato il bundle nel Cestino del profilo. L'apertura delle due app resta da provare in una sessione grafica separata dal profilo personale. Zed fornisce l'editor base gratuito anche senza account. I servizi ospitati sono facoltativi e la build ufficiale ha termini propri. [draw.io](https://github.com/jgraph/drawio-desktop/releases/tag/v32.7.4) · [Zed](https://github.com/zed-industries/zed/releases/tag/v1.30.2) · [Termini e piani Zed](https://zed.dev/terms/zed)

Su Ubuntu, OSTree ha verificato la firma Flathub di tutti e sei i pacchetti. Il banco ha installato la revisione corrente dal motore, predisposto la precedente nel profilo temporaneo e controllato che l’installazione esterna non fosse modificabile dal launcher. Dopo aver registrato quella revisione come gestita nel solo manifest di prova, il motore l’ha aperta, aggiornata, riaperta e rimossa. Prima e dopo l’aggiornamento sono stati controllati digest e stato effettivo dell’app. La [vecchia revisione di Xournal++](native-xournal-previous.png) mostrava un rettangolo nero su parte del foglio in Xvfb. La revisione attuale mostra la pagina intera. [Pinta](https://flathub.org/apps/com.github.PintaProject.Pinta) · [KeePassXC](https://flathub.org/apps/org.keepassxc.KeePassXC) · [Xournal++](https://flathub.org/apps/com.github.xournalpp.xournalpp)
