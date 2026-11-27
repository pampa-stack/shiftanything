# Flameshot su Linux x86_64

Prova del 27 novembre 2026. Runner compilato offline dal commit `1304d4722c7f1a55e26902c6fa385e729cc90cd3` più la ricetta Flameshot. SHA256 runner `3b31ea67dc261d6cbbdbd7617e47aa5bde3dafbf74acbbb0c01670e165aec90d`, verificato dopo il trasferimento.

Il canale stabile è `app/org.flameshot.Flameshot/x86_64/stable`. Flathub riporta `verified: true` tramite `flameshot.org`. [Metadati Flathub](publisher.json) · [Scheda Flathub](https://flathub.org/apps/org.flameshot.Flameshot) · [Installazione ufficiale](https://flameshot.org/docs/installation/installation-linux/) · [Manifest](https://github.com/flathub/org.flameshot.Flameshot)

| Versione | Revisione OSTree | Prova |
|---|---|---|
| 14.0.0 | `c5bcb9c0126aecba6d56b9b796925145707b721a0d82f704bc3fd3e3c731798f` | [Firma](previous.gpg.txt) · [Finestra](previous.png) |
| 15.0.0 | `09826023adac291134b5ecca84c0c449ac4d847249ba628dce5331dcae74ad26` | [Firma](current.gpg.txt) · [Finestra](current.png) |

NativeBackend ha installato 15.0.0 in un profilo utente temporaneo. Il banco ha selezionato 14.0.0 con `flatpak update --commit` e ha verificato la firma. Il motore ha rifiutato aggiornamento e rimozione mentre la revisione era gestita esternamente. Dopo il seed del record proprietario, NativeBackend ha avviato 14.0.0, aggiornato a 15.0.0, avviato di nuovo e rimosso l'app. Il digest del file sintetico è rimasto uguale dopo l'aggiornamento, la rimozione e la pulizia dei runtime.

L'avvio senza argomenti avvia il processo che gestisce la tray. Il banco ha rilevato l'istanza Flatpak prima di aprire separatamente le impostazioni con il comando documentato `flameshot config`. Le finestre sono state catturate soltanto sul display Xvfb sintetico, con bus privato, accessi al filesystem host rimossi e rete dell'app limitata al loopback. Il kernel ha rifiutato il primo contenitore esterno; la prova è ripresa dalla revisione firmata già installata nello stesso profilo usando la sandbox Flatpak. Questo banco verifica l'avvio e il ciclo del motore. I clic nella finestra Tauri e la cattura di un desktop reale restano da verificare. [Comandi e comportamento della tray](https://flameshot.org/docs/advanced/commandline-options/)

- [Log del ciclo e pulizia](lifecycle.log) · [Test matrice Linux x64](core-test.log)
- [Istanza 14.0.0](previous.instances.txt) · [Istanza 15.0.0](current.instances.txt)
- [Rete 14.0.0](previous.network.log) · [Rete 15.0.0](current.network.log)
- [Digest delle evidenze](SHA256SUMS)
