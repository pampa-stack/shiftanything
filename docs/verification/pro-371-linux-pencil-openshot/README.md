# Prova AppImage su VPS2, 27 novembre 2026

**Esito** — Pencil2D 0.7.1 → 0.7.2 e OpenShot 4.1.0 → 4.1.1 hanno completato due cicli ciascuno con il NativeBackend di Sh*ftAnything. Installazione HTTPS senza root, apertura della finestra prima e dopo l’aggiornamento, avvio dall’entry desktop generata, rimozione e conservazione dei dati campione. Nessuna installazione nel profilo reale.

| App | Installazione | Finestra iniziale | Aggiornamento | Finestra aggiornata | Menu | Rimozione |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Pencil2D | 2,105 s | 44,661 s | 7,181 s | 59,423 s | 21,961 s | 42 ms |
| OpenShot | 13,559 s | 11,916 s | 9,513 s | 10,610 s | 9,586 s | 121 ms |

Tempi del secondo ciclo, su VPS2 Ubuntu 24.04.5 x86-64, con carico condiviso e senza accelerazione GPU. Al primo ciclo l’apertura aggiornata aveva richiesto 4,877 s e 8,836 s rispettivamente. I PNG finali del menu sono stati aperti e controllati: canvas e timeline in Pencil2D, editor e tutorial iniziale in OpenShot.

## Provenienza e isolamento

- Pencil2D, [download ufficiale](https://www.pencil2d.org/download/) e release GitHub [0.7.1](https://github.com/pencil2d/pencil/releases/tag/v0.7.1), [0.7.2](https://github.com/pencil2d/pencil/releases/tag/v0.7.2). AppImage AMD64 da 60.471.800 byte per versione. SHA-256 pubblicati dal registro release verificati su file scaricati. Icona estratta da `usr/share/icons/hicolor/256x256/apps/org.pencil2d.Pencil2D.png` e adattata a 128×128.
- OpenShot, [download ufficiale](https://www.openshot.org/download/) e release GitHub [4.1.0](https://github.com/OpenShot/openshot-qt/releases/tag/v4.1.0), [4.1.1](https://github.com/OpenShot/openshot-qt/releases/tag/v4.1.1). AppImage da 258.708.672 e 258.921.664 byte. SHA-256 verificati sia rispetto alla release API sia ai file di checksum pubblicati. Icona estratta da `usr/share/icons/hicolor/128x128/apps/openshot-qt.png`. File `StartupWMClass=openshot` verificato nella finestra reale.
- Il profilo isolato persiste per l’intero ciclo. I processi delle app sono confinati con Bubblewrap, rete scollegata, senza FUSE, display Xvfb 1440×900 e bus DBus privati. L’installazione usa HTTPS tramite il backend, controlla dimensione e SHA-256 e scrive il manifesto e l’integrazione Linux. L’avvio dal menu usa `gio launch`.
- La prova copre il backend e le entry Linux. Il banco non attraversa la finestra di Sh*ftAnything, non verifica firme dei binari upstream e non svolge test delle funzioni di editing. Le icone sono gli asset inclusi nei binari upstream e il test prova l’installazione delle icone native. Nessuna licenza separata di marchio verificata.
- Il banco crea `Applications` nel profilo temporaneo. Il setup della sessione Tauri la crea già nel profilo app, quindi la patch non cambia startup.

## File

- Le ricette x86-64 fissano forma del nome, URL upstream della release stabile, dimensione e SHA-256.
- `final/<id>/cycle.log`, `{open-old,open-new,open-menu}.png`, `installed-icon.png`, entry `.desktop` e SHA delle prove.
- Sorgente e riproduzione su VPS2: `/mnt/dati/lavori/shift-vps2-appimage.RTanHwuk`. File `run-final.sh` e [banco Rust](verify_appimages_vps2.rs).
