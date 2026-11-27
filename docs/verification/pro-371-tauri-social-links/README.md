# Reddit e X nella prova Tauri — 27 novembre 2026

Footer: LinkedIn e Instagram conservati; aggiunte Reddit e X con SVG monocromatici, title e aria-label Reddit · Promptica / X · Promptica. Destinazioni esatte dell’operatore: https://www.reddit.com/user/Comfortable_Ice_4281/ e https://x.com/Pampaeyes . Reddit Simple Icons 16.34.0 CC0, X già attribuito; ATTRIBUTION.md registra fonte e tint.

Clic reali background nella finestra Mac IT e EN a 1440×900, sulle coordinate delle icone lette nel PNG. I log external-it.log / external-en.log confermano entrambi EXTERNAL_LINK_REQUESTED e l’URL esatto dopo che l’opener nativo ha accettato la richiesta. È verifica dell’invio al browser esterno, non della pagina remota caricata o autenticata. Nessun accesso account o invio social.

PNG catalog-it.png e catalog-en.png letti con gli alberi AX: quattro icone presenti, stato e brand separati. L’exporter conserva le aggiunte alla rigenerazione. Il bridge ammette solo l’enum website/linkedin/instagram/reddit/x e converte internamente a URL fissi; non riceve URL, shell o argomenti. Navigazione e nuove finestre nel webview continuano a essere rifiutate. Nessuna aggiunta Discord senza URL verificato.

Build offline PASS, test Tauri 2 PASS, clippy all-targets -D warnings PASS; node --check native.js e management.js, fmt e diff check PASS. Cargo seriale con target condiviso e due job. Il prodotto egui ha checkpoint e prove distinti in c670e1c, non viene sostituito dal proof.
