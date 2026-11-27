# Termix su macOS Apple Silicon

Prova del 27 novembre 2026 in un profilo temporaneo su disco esterno, con Applications, cache e cestino separati. La ricetta usa il DMG desktop ARM64 standard, richiede macOS 12 o successivo e verifica il bundle `com.karmaa.termix` del team Developer ID `2CPC5YZYFN`. [Codice e licenza Apache-2.0](https://github.com/Termix-SSH/Termix/blob/v26.11.3/LICENSE)

| Versione | Byte | SHA-256 |
| --- | ---: | --- |
| [26.11.2](https://github.com/Termix-SSH/Termix/releases/tag/v26.11.2) | 254640116 | `c9b8eda86bf88f85ab2f2aa295e76fe1409805f65b3ab3c69a0cd7186c7cac5e` |
| [26.11.3](https://github.com/Termix-SSH/Termix/releases/tag/v26.11.3) | 263900889 | `a509ba2eebd74b7562f38d5adca342c137e431fab1744af34bdbd07eb9a96f3c` |

I due DMG montati in sola lettura espongono la versione prevista e un binario ARM64. Entrambi i bundle hanno passato la firma profonda, la verifica di notarizzazione e Gatekeeper. La ricetta esclude i tag SDK, le prerelease e le immagini per altre architetture.

Il primo tentativo con il motore scaricava correttamente 26.11.2 ma macOS rifiutava il punto di mount sul disco esterno. Lo stesso file montava con la stessa utenza sotto `/private/tmp`. Il motore e lo script del DMG locale usano ora una directory temporanea privata sul volume di sistema, mentre cache e applicazioni possono restare sul disco esterno.

Dopo la prima correzione, Gatekeeper ha superato il limite di 60 secondi sul bundle copiato e il motore ha lasciato intatta la destinazione. Termix ha ora 180 secondi per la sola valutazione di Gatekeeper, con gli stessi controlli di identità, firma, notarizzazione e versione.

La riprova ha riutilizzato il download originale di 26.11.2 dopo averne verificato il digest. L'elenco storico era salvato dalla risposta ufficiale GitHub; la versione di aggiornamento è stata rilevata dal motore con un controllo online. Installazione, aggiornamento e rimozione hanno passato anche un controllo indipendente di firma, notarizzazione e architettura sul bundle risultante. La rimozione ha cancellato la ricevuta e spostato l'app nel cestino temporaneo, conservando il file sintetico del profilo. Nel bundle è presente l'icona macOS `icon.icns`.

```text
PASS NATIVE termix install: Termix installed: 26.11.2; transferred=254640116
PASS SIGNATURE + PROFILE MARKER termix install 26.11.2
PASS NATIVE termix update: Termix updated to 26.11.3; transferred=263900889
PASS SIGNATURE + PROFILE MARKER termix update 26.11.3
PASS NATIVE termix remove: Termix was removed; transferred=0
PASS SIGNATURE + PROFILE MARKER termix remove 26.11.3
```

Il nuovo DMG del launcher ha passato verifica checksum, mount in sola lettura, controllo licenze, firma ad hoc del bundle ed esecuzione di `--version`. SHA-256 `ed7ae534bf250d5775afeb7db591e7317f36d94d6c68eea19cc45031c2b6860c`. Il lancio di Termix, il suo Dock e la gestione di Termix su Linux, Windows e Mac Intel rimangono da provare.

Nel bundle estratto dal DMG, la finestra Tauri ha mostrato il logo, l’anteprima, la versione 26.11.3, i 263,9 MB e il pulsante Installa. Il clic ha aperto la conferma con l’opzione «Apri dopo l’installazione e mostra nel Dock». Rifiuta ha riportato alla scheda, con il profilo Tauri ancora privo di Termix. Il ciclo di installazione qui sopra usa il motore nativo in un secondo profilo. [Scheda](termix-detail-it.png) · [Conferma](termix-consent-it.png)

Suite finale: 130 test core, 558 native e 12 Tauri passati. Otto test preesistenti rimangono ignorati. Formattazione, Clippy e controlli dello script del DMG passati.
