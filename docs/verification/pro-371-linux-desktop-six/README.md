# PRO-371 · Terzo lotto desktop Flathub su Linux x86-64

Verifica del 27 novembre 2026 su VPS Strumenti. Il runner usa NativeBackend dallo snapshot di Sh*ftAnything compilato offline su VPS1. Ogni app usa HOME, XDG e installazione Flatpak per utente separati. Xvfb e DBus sono privati. Durante l'apertura le app sono isolate dalla rete e dal filesystem dell'host, senza dispositivi o server audio.

Per ogni candidato sono stati controllati editore verificato, corrispondenza tra scheda del catalogo e sorgente upstream, due digest nel ramo stable, e icona del catalogo. [QOwnNotes](https://flathub.org/apps/org.qownnotes.QOwnNotes) · [qView](https://flathub.org/apps/com.interversehq.qView) · [Czkawka](https://flathub.org/apps/com.github.qarmin.czkawka) · [Drawpile](https://flathub.org/apps/net.drawpile.drawpile) · [Kid3](https://flathub.org/apps/org.kde.kid3) · [Super Productivity](https://flathub.org/apps/com.super_productivity.SuperProductivity)

Il tempo di installazione include il runtime nel profilo vuoto. Il ciclo totale include la preparazione della revisione storica, firmata, via CLI. Dopo aver verificato la protezione delle copie gestite fuori dal launcher, il runner associa la revisione storica al profilo sintetico. Aggiornamento alla revisione attuale e rimozione avvengono attraverso il backend. Nessun test sui dati dell'utente, sui servizi online, sulle funzioni di editing delle app o sui clic nella UI Tauri.

## Risultati

Ciclo completato per 6 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| QOwnNotes 26.11.15 → 26.11.16 | 41,84 | 4,23 / 4,02 | 4,42 | 3,22 | 70,68 |
| qView 6.1 → 6.1 | 39,07 | 4,61 / 3,67 | 4,43 | 3,73 | 91,56 |
| Czkawka 9.0.0 → 10.0.0 | 33,83 | 4,02 / 4,01 | 4,42 | 3,12 | 90,17 |
| Drawpile 2.3.0 → 2.3.1 | 38,95 | 4,86 / 4,11 | 5,33 | 3,22 | 93,26 |
| Kid3 3.10.1 → 3.10.2 | 40,61 | 5,62 / 3,98 | 4,52 | 3,42 | 65,81 |
| Super Productivity v20.0.0 → v20.1.0 | 34,04 | 7,00 / 6,05 | 5,33 | 2,82 | 70,69 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| QOwnNotes | `app/org.qownnotes.QOwnNotes/x86_64/stable` | `6a4eb97df1cf527481e78c88b785945aaac5816ec3d8fefb4ec644e9699eb11d` | `c5f6310c3174e3b5b3b073b4d2a9b614a39e0a6aaff918fcbdacf1a7294bbb9b` |
| qView | `app/com.interversehq.qView/x86_64/stable` | `a50677be27a056415a3e16a4e368789f8bbfe0be6517e8d1554cd5233280ebf8` | `c543dd9cf1f7568893e54b40f34dc8f2178c5910f3d7a9fe709e6031f973bfda` |
| Czkawka | `app/com.github.qarmin.czkawka/x86_64/stable` | `f009f980a358f89da49e27265d02ccba6febf4a554ac3940d76826c1f80b90f8` | `55440fc853ba0039626c2fb7691791d341ce9c3f548fab3611dea39ee5b0d36c` |
| Drawpile | `app/net.drawpile.drawpile/x86_64/stable` | `8972cb96b0150bc440041c7a0f8d7fd122d1cb6b15ad71eea303aa03d50097c4` | `a65aace6e2a0459e1e80ed4abe64c2ad9b2156169f212dd18dd32d03879ed207` |
| Kid3 | `app/org.kde.kid3/x86_64/stable` | `1aaf7a340335401d180b37cd87869d7c1fcdb0e84f97d184a2f3b7293e3c8d80` | `689653a7e94ee38334b8dd9252effad4423702fbdcdb9d39d8d069ec91685386` |
| Super Productivity | `app/com.super_productivity.SuperProductivity/x86_64/stable` | `73566c9bbd598ae58b6f34d4c1c7496253a580727dab8eeb03e9bc2037c7670d` | `33d0c4f8fc49bb51cb2f26c5e49a260fc1912fa9fed585cb5136e15cb9f47ab8` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- QOwnNotes. Entrambe le revisioni mostrano la finestra di primo avvio con la scelta della cartella delle note. [Prima](evidence/qownnotes/previous.png) · [Dopo](evidence/qownnotes/current.png) · [Log](evidence/qownnotes/lifecycle.log)
- qView. Entrambe le revisioni aprono la finestra X11 qView con il visualizzatore vuoto. I due pacchetti riportano 6.1, ma l'aggiornamento cambia il digest e passa da KDE 6.10 a KDE 6.11. [Prima](evidence/qview/previous.png) · [Dopo](evidence/qview/current.png) · [Log](evidence/qview/lifecycle.log)
- Czkawka. L'interfaccia Hiccup mostra le modalità di ricerca e solo la cartella isolata. Il footer mostra 9.0.0 e 10.0.0. Nessuna ricerca o eliminazione eseguita. [Prima](evidence/czkawka/previous.png) · [Dopo](evidence/czkawka/current.png) · [Log](evidence/czkawka/lifecycle.log)
- Drawpile. Editor, pennelli e tavolozza visibili. La finestra Start è sovrapposta al canvas; la cattura del singolo editor mostra scura la zona coperta. Sessioni online non esercitate. [Prima](evidence/drawpile/previous.png) · [Dopo](evidence/drawpile/current.png) · [Log](evidence/drawpile/lifecycle.log)
- Kid3. Editor tag e controlli della finestra visibili, senza caricare o modificare audio. [Prima](evidence/kid3/previous.png) · [Dopo](evidence/kid3/current.png) · [Log](evidence/kid3/lifecycle.log)
- Super Productivity. Dashboard Today e barra del planner visibili nelle due versioni. Il primo tentativo ha incontrato un socket D-Bus nella directory di prova con path eccessivo. Il ciclo è riuscito riprendendo con un XDG_RUNTIME_DIR privato più corto. [Prima](evidence/super-productivity/previous.png) · [Dopo](evidence/super-productivity/current.png) · [Log](evidence/super-productivity/lifecycle.log)

Il tempo di Super Productivity somma installazione, preparazione e ripresa riuscita. Esclude il tentativo D-Bus fallito. Il test qView considera l'identità del pacchetto e il cambio di digest anche a etichetta invariata.

Il runner è compilato da `202bb9cd9c2785439b0fb2a25bbd2a652cae9637` più la matrice di prova. Il commit integra le sei ricette verificate. Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `105cebf6d0baed4bf214ef07451a371f4093f27222f655c8313cadbc8010fad0`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.VoI8r6Zy`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Storico Flatpak](history.log) · [Manifest SHA256](SHA256SUMS)

## Controlli visivi supplementari

- qView. Aperta con `Ctrl+O` un'immagine PPM sintetica, in sola lettura, su entrambe le revisioni. I colori del campione sono visibili prima e dopo. [Prima](evidence/qview/supplemental/native-qview-previous-fixture.png) · [Dopo](evidence/qview/supplemental/native-qview-current-fixture.png) · [Log](qview-visual-followup.log)
- Drawpile. Anche la cattura dell'intero display isolato mostra il pannello Start nero sovrapposto. Menu, strumenti, pennelli, tavolozza e parte del canvas appaiono. Nessuna modifica al pacchetto o al renderer. [Display intero](evidence/drawpile/supplemental/native-drawpile-current-visible.png) · [Log](drawpile-visual-followup.log)

Dopo i controlli supplementari la procedura ha ripulito nuovamente app e runtime di qView e Drawpile. VPS Strumenti aveva 46 GiB disco e 6,8 GiB RAM disponibili al controllo finale.

Una seconda prova qView ha aperto un’immagine sintetica su [entrambe](evidence/qview/supplemental/native-qview-previous-fixture.png) le [revisioni](evidence/qview/supplemental/native-qview-current-fixture.png). La [cattura intera Drawpile](evidence/drawpile/supplemental/native-drawpile-current-visible.png) conferma l’editor e l’area coperta dal pannello Start.
