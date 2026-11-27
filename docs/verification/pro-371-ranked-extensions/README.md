# Installazioni verificate su Air e VPS Strumenti

Il 27 novembre 2026 i pacchetti ufficiali sono stati installati, aggiornati e rimossi tramite `NativeBackend` in profili temporanei. Le firme dei pacchetti e la persistenza dei dati di prova sono state controllate. Pacchetti, installazioni e runtime di prova sono stati rimossi dopo i cicli.

- **Clash Verge Rev**, macOS arm64 da 11.0. DMG 2.5.9 → 2.5.10 con Developer ID `JPH3Z7PPBB` e SHA-256 nel registro. GPL-3.0. La finestra e le funzionalità di rete restano da provare. [Fonte ufficiale](https://github.com/clash-verge-rev/clash-verge-rev/releases/tag/v2.5.10)
- **Orca**, macOS arm64 da 13.4. DMG 1.4.279 → 1.4.280 con Developer ID `6CX3WHS9HZ` e SHA-256 nel registro. La finestra resta da provare in un profilo dedicato. [Fonte ufficiale](https://github.com/stablyai/orca/releases/tag/v1.4.280)
- **Floorp**, Linux x64. Flatpak 12.21.0 → 12.21.1, browser aperto in un profilo isolato a entrambe le versioni. [Screenshot](floorp.png) · [Flathub](https://flathub.org/apps/one.ablaze.floorp)
- **PrusaSlicer**, Linux x64. Flatpak 2.9.6 → 2.9.7. L’assistente iniziale ha segnalato il limite di rete del banco di prova. Dopo averlo chiuso, Print Settings e Plater sono stati aperti con un clic reale. AGPL-3.0. [Screenshot](prusaslicer.png) · [Flathub](https://flathub.org/apps/com.prusa3d.PrusaSlicer)
- **Bottles**, Linux x64. Flatpak 67.6 → 67.7. Con `Skip Setup` la finestra principale si apre offline. Il download del runtime Wine e l’avvio di app al suo interno restano da provare. [Screenshot](bottles.png) · [Flathub](https://flathub.org/apps/com.usebottles.bottles)
- **Mumble**, Linux x64. Flatpak 1.5.915 → 1.5.939. È stata aperta la finestra principale dopo l’assistente, senza accedere a un server o usare il microfono. [Screenshot](mumble.png) · [Flathub](https://flathub.org/apps/info.mumble.Mumble)

I DMG sono limitati ai file verificati per Apple Silicon. I cicli Linux usano il canale Flathub firmato per x64. Le versioni indicate sono quelle esercitate sui rispettivi canali.
