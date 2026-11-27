# Collegamento OpenCode 2 e MCP su macOS

Il 27 novembre 2026 una build Tauri locale del ramo beta ha aperto **Collega agente · MCP** in un profilo temporaneo. Il controllo iniziale mostrava «Nessuna richiesta recente». Il comando copiato dall’interfaccia ha avviato il server MCP su stdio e una chiamata `catalog_filters` ha restituito 530 app. Un secondo clic su **Verifica collegamento** ha mostrato «Agente collegato».

OpenCode v2.3.2 ha caricato lo stesso JSON in una configurazione temporanea. Con il suo server locale in ascolto su loopback, `opencode mcp list` ha riportato `shiftanything connected` dopo l’inizializzazione. Il comando `opencode debug paths` ha confermato la cartella della configurazione. Le due prove hanno usato profili separati da quelli personali, senza chiamate a modelli o operazioni di installazione. [Configurazione MCP di OpenCode 2](https://opencode.ai/docs/mcp-servers/#local) · [Segnalazione OpenCode sul ritardo della lista iniziale](https://github.com/anomalyco/opencode/issues/63053)

La finestra macOS è stata aperta a 1440 × 900 in italiano e inglese. In entrambe le lingue, selezionare **OpenCode 2** mostra il nome `opencode.jsonc` e il comando per localizzarlo. Tornando a **Codex**, l’istruzione specifica di OpenCode scompare.
