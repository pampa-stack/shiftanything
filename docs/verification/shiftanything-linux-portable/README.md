# Archivio Linux x86-64

Il 27 novembre 2026, Sh*ftAnything 0.20.6 è partito dall'archivio estratto in Fedora 44 x86-64 su VPS2. Il test usa un utente standard, un profilo vuoto, Xvfb 1440×900 e un container senza rete. Versione, librerie dinamiche e finestra del catalogo verificate. Nessun pacchetto Sh*ftAnything installato nel container. [Catalogo su Fedora](catalog-fedora.png)

| File | Byte | SHA-256 |
| --- | ---: | --- |
| `ShiftAnything-0.20.6-linux-amd64.deb` | 38.227.714 | `0fe3f1b1bd82f45ef574d209b96349bec0ead83e9feaae6db142f1a01aab4a36` |
| `ShiftAnything-0.20.6-linux-amd64.tar.gz` | 41.239.820 | `6c2ca823e19cb6b3b23c97abd96caf59cc9314c494669d3a525e12083bb82621` |

Il digest del DEB corrisponde alla ricevuta dell'asset su GitHub. Due generazioni producono lo stesso archivio. Il generatore rifiuta checksum, versione e percorso di destinazione in conflitto. Il pacchetto richiede le librerie GTK 3 e WebKitGTK 4.1 della distribuzione e glibc 2.39 o più recente. Fedora 44 è stata verificata con GTK 3.24.52, WebKitGTK 2.54.2 e glibc 2.43. La prova copre avvio e caricamento del catalogo, mentre i cicli delle singole app restano nelle rispettive verifiche. L'archivio va riscaricato per aggiornare il launcher. La beta pubblica resta invariata. [Rilascio sorgente del DEB](https://github.com/pampa-stack/shiftanything/releases/tag/v0.20.6) · [WebKitGTK per Fedora](https://packages.fedoraproject.org/pkgs/webkitgtk/webkit2gtk4.1/)
