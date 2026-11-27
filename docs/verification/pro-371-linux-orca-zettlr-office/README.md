# OrcaSlicer, Zettlr e ONLYOFFICE Desktop su Ubuntu

Il 27 novembre 2026 VPS2, Ubuntu 24.04.5 x86-64, ha eseguito il ciclo NativeBackend con gli AppImage ufficiali. Download HTTPS, dimensioni e SHA-256 verificati, profili estemporanei, sandbox Bubblewrap, GUI Xvfb senza rete e dati campione conservati. Le tre app si sono aperte prima e dopo l'aggiornamento e dal menu Linux. I PNG e i log sono nelle tre sottocartelle.

| App | Versioni | Cosa mostra la finestra |
| --- | --- | --- |
| OrcaSlicer | 2.5.1 → 2.5.2 | Benvenuto e configurazione iniziale |
| Zettlr | 4.10.0 → 4.11.0 | Aggiornamento completato e avvio dell'editor dal menu |
| ONLYOFFICE Desktop Editors | 9.4.0 → 10.0.0 | Apertura locale di documenti, fogli, presentazioni e PDF |

OrcaSlicer richiede WebKitGTK 4.1, JavaScriptCore e xdg-dbus-proxy. Nella prova sono stati estratti in scratch da pacchetti Ubuntu firmati. Il DEB Sh*ftAnything richiede WebKitGTK e JavaScriptCore e la dipendenza Ubuntu WebKitGTK include xdg-dbus-proxy. Il ciclo copre l'avvio e la gestione, senza prove di slicing, stampa, cloud, editing documenti o servizi AI. [OrcaSlicer per Linux](https://github.com/OrcaSlicer/OrcaSlicer#linux) · [Licenza e disponibilità ONLYOFFICE Desktop](https://www.onlyoffice.com/open-source)

Le ricevute di queste app, Pencil2D, OpenShot, MuseScore Studio e LosslessCut usano repository, tag, nome file, byte e SHA-256 esatti. Il test esercita il selettore Linux x86-64 con quattordici asset di rilascio, usando un manifest vuoto come fallback da GitHub API.

Readest è stato tolto dal catalogo perché alcune opzioni locali di layout, temi e font sono riservate agli acquisti. Il catalogo scende a 531 schede e le tre pagine HTML vengono rigenerate dal JSON Rust. [Prezzi Readest](https://readest.com/pricing)
