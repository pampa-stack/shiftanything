#!/usr/bin/env bash
# Preflight dell'archivio Umi-OCR Linux: librerie mancanti per il plugin Qt xcb e apertura della finestra.
# Uso: preflight.sh <cartella con l'archivio estratto> <cartella di uscita>
# Richiede Docker e le immagini shift-umi-guest:base (Dockerfile.base) e shift-umi-guest:xcb (Dockerfile.xcb).
# I container girano senza rete, senza capability e come utente 1000.
set -euo pipefail
app=$(realpath "${1:?cartella Umi-OCR_Linux_Paddle_2.1.5}")
out=$(realpath "${2:?cartella di uscita}")
here=$(dirname "$(realpath "$0")")
run() { docker run --rm --network none --cap-drop ALL --security-opt no-new-privileges --user 1000:1000 "$@"; }
ldd_check='E=/app/UmiOCR-data/.embeddable; Q=$E/lib/python3.10/site-packages/PySide2/Qt
export LD_LIBRARY_PATH=$E/lib:$Q/lib
missing=$(ldd $Q/plugins/platforms/libqxcb.so | grep "not found" | sort -u || true)
echo "$missing"; echo "librerie mancanti: $(echo -n "$missing" | grep -c . || true)"'
{
    echo "## senza i pacchetti xcb (immagine base)"
    run -v "$app:/app:ro" shift-umi-guest:base bash -c "$ldd_check"
    echo "## con i pacchetti xcb (immagine xcb)"
    run -v "$app:/app:ro" shift-umi-guest:xcb bash -c "$ldd_check"
} | tee "$out/ldd.log"
run -v "$app:/app" -v "$here/open-window.sh:/run-umi.sh:ro" -v "$out:/out" shift-umi-guest:xcb bash /run-umi.sh | tee "$out/open-window.log"
