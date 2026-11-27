# Controllo di coerenza del catalogo nel DMG

Il 27 novembre 2026 il DMG ricostruito dopo `90156b3` con questa correzione ha aperto una finestra macOS Apple Silicon in un profilo temporaneo. Il DOM WebKit ha misurato 532 schede, tutte 408,328 × 400 a 1440 × 900. I sei loghi e le sei anteprime iniziali erano caricati. Il footer mostra Rust 532, macOS ARM64 e 49 stati rilevati. [Catalogo](catalog-it.png)

I clic in background hanno premuto Installa sulla scheda Handy e Rifiuta nella finestra di consenso. Il bridge Rust ha ricevuto una richiesta `install` su `handy` e, dopo il rifiuto, lo stato si è azzerato senza avviare un download. [Consenso](handy-consent-it.png)

Il DMG precedente rendeva visibili 533 schede HTML per 532 schede Rust, lasciando indisponibili le azioni. L'esportazione adesso legge il JSON del prodotto. Il build script rifiuta l'HTML divergente prima dell'impacchettamento. La prova negativa ha riusato la vecchia pagina in una cartella separata. [Output](mismatch.log)

SHA-256 del DMG locale `67b5c6c05d9e7a7b0736e6d86f97cb488649f8d61b84acb319f3de2384c4e09e`. Mount in sola lettura, licenze, architettura, firma ad hoc e versione 0.20.3 verificati. La prova dei clic ha usato la copia estratta.
