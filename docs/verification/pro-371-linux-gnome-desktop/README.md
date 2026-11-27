# App GNOME su Linux x64

## Camera (Snapshot)

Il 27 novembre 2026 il NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso Camera in un profilo dedicato su Ubuntu 24.04 amd64. Su entrambe le versioni si è aperta la finestra principale con il menu e lo stato «No Camera Found», previsto perché il guest non aveva una webcam. Il test ha verificato la gestione dal launcher e la conservazione dei dati sintetici. La registrazione è fuori dal test. [Prima](snapshot/previous.png) · [Dopo](snapshot/current.png) · [Ciclo NativeBackend](snapshot/lifecycle.log)

GNOME pubblica Camera con licenza GPL-3.0-or-later e rimanda al suo pacchetto su Flathub. Entrambe le versioni erano firmate dalla chiave Flathub `4184DD4D907A7CAE`. [Camera su GNOME](https://apps.gnome.org/Snapshot/) · [Pacchetto Flathub](https://flathub.org/apps/org.gnome.Snapshot)

- 50.0 · `7cc41e5fdc76fd0749a459e7acb1975c043d73ebcb996d8f24e15b60e5c98efe`. [Firma](snapshot/previous.signature.txt) · [Pacchetto](snapshot/previous.package.txt)
- 51.0 · `8a1a93d2335c92f28d54590f2fa1ee60390bbfca4d89a1bcefe3bc688aa75301`. [Firma](snapshot/current.signature.txt) · [Pacchetto](snapshot/current.package.txt)
- SHA-256 del probe con ricetta candidata `891ddeba7da2f1c786a1582e6e30fc94dbb3966988bd0fa6c240564d4856baa0`.

## Fractal

Il 27 novembre 2026 il NativeBackend 0.20.8 ha installato Fractal, aperto la schermata di benvenuto, aggiornato l'app, riaperto e rimosso la nuova versione su Ubuntu 24.04 amd64. Il guest usava il portale Secret con un portachiavi di prova. Entrambe le finestre mostrano il pulsante «Log In» e lo stato offline previsto nel profilo isolato. Il ciclo ha preservato il file sintetico. L'accesso a Matrix e l'invio di messaggi sono fuori dal test. [Prima](fractal/previous.png) · [Dopo](fractal/current.png) · [Ciclo NativeBackend](fractal/lifecycle.log)

Fractal è pubblicato con licenza GPL-3.0-or-later. Il pacchetto è verificato su Flathub ed entrambe le versioni sono firmate dalla chiave Flathub `4184DD4D907A7CAE`. [Sorgente GNOME](https://gitlab.gnome.org/World/fractal) · [Pacchetto Flathub](https://flathub.org/apps/org.gnome.Fractal)

- 14.2 · `079decd4f0521993570fd066c64a010a375cb4bae1990309395b3485dfd1ee42`. [Firma](fractal/previous.signature.txt) · [Pacchetto](fractal/previous.package.txt)
- 14.3 · `08b88c9967887efa7da1d162c0e3c2454a165d28d0d75a3798ca10f34f8e86bb`. [Firma](fractal/current.signature.txt) · [Pacchetto](fractal/current.package.txt)
- SHA-256 del probe con ricetta candidata `891ddeba7da2f1c786a1582e6e30fc94dbb3966988bd0fa6c240564d4856baa0`.
