# UltiMaker Cura su Linux x64

Il 27 novembre 2026 il launcher ha installato UltiMaker Cura 5.13.0, aperto la finestra iniziale, aggiornato l'app alla 5.14.0, aperto la nuova versione e rimosso l'app da un profilo temporaneo Ubuntu 24.04 amd64. Abbiamo verificato versione, dimensione e SHA-256 dei due AppImage ufficiali. La rimozione ha eliminato i pacchetti correnti e precedenti, la voce desktop e l'icona, preservando i dati di prova. [Finestra 5.13.0](open-old.png), [finestra 5.14.0](open-new.png), [log](native-lifecycle.txt).

La prova è passata dal NativeBackend. L'installazione iniziale ha usato i metadati GitHub verificati del tag precedente. L'aggiornamento ha rilevato il tag corrente dal redirect pubblico e ha selezionato l'AppImage dalla ricevuta per quel tag. La quota dell'API anonima era esaurita nel guest. Entrambe le aperture hanno mantenuto i parametri predefiniti, con connettività limitata a loopback. La classe rilevata è `UltiMaker-Cura` e coincide con quella usata nella voce desktop. Il test si è fermato al benvenuto, prima di configurare una stampante o aprire un progetto. [Release 5.13.0](https://github.com/Ultimaker/Cura/releases/tag/5.13.0), [release 5.14.0](https://github.com/Ultimaker/Cura/releases/tag/5.14.0), [licenza LGPL-3.0](https://github.com/Ultimaker/Cura/blob/5.14.0/LICENSE), [sonda Rust](../pro-371-linux-desktop-expansion/verify_native_appimages.rs).

La ricetta vale per Linux x64 con FUSE 2 e autorizza i pacchetti dei due tag provati. Un tag successivo può essere rilevato ma richiede una nuova ricevuta prima di poterlo installare.

## Sniffnet

Sniffnet 1.5.1 ha completato l'installazione e il controllo di dimensione e SHA-256. La richiesta di apertura è partita, ma l'app si è chiusa prima di mostrare una finestra. Una seconda esecuzione offline con la stessa AppImage ha riportato `Library libxkbcommon-x11.so could not be loaded`. La ricetta rimane fuori da questo lotto. La prova non ha attivato la cattura di rete né modificato le dipendenze del guest. [Release 1.5.1](https://github.com/GyulyVGC/sniffnet/releases/tag/v1.5.1), [log del blocco](sniffnet-startup.txt).
