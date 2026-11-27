# Godot Engine su macOS Apple Silicon

Prova del 27 novembre 2026 con `NativeBackend` e una cartella Applications in un profilo temporaneo. Pacchetti del canale stabile, variante standard universale. Licenza MIT e team Developer ID `6K46PWY5DM`. [Licenza](https://godotengine.org/license/) · [Release 4.7.1](https://github.com/godotengine/godot-builds/releases/tag/4.7.1-stable) · [Release 4.7.2](https://github.com/godotengine/godot-builds/releases/tag/4.7.2-stable)

| Versione | Byte | SHA-256 |
| --- | ---: | --- |
| 4.7.1 | 166939858 | `897cb7f9799796c717ae75f31446aed883dc92b1d6c3b33d893cc7843fff2fa9` |
| 4.7.2 | 170622178 | `c58a24e31d720be9d62f60cb5627c4e695fb72f21b0cfe1bc9ccaa9a3b3ba63e` |

1. Il runner ha caricato l'elenco delle versioni dalla risposta ufficiale e il motore ha scaricato e installato 4.7.1. Firma, notarizzazione, architettura e ricezione del bundle `org.godotengine.godot` verificate.
2. Il primo aggiornamento ha scaricato 4.7.2. Gatekeeper ha superato i 60 secondi concessi e il motore ha conservato 4.7.1. La seconda prova ha riutilizzato lo ZIP dopo averne ricontrollato SHA-256. Con 180 secondi per la sola valutazione Gatekeeper di Godot, il motore ha verificato e installato 4.7.2.
3. Il motore ha rimosso 4.7.2 dal profilo e l'ha spostato nel cestino temporaneo. Il digest del file sintetico del profilo è rimasto uguale.

La prova usa pacchetti ufficiali e la verifica nativa. Restano da provare il lancio di Godot, i clic nella finestra Tauri e gli altri sistemi operativi.

## Esiti osservati

```text
PASS NATIVE godot-engine install: Godot Engine installed: 4.7.1; transferred=166939858
PASS SIGNATURE + PROFILE MARKER godot-engine install 4.7.1
Error: "macOS verification did not succeed (macOS policy: /usr/sbin/spctl didn't finish within 60 seconds), so the app wasn't installed"
PASS NATIVE godot-engine update: Godot Engine updated to 4.7.2; transferred=170622178
PASS SIGNATURE + PROFILE MARKER godot-engine update 4.7.2
PASS NATIVE godot-engine remove: Godot Engine was removed; transferred=0
PASS PRIVATE TRASH + PROFILE MARKER godot-engine remove
```
