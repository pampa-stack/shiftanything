# Sh*ftAnything 0.20.3 beta

Build dal commit `32fd18929fe3c307321ebcb4edcb0d511a79a63f` del 27 novembre 2026. Il catalogo contiene 531 schede, con 55 installazioni verificate per macOS Apple Silicon e 141 per Linux x64.

| Pacchetto | Dimensione in byte | SHA-256 |
| --- | ---: | --- |
| `ShiftAnything-0.20.3-macos-arm64.dmg` | 41.218.283 | `f2965cc1924574c1781521ed01612f4dfc936951927bef30d8fef3063d7e91ca` |
| `ShiftAnything-0.20.3-linux-amd64.deb` | 37.544.028 | `679ed18bd7c4e4dfe5ef52e44ef590392548c48fe87d5b5266a9348ad947c039` |

## macOS Apple Silicon

Apple ha accettato sia l'app, inviata alle 16:06 CET, sia il DMG, inviato alle 16:54 CET. Il DMG finale è stato copiato nella cartella Download del MacBook Air. Sul MacBook Air sono passati `stapler validate`, Gatekeeper, firma dell'app, verifica dell'icona e avvio del binario con `--version` dalla copia isolata. La prova manuale da Finder e l'installazione di un'app dal launcher restano da fare sull'Air prima della pubblicazione.

## Linux x64

Su Ubuntu 24.04.5 in VM su AX42, APT ha installato il deb. Il launcher è stato aperto dal menu desktop. Dal launcher sono stati provati rifiuto e conferma dell'installazione di Lapce 0.4.6, apertura dell'editor e rimozione mantenendo il profilo di prova. APT ha rimosso il launcher. [Prova desktop Linux](../pro-371-ax42-linux/README.md)

I due pacchetti sono candidati beta locali.
