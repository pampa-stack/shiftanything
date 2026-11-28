# Parabolic su Ubuntu 24.04 x64

Il 28 novembre 2026, in un guest AX42 temporaneo, `NativeBackend` ha installato Parabolic 2026.5.0 dal canale stabile Flathub. Il pacchetto ha esportato la voce del menu e l’icona. La prova ha preparato la revisione 2026.4.1 dal registro firmato per riprodurre un’installazione precedente. Il backend ha rifiutato la modifica di una revisione cambiata esternamente prima che la configurazione di prova registrasse la versione precedente. Da lì ha aperto l’app, aggiornato a 2026.5.0, aperto la nuova versione e rimosso il pacchetto preservando il file sintetico nel profilo. Entrambe le revisioni portavano la firma Flathub `4184DD4D907A7CAE`.

Le finestre mostrano l’avviso iniziale in entrambe le versioni. La rete era isolata e la funzione di scaricare contenuti non è stata provata. I 297,2 MB indicati nella scheda si riferiscono al solo pacchetto. Su Apple Silicon il controllo si è fermato prima dell’apertura perché `codesign` non ha rilevato una firma nel pacchetto upstream provato.

[Flathub](https://flathub.org/apps/org.nickvision.tubeconverter) · [Prima](previous.png) · [Dopo](current.png) · [Log](lifecycle.log) · [Firma precedente](previous.signature.txt) · [Firma corrente](current.signature.txt) · [Sandbox](sandbox.txt)
