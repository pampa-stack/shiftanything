# Deskreen CE, OpenChamber e Recordly su Linux x86-64

Il 28 novembre 2026, il motore di Sh*ftAnything ha scaricato le AppImage ufficiali in una VM Ubuntu 24.04 su AX42. Per ciascuna app ha verificato dimensione e SHA-256, installato la prima versione, aperto la finestra, aggiornato e aperto la nuova versione. La rimozione ha cancellato il lanciatore dal menu Applicazioni e preservato le preferenze nel profilo temporaneo.

| App | Versioni provate | Finestre | Registro |
| --- | --- | --- | --- |
| Deskreen Community Edition | 3.2.15 → 3.2.16 | [Prima](deskreen/launch-old.png) · [Dopo](deskreen/launch.png) | [Log](deskreen/lifecycle.log) |
| OpenChamber | 2.8.0 → 2.8.1 | [Prima](openchamber/launch-old.png) · [Dopo](openchamber/launch.png) | [Log](openchamber/lifecycle.log) |
| Recordly | 1.3.3 → 1.4.0 | [Prima](recordly/launch-old.png) · [Dopo](recordly/launch.png) | [Log](recordly/lifecycle.log) |

L'avvio avviene con Bubblewrap e Xvfb in un ambiente senza rete. Deskreen CE mostra il messaggio di rete assente. OpenChamber si apre alla configurazione di OpenCode. Recordly apre la barra di registrazione. Queste prove riguardano il ciclo di gestione del launcher.

Per riprodurre, compila `verify.rs` come binario del crate nativo e salva l'eseguibile come `verify` accanto a `run.sh`. Per ogni riga di `assets.tsv`, salva nel sottodirettorio dell'app i metadati della release GitHub, nel formato `<tag>.json`, e il digest dell'asset ufficiale nel file `<tag>.sha256`. Avvia `bash run.sh <app>` nel guest `shift-launcher-qa`, con i namespace Bubblewrap disponibili e Xvfb, xdotool, DBus e Pillow installati.

[Deskreen](https://github.com/pavlobu/deskreen/releases) · [OpenChamber](https://github.com/openchamber/openchamber/releases) · [Recordly](https://github.com/webadderallorg/Recordly/releases)
