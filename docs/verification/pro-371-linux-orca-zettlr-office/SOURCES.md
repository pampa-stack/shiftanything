# Fonti AppImage e riproduzione

- Zettlr è distribuito con licenza GPL-3.0. L’icona da 128×128 arriva da `usr/share/icons/hicolor/128x128/apps/Zettlr.png` nella 4.11.0. [Repository](https://github.com/Zettlr/Zettlr) · [Download ufficiali](https://www.zettlr.com/download) · [Release 4.10.0](https://github.com/Zettlr/Zettlr/releases/tag/v4.10.0) · [Release 4.11.0](https://github.com/Zettlr/Zettlr/releases/tag/v4.11.0)
- ONLYOFFICE Desktop Editors è una suite locale gratuita con licenza AGPL-3.0. L’icona da 128×128 arriva da `usr/share/icons/hicolor/128x128/apps/onlyoffice-desktopeditors.png` nella 10.0.0. Il nucleo locale funziona offline. Cloud e modelli AI sono funzioni opzionali fuori dal ciclo testato. [Repository](https://github.com/ONLYOFFICE/DesktopEditors) · [Edizione desktop](https://www.onlyoffice.com/desktop.aspx) · [Release 9.4.0](https://github.com/ONLYOFFICE/DesktopEditors/releases/tag/v9.4.0) · [Release 10.0.0](https://github.com/ONLYOFFICE/DesktopEditors/releases/tag/v10.0.0)

| Release | Byte | SHA-256 pubblicato |
| --- | ---: | --- |
| Zettlr 4.10.0 | 172.628.308 | `bf6f7458c2377818d5d4ed008a721c295bec2c25d570cf8f54d053f8819508fc` |
| Zettlr 4.11.0 | 172.657.896 | `e9b5406d62e2c9e5fef3adb092877669d849d16a217375c4e1736cf9c7aa9bd7` |
| ONLYOFFICE 9.4.0 | 625.961.464 | `f5faf24552665262fe94486a510b997d2e5a24bde14df94c802c79bf17f9254c` |
| ONLYOFFICE 10.0.0 | 425.810.424 | `3e4668d44f6881d1d8873b38ea9c9fd81b9c77132facdb735bfebfe960edb745` |

I file AppImage corrispondono ai digest upstream Github REST e i quattro file contengono ELF AMD64 AppImage Type 2. Per riprodurre, percorso VPS2 `/mnt/dati/lavori/shift-vps2-appimage.RTanHwuk/lot3`, script `run-final.sh`, binario `source/crates/native/src/bin/verify_appimages_vps2_lot3.rs`. L’harness esegue NativeBackend su un HOME/XDG scratch, senza root, con X11/DBus isolati e senza rete durante gli avvii. HTTPS rimane disponibile per l’installazione delle release.
