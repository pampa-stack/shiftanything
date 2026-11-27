# Controllare release e changelog

`tools/aggiorna_release.py` recupera metadati pubblici delle release stabili dai repository GitHub di origine e dai canali Flathub indicati nel catalogo. Usa Python 3.11 o successivo e `gh` già configurato. Il controllo scarica solo risposte JSON. Le dimensioni si riferiscono agli asset di rilascio o al download di Flathub. [API GitHub](https://docs.github.com/en/rest/releases/releases#get-the-latest-release) · [Flathub](https://flathub.org/apps/org.gimp.GIMP)

Dalla radice della repo, stampare il risultato del controllo di due schede.

```sh
python3 tools/aggiorna_release.py --app meetily --app gimp
```

`--app` limita il controllo a ID del catalogo ed è ripetibile. Senza filtro viene letto il catalogo intero. Una versione precedente, una beta o un tag non confrontabile preservano la scheda in ingresso e compaiono nei risultati saltati. Un errore nel recupero delle fonti interrompe lo snapshot.

Per conservare i risultati, passare una directory nuova. Il secondo controllo può usare i dati e i changelog del primo per riconoscere anche annunci aggiunti alla stessa versione.

```sh
shift_review_dir=$(mktemp -d)
python3 tools/aggiorna_release.py --app meetily --app gimp --output "$shift_review_dir/first"
python3 tools/aggiorna_release.py --app meetily --app gimp \
  --catalog "$shift_review_dir/first/curated.json" \
  --baseline "$shift_review_dir/first/release-notes.json" \
  --output "$shift_review_dir/second"
```

- `curated.json` contiene la copia del catalogo con i metadati osservati. Il motore di installazione mantiene la sua verifica di pacchetti e piattaforme.
- `release-report.json` riporta nuove versioni, modifiche ai metadati, controlli invariati e rollback rifiutati. Compare per ultimo, a snapshot completo.
- `shifter-proposals.json` contiene le righe candidate con versione, fonte e profili da rivedere. L'importazione di evidenze in SH*FTER richiede la verifica della funzione, dell'edizione gratuita e delle eventuali limitazioni. La copertura si calcola dalle evidenze approvate.
- `release-notes.json` conserva le righe candidate già osservate. Il primo controllo di una versione già in catalogo serve da riferimento. Un successivo controllo rileva le righe aggiunte.

Il parser riconosce formule comuni di aggiunta, supporto, rimozione e deprecazione. Changelog senza note strutturate e fonti fuori da GitHub e Flathub richiedono verifica manuale.

## Prove offline

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tools/tests -p 'test_aggiorna_release.py' -v
```

Le fixture del 27 novembre 2026 contengono campi selezionati da due fonti ufficiali.

- Meetily v0.5.0, metadati della versione, asset e una riga del changelog. [API GitHub](https://api.github.com/repos/Zackriya-Solutions/meetily/releases/tags/v0.5.0)
- GIMP 3.2.8 e 3.2.6, metadati AppStream e dimensione. [AppStream](https://flathub.org/api/v2/appstream/org.gimp.GIMP) · [Riepilogo Flathub](https://flathub.org/api/v2/summary/org.gimp.GIMP)

I casi coprono avanzamento, ripetizione invariata, riga aggiunta a versione identica, rollback, release di anteprima e conservazione dei file di ingresso. Il connettore originale proviene da `specs/launcher-scale-anything/aggiorna_release.py`, commit `44d08184` della repo Regia.
