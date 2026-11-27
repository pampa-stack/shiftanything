# Ente Auth su macOS Apple Silicon

Verifica del 27 novembre 2026 sul codice 7007986 con le patch scratch `stats-whispering-native.patch` ed `ente-auth-native.patch` in quest'ordine.

| Versione | DMG ufficiale | Byte | SHA-256 |
|---|---|---:|---|
| 4.4.26 | [ente-auth-v4.4.26.dmg](https://github.com/ente/ente/releases/tag/auth-v4.4.26) | 46.193.389 | `f063d87dc247feed7dba0ecdae208ffc22f9b5b2de0c58506394a2cae0610ada` |
| 4.4.27 | [ente-auth-v4.4.27.dmg](https://github.com/ente/ente/releases/tag/auth-v4.4.27) | 58.891.715 | `9c05d588b612d697f50d5b54cce1677cfac3c38a9aa32910279f68cd81f6c718` |

Entrambi i pacchetti sono universal e riportano macOS minimo 12.0 nel plist e sul binario ARM64. Il bundle è `io.ente.auth.mac`, il Team ID è `6Z68YJY9Q2`, Developer ID Application Ente Technologies, Inc. Identità, versione, SHA-256, architettura, notarizzazione e Gatekeeper verificati dai DMG montati in sola lettura. La ricetta è abilitata solo per Apple Silicon.

Il repository Ente condivide i rilasci di Photos, CLI e Auth. Il launcher legge le release stabili `auth-v` e richiede il nome esatto del DMG, il repo, la coincidenza della versione, dimensione positiva e SHA-256. L'endpoint pubblico del produttore indica il DMG corrente quando l'API non risponde. [Pagina Auth](https://ente.com/auth/), [download desktop](https://ente.com/download/auth/dmg), [documentazione](https://ente.com/help/auth/).

NativeBackend ha installato 4.4.26 e aggiornato a 4.4.27 con due download HTTPS effettivi, poi ha rimosso il bundle dal manifest e l'ha spostato nel Cestino del profilo. Il marker dei dati è rimasto identico. La lista storica è stata popolata con le risposte GitHub ufficiali conservate nel T7 perché la quota anonima era esaurita. Il controllo versione corrente è rimasto online tramite l'endpoint del produttore. L'installazione ha usato una directory Applications temporanea, senza prima esecuzione GUI o accesso al portachiavi personale.

- Prove DMG in `ente-auth-v4.4.26-package.log` e `ente-auth-v4.4.27-package.log`.
- Flusso, download, ricevute, firma dei bundle e marker in [lifecycle-seeded.log](lifecycle-seeded.log).
- Selezione della release corrente a cache vuota verificata via redirect del produttore, con quota GitHub esaurita. [Log](current-channel.log)
