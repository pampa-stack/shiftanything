# Critica del piano della beta 0.21

Fase 4/9, 28 novembre 2026.

**Modello vero usato:** Claude (agent `codex-consultant`), non Codex. L'agent ha provato `codex exec` due volte ed è stato fermato da un controllo di sicurezza sul testo del comando; il lancio diretto di `codex exec` (gpt-6.1-sol) è fallito per quota esaurita fino al 3 dicembre 2026. **La critica di un modello di un'altra famiglia manca ancora**: la fase 4 non è chiusa finché non arriva.

Esito della lettura di Claude: 0 Critical, 7 High, 7 Medium, 1 Low. Quelli verificati da me sul codice sono già nel piano:

| Severità | Punto | Verificato | Dove nel piano |
|---|---|---|---|
| High | `local-deb.sh` legge solo `target/debug` | sì, riga 9 | passo 3 |
| High | note scritte su `R`, tag su `M` | sì, `release-notes.py` stampa lo SHA passato | passo 13 (alberi uguali) |
| High | bozza con `--target R` poi spostata | rischio reale, nessuna prova che GitHub lo consenta | passo 13 (bozza dopo il merge, `--target M`) |
| High | controllo della chiave pubblica facoltativo | sì, righe 140-162 | passo 5 |
| High | `tailscale serve reset` toglie tutto | sì, era nella specifica | specifica e passo 6 |
| High | `gh release download` non mette la quarantena | sì | passo 7 |
| High | aggiornamento Linux via `pkexec` senza agente | sì, `signed.rs` 206-216 | passo 6 (regola polkit nel guest) |
| High | la 0.20.9 pubblicata ha indirizzo e chiave? | sì: il binario Linux contiene l'indirizzo GitHub e la chiave | — |
| Medium | `local-app.sh` non compila, serve il percorso esatto | sì, righe 19-25 | passo 4 |
| Medium | binario Air e Pro non confrontati | — | passo 4 (SHA-256) |
| Medium | cartella servita e file della chiave | — | passi 5 e 6 |
| Medium | build di prova della 0.20.9 diversa dalla pubblicata | — | passo 6 (dichiarato) e passo 13 |
| Medium | `requireSignedVersion` da verificare nel plugin | da fare | passo 12 |
| Medium | il `.tar.gz` non si aggiorna da solo | sì, `installed_path` | passo 8 |
| Low | dipendenze Fedora del `.tar.gz` | — | passo 8 |
