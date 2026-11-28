# Verifica Linux x86-64 — audio e cattura

Il 27 novembre 2026 sono stati verificati tre cicli completi su Ubuntu 24.04 x86-64 in una VM KVM temporanea. Il launcher ha installato il ref Flathub firmato nel profilo di prova, aggiornato il pacchetto, avviato entrambe le versioni e rimosso l'app senza cancellare i dati sintetici. Le catture mostrano le finestre aperte dal motore nativo.

| App | Aggiornamento | Prove |
| --- | --- | --- |
| Nuclear | 1.54.2 → 1.56.4 | [Log](nuclear/lifecycle.log) · [Prima](nuclear/previous.png) · [Dopo](nuclear/current.png) |
| Live Captions | 0.4.1 → 0.4.2 | [Log](live-captions/lifecycle.log) · [Prima](live-captions/previous.png) · [Dopo](live-captions/current.png) |
| Ksnip | 1.10.1, runtime KDE 5.15-24.08 → 5.15-25.08 | [Log](ksnip/lifecycle.log) · [Prima](ksnip/previous.png) · [Dopo](ksnip/current.png) |

La prova esegue i comandi `InstallExpected`, `Launch` e `Uninstall` di NativeBackend. Dopo il primo download ha riportato il ref al commit storico firmato e provato il rifiuto dell'aggiornamento e della rimozione quando il pacchetto risulta modificato fuori dal launcher. Ha quindi registrato il commit storico nel solo profilo temporaneo per verificare la migrazione. Il lancio si è svolto via Xvfb e D-Bus privati in un namespace con la sola interfaccia di loopback. Per ogni ref ha controllato firma, architettura, stato, collegamento desktop, icona esportata e conservazione dei dati.

Il pacchetto [Ksnip su Flathub](https://flathub.org/apps/org.ksnip.ksnip) è mantenuto dalla community. Il progetto raccomanda quel ref nel [README upstream](https://github.com/ksnip/ksnip#linux). Il runtime storico era fuori supporto per Ksnip e Live Captions. La schermata di Live Captions mostra il primo avvio con audio sintetico, mentre quella di Ksnip mostra la barra di cattura inattiva. La prova non esegue la trascrizione o la cattura. Nuclear su Flathub mostra un avviso sul download di yt-dlp nel namespace offline. Il [DEB GitHub ufficiale](https://github.com/nukeop/nuclear/releases) 1.56.0 → 1.56.4 ha passato un controllo indipendente via apt, con [log](nuclear-deb/lifecycle.log) e schermate [prima](nuclear-deb/previous.png) e [dopo](nuclear-deb/current.png). Il launcher usa il ref [Flathub raccomandato da Nuclear](https://github.com/nukeop/nuclear/blob/master/packages/docs/user-manual/installation.md). Streaming e aggiornamento del DEB direttamente dal launcher sono fuori da questa prova.

L'esecuzione prova il motore nativo Linux x86-64, con 149 test core passati e la compilazione della libreria nativa riuscita. Easy Effects resta fuori dalla matrice perché il primo avvio della versione precedente è terminato con codice 134 nel guest.
