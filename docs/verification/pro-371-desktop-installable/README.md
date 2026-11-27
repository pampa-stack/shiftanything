# Filtro Installabili qui

Prova del 27 novembre 2026, build `d60eca2`, catalogo da 531 schede. Il filtro è stato selezionato e deselezionato da entrambe le finestre Tauri a 1440 × 900. I risultati sono stati confrontati con l'intersezione degli ID idonei restituiti dal motore nativo e gli ID del catalogo, usando un profilo nuovo. Il reset ha ripristinato tutte le 531 schede.

- Mac Apple Silicon: 49 schede idonee con rete attiva. DMG locale SHA-256 `7ac6dbae47aff6c3d9d2d8c796f077984699e633e5651f195be03f8fb51b437f`. [Finestra](macos-it.png) · [Esito](macos-result.json)
- Ubuntu amd64 (VPS2): 27 schede idonee con rete attiva. Il banco non disponeva di Flatpak. DEB locale SHA-256 `8ade051f7f0490cb1c6833b7fdc17ec9bd8233ed3254fd05d6964a4de92089fb`. [Finestra](linux-it.png) · [Esito](linux-result.json)

Il conteggio dipende dal sistema, dal gestore di pacchetti installato e dalla disponibilità delle fonti di rilascio. I cicli di installazione reali sono registrati separatamente.
