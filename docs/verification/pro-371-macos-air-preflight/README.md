# Preflight DMG su Air, 27 novembre 2026

MacBook Air arm64 con macOS 26.6.2, 27 GiB liberi e 43% memoria libera alla partenza. Quattro prove sequenziali in directory private su Air, con montaggio in sola lettura e scollegamento a fine controllo. I log riportano orari UTC, fonti, dimensioni, digest, metadati e risultati dei controlli.

| App | Versione | Byte | Identità del bundle | Apple Developer ID | Architettura | Log |
|---|---:|---:|---|---|---|---|
| LuLu | 4.5.7 | 6.684.272 | `com.objective-see.lulu.app` | Objective-See, LLC `VBG97UB4TA` | `x86_64 arm64` | [Prova](lulu-4.5.7.log) |
| LuLu | 4.6.0 | 6.737.625 | `com.objective-see.lulu.app` | Objective-See, LLC `VBG97UB4TA` | `x86_64 arm64` | [Prova](lulu-4.6.0.log) |
| LocalAI | 4.12.0 | 20.473.080 | `com.localai.launcher` | LocalAI S.R.L. `34F3GGU9A6` | `arm64` | [Prova](localai-4.12.0.log) |
| LocalAI | 4.12.1 | 20.568.993 | `com.localai.launcher` | LocalAI S.R.L. `34F3GGU9A6` | `arm64` | [Prova](localai-4.12.1.log) |

Quattro SHA-256 ricontrollati dai metadati ufficiali GitHub. `CFBundleShortVersionString`, identificativo di firma e proprietario coerenti. Il numero di build interno LocalAI è `1` in entrambe le release. `codesign --verify --deep --strict`, ticket notarili per immagine e app, e Gatekeeper `Notarized Developer ID` positivi su tutte e quattro. I pacchetti sono stati smontati e rimossi dalla directory temporanea. La prova ha riguardato solo gli asset, senza avviare app, installare server o abilitare estensioni.

- LuLu richiede l’approvazione manuale di estensione e filtro di rete per una prova funzionale. Un ciclo successivo richiede una VM o un Mac di prova dedicato. [Guida ufficiale](https://objective-see.org/products/lulu.html) · [4.5.7](https://github.com/objective-see/LuLu/releases/tag/v4.5.7) · [4.6.0](https://github.com/objective-see/LuLu/releases/tag/v4.6.0)
- LocalAI 4.12 usa il launcher Go precedente. Il DMG desktop v5 è circa 121 MB, oltre il limite del preflight. Una futura ricetta per il prodotto corrente richiede controlli distinti su v5 e sul setup del server. [4.12.0](https://github.com/mudler/LocalAI/releases/tag/v4.12.0) · [4.12.1](https://github.com/mudler/LocalAI/releases/tag/v4.12.1) · [5.0.0](https://github.com/mudler/LocalAI/releases/tag/v5.0.0)
- OpenShot escluso per DMG Intel con requisito Rosetta. ImageOptim escluso perché l’app si distribuisce in `.tbz2`. [OpenShot](https://www.openshot.org/static/files/user-guide/installation.html) · [ImageOptim](https://imageoptim.com/howto.html)
