# Zotero su Mac Apple Silicon

Il launcher riconosce l'ultima versione dal canale stabile ufficiale Zotero per macOS, senza usare le release GitHub o il feed ArtCraft. Il controllo legge la destinazione del redirect HTTPS e accetta solo il percorso ufficiale che contiene la stessa versione nel nome del DMG. Un nuovo rilascio è visibile anche se manca la ricevuta, ma l'installazione e l'aggiornamento restano limitati ai DMG verificati. [Canale stabile macOS](https://www.zotero.org/download/client/dl?channel=release&platform=mac) · [Cronologia Zotero](https://www.zotero.org/support/changelog)

| Versione | Byte | SHA-256 |
| --- | ---: | --- |
| 10.0.8 | 192345844 | `bf6192a0745ae67efbb7ff4e7f1953d8b78f5eed7083f4eefa12b762ecddcabe` |
| 10.0.9 | 192346587 | `fa42a04c33028ee020ccf47e64b3e135ce2fc57ba9a099e9f467f280ea56a387` |

Entrambi i DMG contengono un bundle universale con ID `org.zotero.zotero` e Team ID Apple `8LAYR367YV` della Corporation for Digital Scholarship. Il binario ARM64 dichiara macOS 11. Il launcher abilita la ricetta su Apple Silicon, dove è stato eseguito il ciclo. Zotero Desktop è AGPL-3.0 e funziona gratuitamente in locale. [Sorgente e licenza](https://github.com/zotero/zotero) · [Istruzioni di installazione](https://www.zotero.org/support/installation) · [Controlli dei bundle](../pro-371-macos-air-gui-batch/zotero.log)

Il probe in questa cartella collega le librerie Rust reali del launcher ed esegue `ListVersions`, `InstallChosen`, `CheckForUpdates`, `InstallExpected` e `Uninstall`. Lo script usa una home temporanea e ricontrolla dopo ogni fase il bundle, la firma, Gatekeeper e il dato Zotero sintetico. L'accesso SSH a MacBook Air usa una sessione in background e il desktop appartiene all'utente. Nessun avvio GUI per Zotero, Kap o SimpleX Chat in questa prova.

Su MacBook Air con macOS 26.6.2 sono passati il download e l'installazione di 10.0.8, il rilevamento di 10.0.9 dal canale stabile, il download e l'aggiornamento a 10.0.9 e la rimozione nel Cestino temporaneo. Gatekeeper ha accettato il bundle dopo tutte e tre le azioni. Il dato sintetico è rimasto invariato e il profilo temporaneo è stato eliminato. [Log del ciclo nativo](native-cycle.log)
