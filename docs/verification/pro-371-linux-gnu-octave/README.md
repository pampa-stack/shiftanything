# GNU Octave su Linux x64

Il 28 novembre 2026 il NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso GNU Octave su Ubuntu 24.04 amd64. Il launcher aggiunge `--gui` per l'ID `org.octave.Octave`, come prevede la documentazione GNU, perché l'eseguibile avvia di default la CLI. Entrambe le revisioni aprono l'editor e la console grafica 11.3.0. Il test ha preservato il file sintetico. [Prima](gnu-octave/previous.png) · [Dopo](gnu-octave/current.png) · [Ciclo](gnu-octave/lifecycle.log)

Il progetto GNU distribuisce GNU Octave con licenza GPL-3.0-or-later e la relativa wiki indica il pacchetto Flathub. Le due revisioni portano la firma Flathub `4184DD4D907A7CAE`. [Requisiti di avvio GNU](https://docs.octave.org/latest/Command-Line-Options.html) · [Guida Octave per Linux](https://wiki.octave.org/Octave_for_GNU/Linux#Flatpak) · [Pacchetto Flathub](https://flathub.org/en/apps/org.octave.Octave)

- 11.3.0 · `475a1d58a15fb70519407398d01984eb1a14a50842e3abe7646cc78a4d96191f`. [Firma](gnu-octave/previous.signature.txt) · [Pacchetto](gnu-octave/previous.package.txt)
- 11.3.0 · `a37e6c8b9898ad580b4666407cb324fe8cd6e18dd1860fa811028b34575c577f`. [Firma](gnu-octave/current.signature.txt) · [Pacchetto](gnu-octave/current.package.txt)

L'aggiornamento ha cambiato la revisione Flatpak della stessa versione. Gli avvii di prova escludono rete esterna, file dell'host e chiamate Flatpak sul bus di sessione. Il manifest distribuito consente normalmente queste chiamate. Il test copre il ciclo del launcher e l'avvio grafico. [Isolamento](gnu-octave/sandbox.txt) · [Manifest Flathub](https://github.com/flathub/org.octave.Octave/blob/master/org.octave.Octave.yaml)

SHA-256 del probe candidato `4e352e141403d581d9fb926f4880eaf4a8d4cb674a8c8ed4f42fcd893ef659dd`.
