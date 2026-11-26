# Catalogo GUI: 336 schede — 27 novembre 2026

Import dalla fonte Regia `ab0979b713011d1e0ef33f3858aa0e8ff46528df`, sola lettura. 336 app, 24 categorie. Restic/Wine/Chatbox esclusi; Backrest sostituisce restic nelle evidenze SH*FTER. Trova app importato come seed, senza dichiarare implementazione della pagina. Nessuna ricetta aggiunta o abilitata.

SHA-256 delle tre fonti importate:

- `catalogo-curato.json`: `ccc34a660569fb14a37d735a8a97f3c12136d6770577906650ac596ea3f7959c`
- `shifter.json`: `96e3603934030acc199cfb4d886f3033bdd132ee3d34f5cc30b030f83085023d`
- `trova-app.json`: `f2930ee12e7adce69f9644c87f43cb28b6e46ff7cb83d427a557c1a34458f8a1`

Eseguito: `cargo test -p artcraft-launcher-core --offline`: 117 PASS, 1 test già ignorato; clippy core all-targets `-D warnings`: PASS; fmt/diff check: PASS.
`cargo run -p artcraft-launcher-core --example shifter_contract --offline` + `python3 tools/verify_shifter.py /tmp/shift-catalog336-shifter.json`: 13.611 combinazioni selezionate/indispensabili con scoring Rust/MCP identico.
`uv run --offline tools/verify_mcp.py`: stdio reale, 336 record, 8 profili, ricerca/filtri, esclusioni e Backrest PASS. Le azioni in questo script sono fixture IPC esplicite, non nuovi cicli di installazione.

Log: `/tmp/shift-catalog336-{core,clippy,mcp}.log`. Prove di pacchetto, ricette e frontend restano separate. Il catalogo non amplia i permessi del backend.
