# Shotcut e Pinta su macOS Apple Silicon

Mac arm64 con macOS 26.4 (25E246), policy Gatekeeper attiva. DMG scaricati dai repository GitHub ufficiali con SHA-256 pubblico e dimensione esatta, montati in sola lettura e verificati con codesign deep/strict su tutte le architetture, notarizzazione e Gatekeeper. Quattro preflight verdi.

| App | Versione | DMG | Byte | SHA-256 |
| --- | --- | --- | ---: | --- |
| Shotcut | 26.9.27 | [Release stabile](https://github.com/mltframework/shotcut/releases/tag/v26.9.27) | 210131979 | `629b056bfee1670c1669dd73f19f76fe48e0cdf90e184dc1aa1bdd6d09aa2548` |
| Shotcut | 26.11.15 | [Release stabile](https://github.com/mltframework/shotcut/releases/tag/v26.11.15) | 211234714 | `664c6ea4195d58dbf3fd43f3d74ae46d59b6310f9ac628d5e70a8e2786a4f80b` |
| Pinta | 3.1.1 | [Release stabile](https://github.com/PintaProject/Pinta/releases/tag/3.1.1) | 67815026 | `84912d381d835a912a11c092e74fef7ad5c13e8563d94dedb1f5e9ad7e9b2b3a` |
| Pinta | 3.1.2 | [Release stabile](https://github.com/PintaProject/Pinta/releases/tag/3.1.2) | 68171678 | `daad4b3550e7d5f27928ccb9ca99392aa46a5ef71cfd54ce72c0b7a810f4f993` |

Shotcut firma Meltytech, LLC (Y6RX44QG2G), bundle `com.Meltytech.Shotcut`, arm64/x86_64 con minimo 12.0 in Mach-O e plist. Ricetta limitata ad Apple Silicon. Il selettore esclude il DMG unsigned e gli asset con versione diversa dal tag. [Requisiti ufficiali](https://www.shotcut.org/FAQ/)

Pinta firma Cameron White (D5G6C56TBH), bundle `com.ximian.pinta`, arm64. Entrambi gli eseguibili principali dichiarano minos 11.0 nel Mach-O, senza dichiarazione `LSMinimumSystemVersion`. Verifica runtime su macOS 26.4 limitata all'installazione e alla gestione del bundle. Il nome costante `Pinta-macos-arm64.dmg` è vincolato a repo, tag, URL, digest, dimensione e versione firmata all'interno. [Download del progetto](https://www.pinta-project.com/releases/)

Il primo ciclo Pinta ha riprodotto il rifiuto della copia di staging dopo che `ditto --noextattr` aveva eliminato la firma di `Mono.Cecil.Rocks.dll`. La copia Pinta preserva le xattr e continua a escludere le ACL, conservando le cinque `com.apple.cs.*`. La stessa verifica nativa di firma, notarizzazione e Gatekeeper corre prima di ogni sostituzione. Il ciclo ripetuto ha verificato installazione 3.1.1, aggiornamento 3.1.2, rimozione nel Cestino temporaneo, registro ripulito, versioni corrette e file dati sintetico invariato. Log negativo `pinta-stripped-signature-negative.log`, positivo `pinta-native-cycle.log`.

I cicli usano il `NativeBackend` collegato alle normali librerie di prodotto con un profilo temporaneo. Le fasi consumano HTTPS e metadati GitHub reali e invocano `ListVersions`, installazione scelta, `CheckForUpdates`, aggiornamento e rimozione. Ogni app, bundle e Cestino restano nella propria root di prova. L'avvio GUI, macOS Intel e release future richiedono evidenze separate.

Il ciclo Shotcut ha verificato installazione 26.9.27, aggiornamento 26.11.15, rimozione nel Cestino temporaneo e pulizia del registro. Al termine di ogni fase la firma Developer ID e il file dati sintetico erano intatti. Log `shotcut-native-cycle.log`. Il primo test di installazione nativa era già riuscito, ma lo script di prova si era fermato per l'ordine degli argomenti di `lipo`. Il ciclo completo corretto è terminato con codice 0.

Controlli sorgente: 128 test core passati, 1 ignorato preesistente. Test mirato compatibilità macOS e Clippy core/native su tutti i target passati.
