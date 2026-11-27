# PRO-371 · AppImage ufficiali su Linux x86-64

VPS2, Ubuntu 24.04.5, 27 novembre 2026. Pacchetti scaricati dal motore NativeBackend, con dimensione e SHA-256 ricontrollati sui file installati. Avvio diretto e dal menu su Xvfb in un profilo Bubblewrap, con accesso di rete esterno escluso. Aggiornamenti e rimozioni hanno lasciato intatto il file sentinella del profilo.

| App | Versioni | Risultato | Prove |
| --- | --- | --- | --- |
| draw.io Desktop | 32.6.10 → 32.7.4 | Installazione, editor in entrambe le versioni, aggiornamento, menu e rimozione | [Log](full-interface/drawio/cycle.log), [precedente](full-interface/drawio/open-old.png), [corrente](full-interface/drawio/open-new.png) |
| Audacity 4 | 4.0.1 → 4.0.2 | Installazione, editor con configurazione iniziale in entrambe le versioni, aggiornamento, menu e rimozione | [Log](full-interface/audacity/cycle.log), [precedente](full-interface/audacity/open-old.png), [corrente](full-interface/audacity/open-new.png) |
| Nextcloud Desktop | 35.0.2 → 35.0.3 | Installazione, schermata di connessione in entrambe le versioni, aggiornamento, menu e rimozione | [Log](full-interface/nextcloud-desktop/cycle.log), [schermata](full-interface/nextcloud-desktop/open-new.png) |

Nextcloud richiede un server per sincronizzare. Il test ha aperto il dialogo iniziale senza collegarsi a un server o usare credenziali. Il codice e le stelle appartengono a [nextcloud/desktop](https://github.com/nextcloud/desktop), i pacchetti provengono da [nextcloud-releases/desktop](https://github.com/nextcloud-releases/desktop).

I log riportano URL, byte e SHA-256 delle due versioni. Le icone provengono dall'AppImage corrente di ogni app. [Audacity 4.0.2](https://github.com/audacity/audacity/releases/tag/Audacity-4.0.2) · [draw.io 32.7.4](https://github.com/jgraph/drawio-desktop/releases/tag/v32.7.4) · [Nextcloud 35.0.3](https://github.com/nextcloud-releases/desktop/releases/tag/v35.0.3)

AppFlowy 0.15.0 ha completato l'installazione, con schermo nero nell'avvio isolato. Trilium 0.106.1 ha completato l'installazione, con avvio bloccato dal sandbox Chromium del suo AppImage. Entrambe rimangono fuori dalla patch. [AppFlowy 0.15.0](https://github.com/AppFlowy-IO/AppFlowy/releases/tag/0.15.0) · [Trilium 0.106.1](https://github.com/TriliumNext/Trilium/releases/tag/v0.106.1)

Suite core 133 pass, 1 ignored. [Log](final-core-tests.log)
