# Sh*ftAnything · scansione dell'inventario Flatpak

`a736dad90b8aa5b6ec3014047f350cceca212931` riusa l'elenco dei pacchetti e lo stato della sorgente Flathub per una sola scansione. Il motore controlla ancora per ogni pacchetto installato il commit, il percorso e la proprietà. Le operazioni verificano lo stato reale al momento dell'esecuzione.

Su Strumenti, usando Flatpak 1.16.6 in un profilo vuoto e senza sorgenti configurate, il costo delle chiamate CLI è sceso da 162 comandi per 81 ricette a due comandi. Mediana di tre misure per serie. La misura riguarda esclusivamente i sottoprocessi locali, senza rete o rendering della finestra.

| Serie | Campioni (ms) | Mediana (ms) |
|---|---|---:|
| 162 chiamate | 5.190,4 / 5.137,6 / 5.264,8 | 5.190,4 |
| 2 chiamate | 58,0 / 53,4 / 53,4 | 53,4 |

Suite nativa su VPS2 con la patch: 616 passati, 4 ignorati preesistenti. I test coprono la precedenza del profilo utente, gli errori di lettura, la prima installazione senza sorgente configurata e il rifiuto di una sorgente diversa da quella prevista.

## Verifica dal pacchetto Linux

Il DEB amd64 `shiftanything_0.20.3_amd64-local.deb` è stato compilato su VPS2 dal commit `a736dad90b8aa5b6ec3014047f350cceca212931`. SHA-256 `3f4979c383ea2d8fc993c1c5a8bf9bb19e6e5eed9cb8c4e01d3ebd7b6e3f733a`, confermato anche sulla copia in `/Volumes/T7-Backup/builder-staging-agenti/shift-flatpak-inventory.YqRo1Q/`.

Su Strumenti con Flatpak 1.16.6, Tauri ha aperto il catalogo di 531 app a 1440×900 in un ambiente con solo rete loopback. Il profilo era vuoto e il motore ha rilevato tutti gli 82 ID Flatpak attesi come installabili. La ricerca AppFlowy ha restituito una sola scheda e il pulsante INSTALLA era visibile e abilitato nell’albero di accessibilità. La prova ha lasciato vuoti i pacchetti e le sorgenti del profilo. AppFlowy mostrava il logo, con anteprima assente nella prova offline.

[Catalogo](catalog-it.png) · [Scheda AppFlowy](appflowy-it.png) · [Accessibilità](appflowy-it.ax.json) · [Misure CLI](flatpak-cli-bench.log) · [Ciclo AppFlowy separato](../pro-371-linux-appflowy/README.md)
