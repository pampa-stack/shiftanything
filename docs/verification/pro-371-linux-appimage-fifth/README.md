# PRO-371 · Joplin e Cryptomator su Linux x86-64

VPS2, Ubuntu 24.04.5, 27 novembre 2026. NativeBackend ha scaricato e installato le AppImage pubblicate dagli autori, poi ha riconosciuto l'installazione, aggiornato, avviato e rimosso entrambe le app. Il test confronta SHA-256 e byte dei file installati con i metadati GitHub, verifica il launcher desktop e l'icona estratta da ciascuna AppImage, controlla la rimozione dei file gestiti e confronta l'hash del file sentinella. Xvfb e Bubblewrap isolano display, file e rete delle app. Il download accede a GitHub prima degli avvii offline.

| App | Versioni | Installazione / aggiornamento / rimozione | Evidenza GUI |
| --- | --- | --- | --- |
| Cryptomator | 1.19.6 → 1.19.7 | 5.363 / 8.561 / 0.040 s | [Log](full-interface/cryptomator/cycle.log), [precedente](full-interface/cryptomator/open-old.png), [corrente](full-interface/cryptomator/open-new.png), [menu](full-interface/cryptomator/open-menu.png) |
| Joplin | 3.7.22 → 3.7.24 | 6.776 / 9.058 / 0.120 s | [Log](full-interface/joplin/cycle.log), [precedente](full-interface/joplin/open-old.png), [corrente](full-interface/joplin/open-new.png), [menu](full-interface/joplin/open-menu.png) |

Cryptomator ha mostrato il dialogo iniziale di gestione dei vault. Il test copre il ciclo AppImage. Montaggio dei vault, cifratura e portachiavi di sistema restano esclusi. Joplin ha mostrato l'editor e la nota iniziale. Il test dal menu è passato al secondo avvio con un file hosts dedicato a localhost e rete esterna isolata. Sincronizzazione, servizi cloud e accesso con account restano esclusi. [Cryptomator 1.19.6](https://github.com/cryptomator/cryptomator/releases/tag/1.19.6) · [Cryptomator 1.19.7](https://github.com/cryptomator/cryptomator/releases/tag/1.19.7) · [Joplin 3.7.22](https://github.com/laurent22/joplin/releases/tag/v3.7.22) · [Joplin 3.7.24](https://github.com/laurent22/joplin/releases/tag/v3.7.24)

KeePassXC 2.7.12 e 2.8.0 sono stati verificati come pacchetti upstream. Il launcher ha rifiutato l'ammissione AppImage con le regole Flatpak attive. [Log](full-interface/keepassxc/cycle.log) · [KeePassXC 2.8.0](https://github.com/keepassxreboot/keepassxc/releases/tag/2.8.0)

[Ricevute](packages.json) · [Suite core del banco di prova](core-tests.log) · [Ciclo nativo AppImage](../pro-371-linux-appimage-fourth/README.md)
