# Surge XT su Linux x64

Il 27 novembre 2026 il NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso Surge XT standalone in un profilo dedicato su Ubuntu 24.04 amd64. Il motore ha rifiutato di modificare una copia esterna e ha conservato il dato sintetico dopo l'aggiornamento e la rimozione. Le versioni aperte mostravano entrambe l'editor principale. Nel guest l'audio era disabilitato e l'app aveva accesso alla sola rete loopback. [Apertura prima](surge-xt/previous.png) · [Apertura dopo](surge-xt/current.png) · [Ciclo NativeBackend](surge-xt/lifecycle.log)

Il codice è disponibile sotto GPL-3.0. Il progetto documenta il pacchetto Flathub, gestito dalla comunità. Flathub ha firmato con la chiave `4184DD4D907A7CAE` entrambe le revisioni di Surge XT 1.3.4. Il passaggio ha aggiornato la revisione del pacchetto e il runtime, da Freedesktop 25.08 a 26.08. [Codice del progetto](https://github.com/surge-synthesizer/surge) · [Documentazione Linux](https://github.com/surge-synthesizer/surge/blob/main/doc/Linux%20and%20Other%20Unix-like%20Distributions.md#surge-xt-on-flathub) · [Pacchetto Flathub](https://flathub.org/apps/org.surge_synth_team.surge-xt)

- Revisione precedente `c6cca9b482891a1be471dbdca98e40f826f7ab191f7b554ed817938a20f5e157`. [Firma](surge-xt/previous.signature.txt) · [Pacchetto](surge-xt/previous.package.txt)
- Revisione corrente `c7c47e97252bf2b569d2b8e710868828c15d4ea10727f7d041f087c2c6e11616`. [Firma](surge-xt/current.signature.txt) · [Pacchetto](surge-xt/current.package.txt)
- SHA-256 del probe con ricetta candidata `ce1bcb50d74540989f6a4b41d59d83775c3a97c67297fa413625d30343f74e41`.
