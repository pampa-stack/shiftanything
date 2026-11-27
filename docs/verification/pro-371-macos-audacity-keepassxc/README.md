# Audacity 4 e KeePassXC su Apple Silicon

Il 27 novembre 2026 il backend nativo di Sh*ftAnything ha scaricato i DMG ufficiali, verificato i digest, installato la versione precedente, aggiornato a quella successiva e spostato il bundle nel cestino di prova. Firma, notarizzazione, versione e marker sintetico del profilo sono stati verificati a ogni fase. Tutti i profili di prova sono stati ripuliti.

| App | Dal pacchetto → al pacchetto | Bundle | Developer ID | Target provato |
| --- | --- | --- | --- | --- |
| Audacity 4 | 4.0.1 → 4.0.2 | Audacity 4.app, org.audacityteam.audacity4 | MuseScore (6EPAF2X3PR) | arm64, macOS 13+ |
| KeePassXC | 2.7.12 → 2.8.0 (build 2.8.0-1) | KeePassXC.app, org.keepassxc.keepassxc | Janek Bevendorff (G2S7P7J672) | arm64, macOS 12+ |

| Pacchetto ufficiale | Byte | SHA-256 |
| --- | ---: | --- |
| [Audacity 4.0.1 arm64](https://github.com/audacity/audacity/releases/tag/Audacity-4.0.1) | 50.034.126 | 278c8647b78c77af7f07dbd5e7d9bfc950bc14168047b65738716b61d12055ec |
| [Audacity 4.0.2 arm64](https://github.com/audacity/audacity/releases/tag/Audacity-4.0.2) | 50.030.766 | fd462da6204cbf9758b44c7ce1c29d5fe5cab4f31ba308d33c125f1b170401c3 |
| [KeePassXC 2.7.12 arm64](https://github.com/keepassxreboot/keepassxc/releases/tag/2.7.12) | 34.360.675 | 65f4f63607180c0a15794b4a4068f85e99ed5391c87c1fb9312648f1b36fed40 |
| [KeePassXC 2.8.0-1 arm64](https://github.com/keepassxreboot/keepassxc/releases/tag/2.8.0) | 47.786.914 | 85f0e75e258ef513f0fc7f5bf470cad0943b7d7e8191b2171f9898ad32e62b9c |

Per Audacity 4 la ricetta usa la propria directory e il proprio bundle ID. È stata affiancata una fixture Audacity 3 con pathname e identità reali della serie 3. Il probe ha verificato che il relativo plist restasse intatto durante installazione, aggiornamento e rimozione della serie 4. Il framework QtCore di 4.0.1 richiede macOS 13 anche se il plist dichiara 10.15. In 4.0.2 il plist e il Mach-O dichiarano entrambi 13. [Issue upstream sulla compatibilità](https://github.com/audacity/audacity/issues/12535)

Per KeePassXC il sito ufficiale indica come corrente la build 2.8.0-1 con digest separato. Il digest coincide con quello dell’API di GitHub. La ricetta respinge il primo DMG 2.8.0, il 2.7.11 con problemi segnalati e gli asset Intel. [Download KeePassXC](https://keepassxc.org/download/#mac)

Il ciclo inietta nel profilo temporaneo i metadati delle due release per poter scegliere anche la precedente. L’installer, gli scaricamenti e il cestino sono quelli del prodotto. Entrambe le release correnti sono state risolte anche dal servizio reale di aggiornamento usando GitHub Pages dopo il limite dell’API anonima. La lista live delle versioni storiche è stata bloccata dalla quota anonima. Non sono state aperte GUI.

Log nella stessa directory: audacity-native.log, keepassxc-native.log, audacity-keepassxc-live-latest.log, audacity-live-versions.log, keepassxc-live-versions.log, audacity-architecture.log e keepassxc-architecture.log.
