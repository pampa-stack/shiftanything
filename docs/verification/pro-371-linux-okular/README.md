# Okular su Linux x86_64

Prova del 27 novembre 2026. Runner compilato offline da `b8e48d5` più la ricetta Okular con SHA256 `c0d6359d10129adf6cf1757feefebfea1a5df2693584a5a7fc7bc4afd93193d3`, verificato dopo il trasferimento.

Il ref stabile Flathub verificato da KDE è `app/org.kde.okular/x86_64/stable`. [Download KDE](https://okular.kde.org/download/) · [Scheda Flathub](https://flathub.org/apps/org.kde.okular) · [Sorgente KDE](https://invent.kde.org/graphics/okular)

| Versione | Revisione OSTree | Prova |
|---|---|---|
| 26.04.3 | `c294a8e2bb752bc9fc283ff6a3a1c4ed1dc0bf83727e3906cd2a4f0f08902d7d` | [Firma](previous.gpg.txt) · [Finestra](previous.png) |
| 26.08.3 | `6550bed86324b4c526cc08709404e6ec74d2abeb7ce2044df4f118cce818f6a1` | [Firma](current.gpg.txt) · [Finestra](current.png) |

NativeBackend ha installato 26.08.3 in un profilo temporaneo. Il banco ha selezionato il commit stabile precedente con `flatpak update --commit`, verificandone la firma. Il motore ha rifiutato aggiornamento e rimozione della revisione così sostituita da un comando esterno. Dopo aver registrato la proprietà della fixture, NativeBackend ha lanciato 26.04.3, aggiornato a 26.08.3, lanciato di nuovo e rimosso l'app. Il digest del file sintetico è rimasto uguale dopo aggiornamento, rimozione e pulizia delle dipendenze del profilo.

Gli screenshot Xvfb mostrano per entrambe le versioni il menu e la schermata di benvenuto. La rete dell'app era limitata al loopback. La prova riguarda avvio e ciclo del motore. Apertura o modifica di un PDF e clic nella finestra Tauri restano da verificare.

- [Log del ciclo e pulizia](lifecycle.log)
- [Test matrice Linux x64](core-test.log)
- [Rete 26.04.3](previous.network.log) · [Rete 26.08.3](current.network.log)
- [Digest delle evidenze](SHA256SUMS)
