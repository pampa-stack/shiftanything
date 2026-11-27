# Super Productivity su macOS Apple Silicon

Il 27 novembre 2026 su MacBook Air con macOS 26.6.2, il motore nativo in un profilo temporaneo ha installato la versione ufficiale `20.0.0` (148.678.260 byte), aggiornato a `20.1.0` (148.973.846 byte) e rimosso il bundle nel Cestino del profilo. I trasferimenti hanno usato GitHub Releases e i digest ufficiali riportati nel log. Firma Developer ID, Team ID `363FAFK383`, bundle `com.super-productivity.app`, architettura `arm64`, Gatekeeper e requisito macOS 13 sono stati controllati sui due pacchetti.

Per riproporre la 20.0.0 e lavorare con la quota GitHub REST esaurita, il test ha inizializzato solo la cache del proprio profilo con i metadati ufficiali. Il controllo di versione nuova è passato dal fallback pubblico. Il motore ha registrato entrambe le versioni e ha rimosso la ricevuta dopo la disinstallazione. Un dato sintetico collocato fuori dal bundle è rimasto presente. La prova ha lasciato l'app chiusa e ha esercitato il motore direttamente.

[Log](cycle.log) · [Ricevuta installazione](sp-install-receipt.json) · [Ricevuta aggiornamento](sp-update-receipt.json) · [Ricevute dopo la rimozione](sp-remove-receipt.json) · [Release 20.0.0](https://github.com/super-productivity/super-productivity/releases/tag/v20.0.0) · [Release 20.1.0](https://github.com/super-productivity/super-productivity/releases/tag/v20.1.0)
