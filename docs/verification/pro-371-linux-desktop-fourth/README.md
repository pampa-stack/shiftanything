# PRO-371 · Quarto lotto desktop Flathub su Linux x86-64

Prove del 27 novembre 2026 su VPS Strumenti, in due corsie. Il runner chiama NativeBackend dallo snapshot compilato offline su VPS1. Ogni app ha installazione e profilo Flatpak separati, Xvfb e sessione D-Bus privati. All'apertura la rete del sandbox è limitata al loopback. File dell'host, dispositivi, microfono e audio sono esclusi.

Corrispondenza tra sorgente della scheda e app ID verificato, firma del remoto Flathub e due revisioni del ramo stable. [Kate](https://flathub.org/apps/org.kde.kate) · [Ark](https://flathub.org/apps/org.kde.ark) · [Filelight](https://flathub.org/apps/org.kde.filelight) · [Glaxnimate](https://flathub.org/apps/org.kde.glaxnimate) · [Evolution](https://flathub.org/apps/org.gnome.Evolution) · [GNOME Contacts](https://flathub.org/apps/org.gnome.Contacts) · [CopyQ](https://flathub.org/apps/com.github.hluk.copyq) · [fre:ac](https://flathub.org/apps/org.freac.freac)

L'installazione da profilo vuoto include il runtime. Dopo aver installato la versione corrente con il launcher, il runner prepara la revisione precedente con il client Flatpak e verifica che il motore protegga la copia gestita esternamente. Poi associa la revisione storica al profilo sintetico e chiama il motore per aprire, aggiornare, riaprire e rimuovere il pacchetto. Le prove della finestra non includono account, file reali o clic nella UI Tauri.

## Risultati

Ciclo completato per 6 app. Due firme valide per app con chiave primaria Flathub `4184DD4D907A7CAE` e due revisioni stabili diverse. Ogni profilo verificato conserva il file sintetico dopo la rimozione di app e runtime.

| App e versioni | Installa (s) | Apri prima e dopo (s) | Aggiorna (s) | Rimuovi (s) | Ciclo totale (s) |
|---|---:|---:|---:|---:|---:|
| Kate 26.04.2 → 26.08.3 | 46,49 | 4,67 / 4,60 | 5,53 | 3,92 | 97,05 |
| Ark 26.08.3 → 26.08.3 | 43,42 | 4,68 / 4,18 | 4,62 | 2,90 | 66,77 |
| Filelight 26.08.2 → 26.08.3 | 38,09 | 5,12 / 4,41 | 4,22 | 3,32 | 61,93 |
| Glaxnimate 0.6.0 → 0.6.0 | 39,88 | 7,29 / 6,67 | 4,73 | 3,62 | 102,08 |
| Evolution 3.62.0 → 3.62.1 | 38,32 | 6,80 / 6,18 | 5,02 | 3,42 | 74,11 |
| fre:ac 1.1.7 → 1.1.7 | 28,70 | 5,66 / 5,50 | 5,02 | 3,32 | 75,01 |

| App | Ref stable x86-64 | Digest precedente | Digest corrente |
|---|---|---|---|
| Kate | `app/org.kde.kate/x86_64/stable` | `391d8f4fe21fa08c1df1fe0011cee7a3f48e2441275f68a342ec4ac2aeef2194` | `c697866017b8f501dfdd29cb09971f1309dcb2b86955e495677f3a8e9008bf82` |
| Ark | `app/org.kde.ark/x86_64/stable` | `908876d27bf2ede02d536cdc5f8369a94ac7dc3a77665c6ef31260346c9e4397` | `8275aad8fadad4178df3f8939ac1f36f5e67f556cfc18455b0afc18348ddfeab` |
| Filelight | `app/org.kde.filelight/x86_64/stable` | `939300f48e847dd39f389f6b87d3437d8f7b8ca474f959805f6184f2ece4119e` | `b0344bab1ac77dbbce1ca21f0c1060adadb3dc0045c3e1f4bb3d4133920b735e` |
| Glaxnimate | `app/org.kde.glaxnimate/x86_64/stable` | `ef03dffc433cd98d3223608bb29e1c33d1a01c5fbc416ee3a7ff7b457351a09e` | `8a950bade30d9e153ceafb276ee22568588b99d03f6f9cb0c03644172e9dac39` |
| Evolution | `app/org.gnome.Evolution/x86_64/stable` | `2d34bb5810fa1b627de94125d03abb9f180755d5bae5dd9b114c26c0fb9bbf3d` | `053e6c5b734895039581eb45715e846987b5b8e3828952c76bd1e53e0bc7a0fd` |
| fre:ac | `app/org.freac.freac/x86_64/stable` | `25fe0b4974a73e40b3e98041059710d412ec31c35874388f17fa1aca7ba233d1` | `2851adb129f2b31009c4d249a6dc17e4d974db8cd854e122f97af00c6e752092` |

Ho letto entrambi gli screenshot e l'icona installata per ciascuna ricetta consegnata.

- Kate. Schermata iniziale dell'editor con Nuovo file, documenti recenti e sessioni vuote, su entrambe le revisioni. [Prima](evidence/kate/previous.png) · [Dopo](evidence/kate/current.png) · [Log](evidence/kate/lifecycle.log)
- Ark. Finestra di benvenuto del gestore archivi, senza archivi recenti. Il pacchetto cambia da un digest all'altro e aggiorna 7-Zip pur mantenendo l'etichetta 26.08.3. [Prima](evidence/ark/previous.png) · [Dopo](evidence/ark/current.png) · [Log](evidence/ark/lifecycle.log)
- Filelight. Finestra Overview con il comando Choose Folder. Nessuna scansione o eliminazione eseguita. [Prima](evidence/filelight/previous.png) · [Dopo](evidence/filelight/current.png) · [Log](evidence/filelight/lifecycle.log)
- Glaxnimate. Editor di animazioni, strumenti e pannelli di livelli e timeline. La modale di avvio sovrapposta appare nera nello screenshot Xvfb. [Prima](evidence/glaxnimate/previous.png) · [Dopo](evidence/glaxnimate/current.png) · [Log](evidence/glaxnimate/lifecycle.log)
- Evolution. Finestra Mail con cartelle locali, Contatti, Calendario e Attività. Un riquadro centrale appare nero in Xvfb. Nessun account configurato e nessuna email inviata. [Prima](evidence/evolution/previous.png) · [Dopo](evidence/evolution/current.png) · [Log](evidence/evolution/lifecycle.log)
- fre:ac. Finestra del convertitore audio con lista lavori vuota e consigli iniziali. La stessa versione applicativa 1.1.7 è distribuita su due revisioni firmate e cambia runtime Freedesktop da 25.08 a 26.08. [Prima](evidence/freac/previous.png) · [Dopo](evidence/freac/current.png) · [Log](evidence/freac/lifecycle.log)

Le versioni KDE come 26.08.3 sono preservate dall'etichetta Flatpak. Il motore valuta gli aggiornamenti attraverso il digest anche quando due pacchetti riportano la stessa versione.

Il runner è compilato da `2748427dd34326d0cc039e32a142a74330cca03a` più la matrice di prova. Le ricette verificate sono nel commit che include queste prove. Test della matrice Linux x86-64 superato. SHA256 dell'eseguibile `a305f18acdf1ae1f65665b026ea0a0dd030d9461cc33472026fe464fce5613c0`. Scratch su VPS Strumenti `/var/tmp/shift-flatpak-desktop.ck1FCGXU`. [Test Rust](core-test.log) · [Metadata, digest e tempi](verified-proofs.json) · [Storico Flatpak](history.log)
