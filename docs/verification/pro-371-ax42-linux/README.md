# Sh*ftAnything 0.20.3 su Ubuntu

Prova del 27 novembre 2026 sul server AX42-1. La VM KVM Ubuntu 24.04.5 x64 è partita da un'immagine Ubuntu con firma GPG verificata. La VM ha usato un profilo temporaneo e un display Xvfb a 1440×900. [Immagini Ubuntu](https://cloud-images.ubuntu.com/noble/current/)

Il pacchetto `ShiftAnything-0.20.3-linux-amd64.deb` del commit `32fd189` pesa 37.544.028 byte. SHA-256 `679ed18bd7c4e4dfe5ef52e44ef590392548c48fe87d5b5266a9348ad947c039`. APT ha installato il pacchetto, `gio launch` ha avviato il launcher dalla voce del desktop e il catalogo ha mostrato 531 app. Un secondo avvio ha mantenuto una sola istanza. APT ha rimosso pacchetto e voce del desktop. [Catalogo del launcher](catalog-531.png)

## Lapce

Dal launcher, il primo rifiuto ha lasciato l'app assente. Il secondo clic su Installa, seguito dall'approvazione, ha installato Lapce 0.4.6 da Flathub. La firma del repository è valida per il commit OSTree `b3110ba6ba344d33005e045864ec1d8c17905a6949f1e23dc9eda117918ba03b`. Dopo approvazione, Apri ha avviato la finestra di Lapce. Rimuovi ha eliminato il pacchetto e preservato il file di prova nelle impostazioni utente. Il ciclo di aggiornamento fra due versioni di Lapce ha un riscontro separato nel NativeBackend. [Approvazione](lapce-approval.png) · [Finestra di Lapce](lapce-open.png) · [Dopo rimozione](lapce-removed.png) · [Lapce su Flathub](https://flathub.org/apps/dev.lapce.lapce)

## Barra di avanzamento

Il pacchetto provato mostrava la fase di installazione Flatpak senza barra. Il frontend del prossimo aggiornamento mostra una barra di attività con nome accessibile in italiano e in inglese. Per i download con totale in byte continua a mostrare la percentuale. I test del frontend hanno esercitato entrambe le modalità usando eventi in un profilo temporaneo. [Barra di attività](flatpak-progress-it.png) · [Output Flatpak 1.14.6](https://github.com/flatpak/flatpak/blob/1.14.6/app/flatpak-quiet-transaction.c)

## Nuove app provate sull'AX42-1

DOSBox Staging, ScummVM e Video Downloader hanno completato installazione, apertura, aggiornamento Flatpak e rimozione con NativeBackend e directory utente isolate. Le coppie di screenshot mostrano la finestra prima e dopo. Per DOSBox Staging e ScummVM cambia la revisione firmata Flatpak. Il numero di versione dell'app resta 0.83.1 e 2026.4.0 rispettivamente. Le prove riguardano l'apertura delle finestre. [DOSBox Staging](dosbox-staging.png) · [ScummVM](scummvm.png) · [Video Downloader](video-downloader.png)

Pixelorama 1.2.3 → 1.2.4 ha completato il ciclo aprendo il menu File in entrambe le versioni. Zotero 10.0.8 ha completato il ciclo su due revisioni Flatpak e ha aperto una libreria vuota. Il pacchetto Zotero su Flathub è distribuito dalla community ed è citato dagli autori. Plugin e sincronizzazione non sono stati provati. [Pixelorama](pixelorama.png) · [Zotero](zotero.png) · [Documentazione Zotero](https://www.zotero.org/support/kb/linux_packaging)

Color Palette ha completato il ciclo su due revisioni firmate, con la palette GNOME visibile su entrambe. [Prima e dopo l'aggiornamento](color-palette.png)

Video Trimmer, GNOME Connections, Tokodon, NeoChat, Workbench, Jami e Furnace hanno completato il ciclo su due revisioni Flatpak firmate, con finestra aperta in un ambiente isolato e pacchetti rimossi al termine. Prove di apertura senza account, server o dati utente. Per Workbench il pannello di anteprima è visibile, per Furnace l'interfaccia del tracker. [Video Trimmer](video-trimmer.png) · [Connections](gnome-connections.png) · [Tokodon](tokodon.png) · [NeoChat](neochat.png) · [Workbench](workbench.png) · [Jami](jami.png) · [Furnace](furnace.png)
