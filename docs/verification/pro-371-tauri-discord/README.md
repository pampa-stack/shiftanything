# Discord Promptica nella prova Tauri — 27 novembre 2026

Aggiunto dopo gli altri social, senza sostituzioni, con icona Discord già attribuita, aria-label e tooltip Discord · Promptica. Destinazione fissa https://discord.gg/Xrg2kksz2m , enum IPC discord. Export rigenerabile aggiornato. Nessuna nuova scheda Chrome aperta per Discord.

Verifica senza inviare traffico: node tools/verify_tauri_social.mjs esegue il listener click reale native.js con invoke intercettato. Legge gli href dalle pagine IT/EN ed esercita tutti e sei i link del footer; verifica nomi, tooltip, comando open_social_link e ID chiuso. Rifiuta link sconosciuti, file e link fuori footer. È una prova del dispatcher con DOM minimo, non un clic nel browser.

Test Rust social_opener_accepts_only_closed_ids_and_propagates_failure intercetta il vero confine dispatch_social_link→opener: ciascuno dei sei ID arriva all’URL esatto, URL/argomenti/ID sconosciuti vengono rifiutati dalla deserializzazione e il fallimento dell’opener risale al chiamante. Tre test Tauri PASS, clippy all-targets -D warnings e build offline PASS; fmt, node --check e git diff --check PASS. Target condiviso, Cargo seriale, due job.

Finestra Mac reale riavviata IT/EN in profilo temporaneo e modalità readonly: PNG 1440×900 letti con AX; Discord visibile a destra degli altri social, stato separato. Le azioni native sono disabilitate perché questa acquisizione non usa --lifecycle; il ciclo precedente ecfc92c resta distinto. Nessuna release o installazione personale modificata. Il link remoto è già verificato dalla regia; qui non si dichiara una verifica remota nuova.
