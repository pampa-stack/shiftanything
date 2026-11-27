# PRO-371 · Settimo lotto desktop Flathub su Linux x86-64

Prove del 27 novembre 2026 su VPS Strumenti. Il runner NativeBackend è compilato offline su VPS1. Ogni profilo usa installazione Flatpak privata, Xvfb, sessione D-Bus e window manager separati. La rete del sandbox espone il solo loopback. File dell'host, periferiche e audio sono esclusi.

Corrispondenza tra sorgente della scheda e ref stabile di publisher verificato. [DB Browser for SQLite](https://flathub.org/apps/org.sqlitebrowser.sqlitebrowser) · [Showtime](https://flathub.org/apps/org.gnome.Showtime) · [Strawberry](https://flathub.org/apps/org.strawberrymusicplayer.strawberry) · [Ruffle](https://flathub.org/apps/rs.ruffle.Ruffle) · [Tux Paint](https://flathub.org/apps/org.tuxpaint.Tuxpaint) · [Dolphin](https://flathub.org/apps/org.kde.dolphin) · [Synfig Studio](https://flathub.org/apps/org.synfig.SynfigStudio) · [KiCad](https://flathub.org/apps/org.kicad.KiCad)

L'installazione dal motore include il runtime ufficiale. Dopo il seed della revisione precedente via Flatpak si verifica la protezione della copia gestita all'esterno. La revisione storica viene poi registrata come gestita nel solo profilo sintetico. Quindi il motore apre, aggiorna al digest corrente, riapre e rimuove. Nessun account, file personale o clic nella UI Tauri.

## Risultati

Ciclo completato per 7 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| DB Browser for SQLite 3.13.1 → 3.13.1 | 42,52 | 6,16 / 5,92 | 5,23 | 3,18 | 75,22 |
| Showtime 50.0 → 50.1.1 | 37,62 | 7,07 / 5,70 | 5,41 | 3,75 | 70,68 |
| Ruffle 0.8.0 → 0.8.0 | 40,58 | 10,92 / 10,63 | 4,77 | 3,72 | 82,53 |
| Tux Paint 0.9.35 → 0.9.35 | 42,48 | 10,91 / 10,97 | 7,16 | 5,53 | 250,38 |
| Dolphin 26.08.3 → 26.08.3 | 49,80 | 5,96 / 5,66 | 5,43 | 4,02 | 81,94 |
| Synfig Studio 1.5.3 → 1.5.3 | 37,86 | 8,23 / 6,76 | 7,54 | 4,33 | 205,59 |
| KiCad 10.0.7 → 10.0.8 | 103,61 | 11,34 / 11,99 | 13,04 | 5,97 | 226,58 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| DB Browser for SQLite | `app/org.sqlitebrowser.sqlitebrowser/x86_64/stable` | `56cf9737dbc30b9154a082f4e51a13d03126e2926aa30fea4ad7fb3a87511c9d` | `0ab811662bd9a64e32f82648425a1d2e797e747fcb83be28c29addf62b5de4ae` |
| Showtime | `app/org.gnome.Showtime/x86_64/stable` | `4e606152a4522fbf1f367edadeb0f803e735029295103f1441f6153349d507bc` | `58989bd5eaea18f0a811e019f65064cee5b9a5fd57c4e82fd62523eb28f48c14` |
| Ruffle | `app/rs.ruffle.Ruffle/x86_64/stable` | `a8d37129b1e8c6fce7913f5bca80dfcf4ae58eed2d533e9908dddaa1db59b499` | `8699385915a1755c17ca2f30417c60ae9c3169e0fef931ac76c0fbabba15df51` |
| Tux Paint | `app/org.tuxpaint.Tuxpaint/x86_64/stable` | `c33d448091967bcdc444abfdf13674991297729489e74694ba80cdcaedfd797c` | `84493b93145fbcf30baa2974a12530d5b41c82c3820f76bcc253dda4c78b3571` |
| Dolphin | `app/org.kde.dolphin/x86_64/stable` | `b305226e54014b0e8eca9cabfa7ea85c2c84d3d96f7fd36117f993e39c4d38cf` | `ea8c4005fabd629392cdf6872d64f2283998f218924d37998403b8f8657cdf2b` |
| Synfig Studio | `app/org.synfig.SynfigStudio/x86_64/stable` | `6d88a1781e7d3cb80533784a75e48f54f33b7e623801fd793ad636b30ee8a177` | `945f078467290baafb2ae8b79445a96f0cb1b79e5f32485efe15dd0201931e4b` |
| KiCad | `app/org.kicad.KiCad/x86_64/stable` | `256130c735cc7f3a032c0e3bfc78e44848a768e6096390f95c82b23de9c920db` | `d59131f226fe88b11529c1ac55e8cbae33525e0b2e72b78d85b7c4ad3fe8b7d5` |

Le catture mostrano l’apertura delle app nelle due revisioni.

- DB Browser for SQLite. Editor con menu Database Structure, navigazione dei dati e tab Execute SQL in entrambe le revisioni. Database vuoto e nessuna connessione esterna. [Prima](evidence/db-browser-for-sqlite/previous.png) · [Dopo](evidence/db-browser-for-sqlite/current.png) · [Log](evidence/db-browser-for-sqlite/lifecycle.log)
- Showtime. Finestra Video Player con menu e pulsante Open in entrambe le revisioni. Nessun video personale caricato. [Prima](evidence/showtime/previous.png) · [Dopo](evidence/showtime/current.png) · [Log](evidence/showtime/lifecycle.log)
- Ruffle. Finestra con menu player e pannello Open File or URL, impostazioni e comandi visibili. Nessun file Flash o sito remoto aperto. [Prima](evidence/ruffle/previous.png) · [Dopo](evidence/ruffle/current.png) · [Log](evidence/ruffle/lifecycle.log)
- Tux Paint. Tavolozza e pennelli sull’editor a schermo intero in entrambe le revisioni. Al primo tentativo di riapertura consecutiva l’app ha applicato il proprio limite di 30 secondi. La riapertura e la rimozione già autorizzate sono riuscite dopo l’attesa, senza cambiare l’opzione di avvio. [Prima](evidence/tux-paint/previous.png) · [Dopo](evidence/tux-paint/current.png) · [Log](evidence/tux-paint/lifecycle.log)
- Dolphin. File manager visibile con barra Home, Places e cartella dei dati di prova in entrambe le revisioni. Nessuna cartella personale aperta. [Prima](evidence/dolphin/previous.png) · [Dopo](evidence/dolphin/current.png) · [Log](evidence/dolphin/lifecycle.log)
- Synfig Studio. Editor aperto con canvas, strumenti, livelli e timeline in entrambe le revisioni. Il build precedente cerca una cartella legacy sotto HOME. Nel banco la cartella .config è stata resa disponibile soltanto nel profilo privato tramite persist; sono rimasti attivi il divieto di accesso all’host e la rete solo loopback. [Prima](evidence/synfig-studio/previous.png) · [Dopo](evidence/synfig-studio/current.png) · [Log](evidence/synfig-studio/lifecycle.log)
- KiCad. Finestra Welcome to KiCad 10.0 aperta e leggibile in entrambe le revisioni, con Project Files dietro il wizard. La prova ha usato il runtime GNOME SDK 51 indicato dal manifesto firmato per entrambi i build. [Prima](evidence/kicad/previous.png) · [Dopo](evidence/kicad/current.png) · [Log](evidence/kicad/lifecycle.log)

Strawberry 1.2.30 → 1.2.31 ha completato le azioni native. La prima cattura mostra l’interfaccia con una finestra di sponsorizzazione parzialmente nera sopra la libreria. Cattura aggiuntiva in corso; Strawberry è assente da questa patch.

Runner di prova basato su `dcad35380ce1dc733be178b6a7f05d259c1a627c`. [Test Rust](core-test.log) · [Metadati e digest](verified-proofs.json)
