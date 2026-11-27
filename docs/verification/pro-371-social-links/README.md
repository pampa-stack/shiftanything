# Footer Reddit e X — 27 novembre 2026

Aggiunti accanto a LinkedIn e Instagram, senza sostituirli. Destinazioni esatte fornite dall’operatore: https://www.reddit.com/user/Comfortable_Ice_4281/ e https://x.com/Pampaeyes . Controlli Link con nome accessibile Promptica Reddit / Promptica X, tooltip e apertura esterna tramite lo stesso open_external_link già usato dagli altri social.

Test reale egui_kittest in IT/EN: catalog_and_agent_connection_render_in_both_languages PASS. Clic AccessKit su tutti e quattro i link e verifica dell’OutputCommand::OpenUrl con URL esatto. Screenshot catalog-it.png e catalog-en.png generati a 1440×900 e letti: quattro icone visibili e stato separato, nessuna sovrapposizione. È una prova del percorso UI di apertura; non autentica o invia dati ai profili.

Comandi: cargo test -p artcraft-launcher-ui-egui catalog_and_agent_connection_render_in_both_languages --offline; cargo clippy -p artcraft-launcher-ui-egui --all-targets --offline -- -D warnings; cargo fmt --all --check; git diff --check. Test e clippy PASS, con target condiviso e CARGO_BUILD_JOBS=2. Reddit Simple Icons 16.34.0 CC0 e X già attribuito; provenienza in assets/social/README.md e ATTRIBUTION.md. Nessun Discord aggiunto in assenza del link confermato.
