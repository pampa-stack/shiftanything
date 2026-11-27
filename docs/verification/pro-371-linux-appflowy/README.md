# AppFlowy · ciclo Flatpak Linux x86-64

Il 27 novembre 2026 il motore nativo ha installato AppFlowy da Flathub in un profilo utente privato su VPS Strumenti. Il pacchetto è `app/io.appflowy.AppFlowy/x86_64/stable`, con licenza AGPL-3.0-only. Il catalogo conserva la dimensione documentata del download, 197.285.434 byte. [Scheda Flathub](https://flathub.org/apps/io.appflowy.AppFlowy) · [Metadati download](https://flathub.org/api/v2/summary/io.appflowy.AppFlowy)

| Release | Revisione Flatpak firmata |
|---|---|
| 0.15.0 | `749936f8d1f2092876bb89ecb18b8e14e220a30d42fb26940ca8abdbe6ad01ba` |
| 0.15.1 | `1f0281272643b47ff7534f91d87b8dbd75a895d57e58e90e9c393f91f75c42c3` |

L'installazione ha richiesto 38,6 s. Dopo la predisposizione della versione precedente, l'apertura dell'editor anonimo offline ha richiesto 16,3 s, l'aggiornamento 9,8 s, la riapertura 15,0 s e la rimozione 6,3 s. Il banco Xvfb ha assegnato un solo core logico al processo di avvio. I due screenshot mostrano l'editor con lo stesso spazio di lavoro, preservato dall'aggiornamento. Dopo la rimozione è rimasto il profilo privato. [Editor 0.15.0](previous.editor.png) · [Editor 0.15.1](current.editor.png) · [Persistenza](profile-persistence.txt)

Il banco ha prima sostituito il pacchetto con la versione precedente tramite Flatpak. Il motore ha rilevato la modifica esterna e ha rifiutato aggiornamento e rimozione. Il banco ha poi predisposto il record di proprietà per simulare una precedente installazione gestita. Le azioni successive sono state eseguite dal motore sui due pacchetti firmati. [Installazione](stage-install.log) · [Preparazione e rifiuto](stage-previous.log) · [Aggiornamento](stage-update.log) · [Rimozione](stage-remove.log) · [Firma precedente](previous.gpg.txt) · [Firma corrente](update.gpg.txt)
