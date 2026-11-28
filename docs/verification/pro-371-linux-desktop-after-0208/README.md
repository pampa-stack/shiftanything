# Mission Center, Boxes e Sound Recorder su Linux x64

Il 27 novembre 2026 il NativeBackend 0.20.8 ha completato installazione, apertura, aggiornamento, riapertura e rimozione su Ubuntu 24.04 amd64. Ogni app aveva un profilo, un display e un bus di sessione dedicati. Tutte hanno esportato l'icona e la voce nel menu applicazioni. Dopo l'aggiornamento e la rimozione il file sintetico risultava intatto. Tutte le revisioni sono firmate dalla chiave Flathub `4184DD4D907A7CAE`.

SHA-256 del probe con le ricette candidate `744fd3ea4ba31524e6ae41e347d0ec447ffaba29998a8e384d16808f9897f79f`.

## Mission Center

Entrambe le versioni mostrano il pannello CPU del guest. La 1.2.0 propone una configurazione facoltativa per rilevazioni aggiuntive. Il test ha lasciato la configurazione predefinita e ha usato i dati del guest. L'autore raccomanda il pacchetto Flathub e indica GPL-3.0-or-later. [Sito del progetto](https://missioncenter.io/) · [Pacchetto Flathub](https://flathub.org/apps/io.missioncenter.MissionCenter) · [Prima](mission-center/previous.png) · [Dopo](mission-center/current.png) · [Ciclo](mission-center/lifecycle.log)

- 1.1.0 · `a3b9f576e39afface73608f9752d2f5b571e0647ed9f6cb7c1ed238515a0ec36`. [Firma](mission-center/previous.signature.txt) · [Pacchetto](mission-center/previous.package.txt)
- 1.2.0 · `98dd962a3249d6d51682698040ef88f60bf415f497acee625c836be35c5158eb`. [Firma](mission-center/current.signature.txt) · [Pacchetto](mission-center/current.package.txt)

## GNOME Boxes

Entrambe le versioni aprono la finestra principale e la guida iniziale. La creazione di macchine virtuali è fuori dal test. GNOME indica il pacchetto Flathub. La licenza indicata dal pacchetto è LGPL-2.1-or-later. [Scheda GNOME](https://apps.gnome.org/Boxes/) · [Pacchetto Flathub](https://flathub.org/apps/org.gnome.Boxes) · [Prima](gnome-boxes/previous.png) · [Dopo](gnome-boxes/current.png) · [Ciclo](gnome-boxes/lifecycle.log)

- 49.1 · `b00f0bb9f4e39fc55aff316c262139bb30785f552b286c69379218d0a82c13cf`. [Firma](gnome-boxes/previous.signature.txt) · [Pacchetto](gnome-boxes/previous.package.txt)
- 50.0 · `476600abc122ef11ed4ec4cf880465e3834449447188e29db1a3c91e66abf47f`. [Firma](gnome-boxes/current.signature.txt) · [Pacchetto](gnome-boxes/current.package.txt)

## Sound Recorder

Entrambe le aperture mostrano la finestra principale con il pulsante di registrazione. Il test ha tenuto l'audio isolato e non ha effettuato registrazioni. La documentazione GNOME collega Flathub. Entrambe le revisioni riportano la versione 42.0 e l'aggiornamento ha sostituito la revisione OSTree e il runtime GNOME 49 con il 50. La licenza indicata dal pacchetto è GPL-2.0-or-later. [Documentazione GNOME](https://wiki.gnome.org/Apps%282f%29SoundRecorder%282f%29Documentation.html) · [Pacchetto Flathub](https://flathub.org/apps/org.gnome.SoundRecorder) · [Prima](gnome-sound-recorder/previous.png) · [Dopo](gnome-sound-recorder/current.png) · [Ciclo](gnome-sound-recorder/lifecycle.log)

- Revisione precedente `ebee3189f16472ae78cadc97bc4d94a4c91a910d28c1fabe342db67f7c2b960d`. [Firma](gnome-sound-recorder/previous.signature.txt) · [Pacchetto](gnome-sound-recorder/previous.package.txt)
- Revisione corrente `3800cfb927ef666da099097d76df00945c1474051d533d9ff421ea0be74532ca`. [Firma](gnome-sound-recorder/current.signature.txt) · [Pacchetto](gnome-sound-recorder/current.package.txt)
