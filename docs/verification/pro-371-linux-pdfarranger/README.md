# PDF Arranger su Linux x86_64

Prova del 27 novembre 2026. Runner compilato offline dal commit `78615f63b962ccfa59e59d6e42c4035b1280d51a` con l'aggiunta della ricetta. SHA256 runner `2514158dffce3e854484aa9bc762da629061c748068bd3863ffd0ee294b7170a` verificato dopo il trasferimento.

Il canale stabile corrisponde al ref `app/com.github.jeromerobert.pdfarranger/x86_64/stable`. La sua origine è verificata da Flathub. [Scheda Flathub](https://flathub.org/apps/com.github.jeromerobert.pdfarranger) · [Manifest](https://github.com/flathub/com.github.jeromerobert.pdfarranger) · [Sorgente PDF Arranger](https://github.com/pdfarranger/pdfarranger)

| Versione | Revisione OSTree | Prova |
|---|---|---|
| 1.13.0 | `0e6435e81fe910ff596d44ebe7d523a5105d7b9089108ec24b073b790d2a310f` | [Firma](previous.gpg.txt) · [Finestra](previous.png) |
| 1.14.0 | `09dad0e4d6c3ae002efe5299fb2a0af5eba9ffd2a4a0f17370e2b7049302290a` | [Firma](current.gpg.txt) · [Finestra](current.png) |

NativeBackend ha installato 1.14.0 in un profilo utente temporaneo senza installazioni di sistema. Il banco ha riportato l'app alla revisione precedente firmata con `flatpak update --commit`. Prima di impostare il record di proprietà della fixture, NativeBackend ha rilevato la sostituzione esterna e ha rifiutato aggiornamento e rimozione. Dopo il seed, NativeBackend ha aperto 1.13.0, aggiornato a 1.14.0, aperto 1.14.0 e rimosso l'app. I dati sintetici sono stati verificati per SHA256 dopo update, rimozione e pulizia dei runtime.

Le due finestre reali in Xvfb, con rete dell'app limitata al loopback, mostrano il titolo e la barra strumenti. In questa cattura la vista a documento vuoto presenta un rettangolo nero centrale. La prova copre l'avvio e il ciclo del motore. Non è stata esercitata la modifica di un PDF o la finestra Tauri.

- [Log del ciclo e della pulizia](lifecycle.log)
- [Test della matrice Linux x64](core-test.log)
- [Rete 1.13.0](previous.network.log) · [Rete 1.14.0](current.network.log)
- [Digest delle evidenze](SHA256SUMS)
