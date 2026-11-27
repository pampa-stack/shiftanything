# Mister Download Manager su Linux x64

Il 27 novembre 2026 NativeBackend 0.20.8 ha installato 0.5.0, aperto l'app, aggiornato a 0.5.2, riaperto e rimosso l'app in un guest Ubuntu 24.04 amd64. Ogni AppImage è stata scaricata in HTTPS con ricevuta per nome, dimensione e SHA-256. L'aggiornamento ha scoperto la release corrente attraverso l'API di GitHub. Per la versione precedente la prova ha ricostruito i metadati dalla pagina della release e dal file SHA-256 del progetto. [Linux e dipendenze](https://github.com/anefzaoui/misterdownloadmanager/blob/main/docs/getting-started.md#on-linux) · [Release 0.5.0](https://github.com/anefzaoui/misterdownloadmanager/releases/tag/v0.5.0) · [Release 0.5.2](https://github.com/anefzaoui/misterdownloadmanager/releases/tag/v0.5.2)

| Versione | Byte AppImage | SHA-256 |
| --- | ---: | --- |
| 0.5.0 | 20.625.912 | `d2c7c568240a938f1010610b49b5fe591a7f37db7070a593735e11ba125a3878` |
| 0.5.2 | 20.777.464 | `70bcad9677b754dfffd118cea1e9f0bcb4b7f28f8f70bd3128fbc1b3ad6a74f4` |

Le due aperture hanno usato HOME, display e portachiavi del guest, con rete limitata al loopback durante l'esecuzione dell'app. Nel guest minimale sono serviti FUSE e `libxkbcommon-x11-0` dai pacchetti Ubuntu. Il backend ha scritto icona e voce del menu, poi le ha rimosse insieme al pacchetto, conservando il file utente sintetico. La prova copre il ciclo del launcher e la finestra principale. Integrazione browser, estensioni, download e servizi di terzi restano fuori dalla prova. [Prima](open-old.png) · [Dopo](open-new.png) · [Log](lifecycle.log)

La licenza del progetto è GPL-3.0. L'icona proviene dal pacchetto upstream ed è stata adattata a 128×128. Il probe Linux ha SHA-256 `09108775a58b52bd3becabc7a935b8e5088aac097084a1dd9c6cf6036131d9d9`. I tag futuri richiedono una nuova ricevuta. [Progetto e licenza](https://github.com/anefzaoui/misterdownloadmanager)
