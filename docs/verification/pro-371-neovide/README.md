# Neovide su macOS Apple Silicon

27 novembre 2026, prova macOS 26.4 (25E246), arm64. Il banco usa le primitive di download, installazione DMG, aggiornamento e rimozione del launcher già compilate (rlib native `267db41058133cc6`, core `181f0302a9698366`). L'input della ricetta è locale al banco. Worktree osservato a `ff517dbb2d0d29189e773c567e1597e7891dde68`. Il backend verificato richiede SHA-256, identità Developer ID, firma, notarizzazione, valutazione Gatekeeper e corrispondenza della versione nel bundle.

## Pacchetti

| Tag stabile | Asset | Byte | SHA-256 dal publisher |
| --- | --- | ---: | --- |
| 0.17.3 | Neovide-aarch64-apple-darwin.dmg | 12.337.549 | `024f11f8e5a69b58cd9f9c01cf5d6bbe5c4813474f0c5cc1150cddb70e7cd2cc` |
| 0.17.4 (latest attestato da REST) | Neovide-aarch64-apple-darwin.dmg | 12.338.261 | `5a4647cce0703552906552ae1d0d955e221f3153af832fa6739ffc0592774e2a` |

Identità `com.neovide.neovide`, Team `X8CNW77992`, Developer ID Application ALEXSANDE FALCUCCI DOS SANTOS, binario `neovide` arm64, minimo da plist macOS 12.0. Entrambi i DMG montati in sola lettura, digest verificato, codesign deep/strict e Gatekeeper Notarized Developer ID accettati. Ticket stapled nel DMG, non nella app montata. [Release 0.17.3](https://github.com/neovide/neovide/releases/tag/0.17.3) · [Release 0.17.4](https://github.com/neovide/neovide/releases/tag/0.17.4) · [Versioni ufficiali](neovide-official-release-data.json).

## Ciclo

- Installazione 0.17.3 PASS in 63,531 s, 12.337.549 byte ricevuti dal motore. Ricevuta di prova e versione del bundle verificate.
- Avvio diretto CLI `--version` di 0.17.3 e 0.17.4 PASS in 3,51 s e 0,38 s. L'eseguibile gira senza rete o lookup Mach, lettura del profilo personale e scrittura esterna al profilo di prova. Canario di scrittura esterna negato. [Riferimento CLI](https://neovide.dev/command-line-reference.html#version).
- Aggiornamento 0.17.3 → 0.17.4 PASS in 4,756 s, 12.338.261 byte ricevuti. Bundle sostituito al medesimo percorso, versione e ricevuta rilette.
- Rimozione PASS in 0,024 s. Bundle nel Cestino del profilo di prova, ricevuta finale vuota, marker dati sintetici preservato.

La prova CLI non usa il pulsante Apri, LaunchServices o finestre macOS. Per verificare Apri dalla GUI serve un account o una VM separata con Neovim ≥0.12 installato. Per usare l'app è necessario che Neovim sia nel PATH o impostato con `neovim-bin`. [Installazione ufficiale](https://neovide.dev/installation.html#binaries). Lo stesso nome asset viene riusato a ogni tag, e il selettore generico non lo riconosce. Per esporre Installa nel prodotto occorre una ricetta arm64 esplicita che confronti nome esatto, repository, tag, URL, dimensione e digest. La ricetta integrata attiva solo macOS Apple Silicon. I test core verificano selezione, tag e vincoli; il test nativo ricontrolla tag, URL, digest e dimensione prima del download.

[Log ciclo](neovide-cycle.log) · [Preflight 0.17.3](neovide-0.17.3-preflight.log) · [Preflight 0.17.4](neovide-0.17.4-preflight.log) · [Banco](cycle.rs) · [Procedura](run-neovide.sh) · [Link rlib](link.log).

## Patch per il launcher

La patch isolata da `ff517dbb2d0d29189e773c567e1597e7891dde68` abilita solo macOS Apple Silicon dal sistema 12. Risolve il DMG dal tag stabile ufficiale senza prefisso, richiede nome e URL esatti e un digest SHA-256, e blocca digest e dimensione diversi per le due versioni verificate. Il controllo si ripete prima del download, insieme ai controlli notarizzazione e identità già presenti.

Applicabilità e formattazione PASS. Suite Core 130 PASS, 1 ignorato già presente. Test Native mirati PASS per versione minima e per confine pre-download, inclusi tag/host/digest/dimensione scorretti. Le prove su DMG ufficiali sopra usano le primitive native con ricetta di prova. La patch è pronta per l'integrazione in prodotto.

[Patch Rust](neovide-native.patch) · [Nota bilingue per la scheda](neovide-description.patch) · [Suite Core](neovide-core-full.log) · [Confine Native](neovide-native-test.log) · [Minimo macOS](neovide-native-minimum.log).

La scheda Yaak è stata rimossa dal catalogo. Il sorgente MIT permette l’uso commerciale, mentre i pacchetti precompilati del publisher richiedono una licenza per l’uso al lavoro. [Licenze Yaak](https://yaak.app/pricing#frequent-questions).
