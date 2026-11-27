# PRO-371 · Déjà Dup, Rnote e Mixxx su Linux x86-64

Il 27 novembre 2026, su VPS Strumenti, tutte e tre le app hanno completato il ciclo con il NativeBackend di Sh*ftAnything. Per ogni app il launcher ha installato il Flatpak ufficiale, aperto due revisioni stabili, aggiornato alla revisione corrente e rimosso il pacchetto. Dati sintetici invariati e nessuna app o runtime residuo nei tre profili privati.

## Pacchetti, tempi e finestre

| App | Flatpak stable x86-64 | Versioni osservate | Installa | Apri prima / dopo | Aggiorna | Rimuovi | Ciclo incluso seed |
|---|---|---|---:|---:|---:|---:|---:|
| Déjà Dup | `org.gnome.DejaDup` | 50.1 → 50.2 | 33,69 s | 4,02 / 3,60 s | 5,73 s | 2,72 s | 88,94 s |
| Rnote | `com.github.flxzt.rnote` | 0.14.1 → 0.14.2 | 31,90 s | 3,78 / 4,05 s | 4,62 s | 2,22 s | 54,81 s |
| Mixxx | `org.mixxx.Mixxx` | 2.5.2 → 2.5.6 | 39,21 s | 3,66 / 3,32 s | 6,03 s | 2,82 s | 69,47 s |

I tempi sono misure su un solo host. Installa include il runtime richiesto. Il tempo del ciclo comprende il seed della revisione precedente ed esclude la pulizia finale del runtime. Le dimensioni del solo download dell'app riportate da Flathub durante la prova sono 56 MB per Déjà Dup, 20 MB per Rnote e 36 MB per Mixxx. [Déjà Dup](https://flathub.org/apps/org.gnome.DejaDup) · [Rnote](https://flathub.org/apps/com.github.flxzt.rnote) · [Mixxx](https://flathub.org/apps/org.mixxx.Mixxx)

| App | Revisione OSTree precedente | Revisione OSTree corrente |
|---|---|---|
| Déjà Dup | `9e539b8bea56e443f9f6cc9d82503db292baf542f4c1d1f45f95dd3bafcdc851` | `b85783b490edcc7f761cc845feb1bb754cf3d4544f71a7c8a40f1186a5e45a13` |
| Rnote | `ce33a86c325529f9d39b1ec79a8a454c458de69c803d1418edb5b2ca01fb3dd8` | `82614efad294842f992a9c7e059f18820ea32a6ab7f9602faed2fe23a0c54433` |
| Mixxx | `8a0612dee35359f71e4f14388feb5863d9876d0cff234db217a41aae9885c31e` | `75cd677a581f17cac06479db9e3dbfc2f83d399afe3e628b34a63d70559a651d` |

Le sei revisioni hanno una firma GPG valida di Flathub, chiave primaria `4184DD4D907A7CAE`. La route richiede repo e ref esatti già associati alle schede. La verifica dell'editore risulta attiva via account GNOME per Déjà Dup, via GitHub per Rnote e via dominio `mixxx.org` per Mixxx. Le istantanee API sono in [publishers.json](publishers.json), le firme per revisione in `evidence/<id>/{previous,current}.gpg.txt` e le cronologie in [history.log](history.log). [Sorgente Déjà Dup](https://gitlab.gnome.org/World/deja-dup) · [Sorgente Rnote](https://github.com/flxzt/rnote) · [Sorgente Mixxx](https://github.com/mixxxdj/mixxx)

Ho letto ciascuno dei sei PNG del primo ciclo. Déjà Dup mostra la scelta tra backup e ripristino in [50.1](evidence/d-j-dup-backups/previous.png) e [50.2](evidence/d-j-dup-backups/current.png). Rnote mostra il canvas vuoto in [0.14.1](evidence/rnote/previous.png) e [0.14.2](evidence/rnote/current.png). Mixxx presenta la selezione iniziale della cartella musica in [2.5.2](evidence/mixxx-dj-software/previous.png) e [2.5.6](evidence/mixxx-dj-software/current.png). In un secondo profilo sintetico il tentativo di scegliere la cartella ha lasciato aperto il selettore. Anche in questa ripetizione l'aggiornamento e la rimozione sono riusciti e i runtime sono stati rimossi. [Log seconda prova](mixer-followup.log) I file `*.properties.txt` mostrano l'identità delle finestre. Le icone PNG attuali usate dal catalogo sono state scaricate e lette. Ogni app ha esportato la sua icona, salvata come `package-icon.png`.

## Riproducibilità e ambito

- Le [ricette Flathub](../../../crates/core/src/flathub.rs) passano da 23 a 26 ref rispetto a `ff517dbb2d0d29189e773c567e1597e7891dde68`. Test `reviewed_flatpak_ids_match_canonical_curated_channels_only_on_linux_x64` superato nella build x86-64 scratch. [Test e build](core-test.log)
- [Runner Rust](native-cycle.rs) compilato da archive e patch su VPS1. SHA256 `a72e8bea76591352cbeba2b0c87e12bad5fc88e790207cc0e152875a1274880f`. App e runtime provati su VPS Strumenti, scratch `/var/tmp/shift-flatpak-desktop.HtjJ7bJn`. [Lifecycle](lifecycle.log)
- La revisione storica firmata è stata preparata dalla CLI e associata al profilo controllato del runner. Prima dell'associazione il motore ha riconosciuto la sostituzione come copia gestita esternamente e ha rifiutato azioni che la sovrascrivessero o rimuovessero. L'aggiornamento verso la versione corrente è avvenuto con il motore, da un digest diverso.
- Apertura tramite il backend dentro Flatpak con rete isolata (solo `lo`), Xvfb e DBus separati. Permessi ridotti, senza accesso a filesystem host, periferiche o audio. Sono stati rimossi solo app e runtime dei profili di prova dopo aver ricontrollato il file dati sintetico.
- La prova riguarda il motore nativo, le finestre e la gestione del pacchetto. Restano fuori i clic nella UI Tauri, backup/ripristino di file reali, editing e riproduzione audio.
