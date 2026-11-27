# NAPS2 su macOS Apple Silicon

Il 27 novembre 2026, NativeBackend su MacBook Air ha installato NAPS2 8.6.0 dal PKG ufficiale, aggiornato a 8.6.1 e trasferito la copia nel Cestino temporaneo. Lo smoke test `console --help` di 8.6.1 è passato senza rete. I dati sintetici e la ricevuta del sistema sono rimasti invariati. La GUI è rimasta chiusa.

La ricetta richiede Apple Silicon e macOS 12 o successivi. I due pacchetti hanno digest esatti, Team ID `2P9Q2K3XM9` e notarizzazione valida. Il motore estrae soltanto `NAPS2.app` in un'area temporanea senza rete, verifica il bundle `com.naps2.desktop` e conserva una ricevuta del launcher per aggiornamento e rimozione.

[Log del ciclo](cycle.log) · [Rilasci ufficiali](https://github.com/cyanfish/naps2/releases)
