# OpenGame Studio su MacBook Air

Prova del 27 novembre 2026 su Apple Silicon in una directory temporanea, dal [commit ufficiale f17151f](https://github.com/nexu-io/opengame/tree/f17151f3cc7838ddbe13d9b94392e0f983fc341b). Il codice dello studio usa la [licenza Apache-2.0](https://github.com/nexu-io/opengame/blob/f17151f3cc7838ddbe13d9b94392e0f983fc341b/LICENSE).

- Compilazione e avvio da sorgente con Node.js 22.23.1. L'endpoint locale ha risposto con editor pronto e 274 template.
- [Dashboard aperta](studio-mac-air.png), ricerca di Tidebreaker, clic su Prova e gioco avviato. Lo [screenshot della gara](../../../apps/shiftanything-tauri-proof/frontend/media/opengame-shot-0.png) mostra 106 nodi e 64 secondi residui. L'accesso del browser era limitato ai due servizi locali.
- Nessun account, chiave o agente collegato. La schermata propone servizi cloud opzionali a pagamento. La generazione tramite agente resta fuori da questa prova.
- Lo studio è stato avviato seguendo la [guida sorgente](https://github.com/nexu-io/opengame/blob/f17151f3cc7838ddbe13d9b94392e0f983fc341b/docs/opengame-setup.md). GitHub non esponeva una [release binaria ufficiale](https://github.com/nexu-io/opengame/releases). Il test non esercita un installer gestito dal launcher.

Nel launcher Tauri su Mac, la ricerca per nome ha aperto la scheda [in italiano](scheda-it.png) e [in inglese](scheda-en.png), con anteprima della gara, licenza, link GitHub e guida di configurazione. La prova stdio ha trovato OpenGame filtrando per MCP. Il catalogo contiene 531 schede. La matrice di installazione nativa esclude OpenGame.
