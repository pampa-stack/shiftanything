# Cinque nuove app Avanzate

Catalogo del 27 novembre 2026. Totale 533 schede, con 45 nella sicurezza difensiva, 18 in VPN e accesso remoto e 21 in rete. Ogni nuova scheda usa il repository canonico e una release stabile. Le stelle sono quelle lette da GitHub il 27 novembre.

| App | Stelle | Licenza | Versione e piattaforme documentate | Interfaccia |
| --- | ---: | --- | --- | --- |
| Sniffnet | [43.339](https://api.github.com/repos/GyulyVGC/sniffnet) | [MIT](https://github.com/GyulyVGC/sniffnet/blob/v1.6.0/LICENSE-MIT) o [Apache-2.0](https://github.com/GyulyVGC/sniffnet/blob/v1.6.0/LICENSE-APACHE) | [v1.6.0, macOS/Linux/Windows](https://github.com/GyulyVGC/sniffnet/releases/tag/v1.6.0) | [Monitoraggio desktop](https://sniffnet.app/) |
| Termix | [16.133](https://api.github.com/repos/Termix-SSH/Termix) | [Apache-2.0](https://github.com/Termix-SSH/Termix/blob/v26.11.3/LICENSE) | [v26.11.3, macOS/Linux/Windows](https://github.com/Termix-SSH/Termix/releases/tag/v26.11.3) | [Client SSH con interfaccia desktop e modalità self-hosted, senza piani a pagamento](https://github.com/Termix-SSH/Termix/tree/v26.11.3#readme) |
| simplewall | [9.378](https://api.github.com/repos/henrypp/simplewall) | [GPL-3.0](https://github.com/henrypp/simplewall/blob/v.3.8.7/LICENSE) | [v.3.8.7, Windows](https://github.com/henrypp/simplewall/releases/tag/v.3.8.7) | [Regole firewall per applicazione](https://github.com/henrypp/simplewall/tree/v.3.8.7#readme) |
| Karing | [16.377](https://api.github.com/repos/KaringX/karing) | [GPL-3.0-or-later con riserva sul nome nei derivati](https://github.com/KaringX/karing/blob/v1.2.26.2934/LICENSE.md) | [v1.2.26.2934, macOS/Linux/Windows](https://github.com/KaringX/karing/releases/tag/v1.2.26.2934) | [Client grafico per profili proxy propri](https://github.com/KaringX/karing/tree/v1.2.26.2934#readme) |
| Zoraxy | [5.598](https://api.github.com/repos/tobychui/zoraxy) | [AGPL-3.0](https://github.com/tobychui/zoraxy/blob/v3.3.5/LICENSE) | [v3.3.5, macOS/Linux/Windows](https://github.com/tobychui/zoraxy/releases/tag/v3.3.5) | [Pannello self-hosted per reverse proxy HTTPS](https://zoraxy.aroz.org/) |

## Esiti e limiti

- Verificati e letti 5 loghi e 9 screenshot originali. Il backend ha caricato 13 slot via HTTPS e li ha restituiti di nuovo dalla cache senza HTTP. Il logo simplewall usa il frame ICO ufficiale, convertito in PNG e incluso nel frontend con fonte, hash e licenza. I loghi Sniffnet e Termix sono inclusi nel frontend. Karing e Zoraxy usano il caricamento nativo.
- I metadati delle release e i collegamenti agli asset ufficiali descrivono i pacchetti del progetto. Per queste cinque schede resta da abilitare e verificare il ciclo installazione, aggiornamento e rimozione.
- Sniffnet macOS ARM64, v1.5.0 e v1.6.0. I due DMG corrispondono ai digest GitHub. Montati in sola lettura. Il bundle di entrambe le release ha restituito `code object is not signed at all` con la verifica macOS. Entrambi i dischi sono stati smontati senza eseguire l'app.

## Scartate

- TrafficMonitor ha una licenza Anti-996. [Licenza del progetto](https://github.com/zhongyang219/TrafficMonitor/blob/master/LICENSE)
- Defguard include un'edizione Enterprise a pagamento e codice riservato. Il client separato è sotto 1.000 stelle. [Piani](https://defguard.net/pricing/) · [Licenza](https://github.com/DefGuard/defguard/blob/v2.2.0/LICENSE.md) · [Client](https://github.com/DefGuard/client)

## Build e prove

- Test Rust, 129 passati in core e 12 in Tauri, con un test core ignorato già in precedenza. Trasporto MCP stdio passato su 533 schede e 11 profili SH*FTER, più verifica del bridge con un profilo temporaneo.
- Prova Chromium a 1440 × 900, in italiano e inglese, con bridge simulato e immagini ricevute dal backend nativo. Ricerca e apertura di tutte e cinque le schede, loghi e anteprime, versione e licenza. simplewall mostra il lucchetto su macOS. [Sniffnet IT](sniffnet-it.png) · [simplewall EN](simplewall-en.png)
- DMG Apple Silicon locale `ShiftAnything-0.20.3-macos-arm64-tauri-local.dmg`, SHA-256 `fb284c8fa19f43fce0bdb8c2eecbb036d5b4a8b51c850bf521eb96715be1e49c`. Verificati mount in sola lettura, firma ad hoc del bundle, architettura, versione e licenze. La verifica di questa build sul desktop macOS resta da fare.
