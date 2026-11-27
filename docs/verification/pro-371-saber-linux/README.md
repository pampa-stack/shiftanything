# Saber su Ubuntu 24.04 amd64

Il 27 novembre 2026, su un guest AX42 isolato, il canale Flathub stabile `com.adilhanney.saber` ha superato installazione, avvio grafico, aggiornamento, secondo avvio e rimozione. Il profilo sintetico è rimasto integro e l'elenco delle app Flatpak del profilo è tornato vuoto.

- Versione 1.37.1, commit `2a465e0a706c62f4f376d9bd3b98f5e704d7f74db7841d68b05c270b05f63a67`. [Finestra prima dell'aggiornamento](previous.png).
- Versione 1.38.0, commit `d29590321c0ab02686b13c41a154ffaf79b1b7b9d65b6ce648b3c772971c072c`. [Finestra dopo l'aggiornamento](current.png).

Entrambi i commit hanno una firma GPG Flathub valida. Le due finestre mostrano la schermata degli appunti. La rete del processo era limitata a loopback. GNOME Keyring forniva Secret Service nella sessione utente, già autorizzato dal manifesto Flatpak.

Il ciclo NativeBackend precedente aveva installato, aggiornato e rimosso gli stessi commit. L'apertura di 1.38.0 in quel banco falliva senza un servizio Secret Service disponibile, quindi la scheda era rimasta esclusa. Questa prova completa la verifica grafica. L'abilitazione vale per Linux x64.

JASP rimane fuori dopo un errore ABI Qt sulla revisione precedente. Speech Note rimane fuori dopo un avvio senza finestra e con un runtime KDE fuori supporto.
