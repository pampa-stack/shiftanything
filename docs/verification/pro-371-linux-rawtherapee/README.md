# RawTherapee su Linux x64

Il 28 novembre 2026 NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso RawTherapee 5.12 → 5.13 su Ubuntu 24.04 amd64 in un guest isolato. Entrambe le versioni hanno aperto una foto sintetica nell’editor con la rete disattivata. La rimozione ha eliminato AppImage, voce del menu e icona, lasciando intatto il file di prova. [Prima](open-old.png) · [Dopo](open-new.png) · [Ciclo](lifecycle.log)

Gli AppImage ufficiali sono distribuiti sotto GPL-3.0-or-later. Il test verifica URL, byte e SHA-256 e usa FUSE 2 della distribuzione. Il pacchetto 5.12 viene selezionato dai metadati storici della release con il checksum pubblicato, la versione 5.13 via GitHub API. [Pacchetto e checksum 5.12](https://github.com/RawTherapee/RawTherapee/releases/tag/5.12) · [Download 5.13](https://www.rawtherapee.com/downloads/5.13/) · [Licenza](https://github.com/RawTherapee/RawTherapee/blob/5.13/LICENSE) · [Ricevute](upstream-receipts.tsv)

Le nuove release richiedono una ricevuta esplicita con asset, dimensione e checksum. Lo scope verificato è Linux x64.

SHA-256 del probe candidato `3d556dcd9e2d366b399ccf718742c883d6aa5639d12cd942d261dc6360e476ba`.
