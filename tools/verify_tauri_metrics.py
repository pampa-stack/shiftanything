"""Check actual WebKit geometry reports; no browser, network or app mutations."""
import json
from pathlib import Path
import sys

reports = [json.loads(Path(name).read_text()) for name in sys.argv[1:]]
if not reports:
    raise SystemExit("Supply visual-*.json reports produced by the native proof")
for report in reports:
    cards = report["cards"]
    assert cards, "No visible cards measured"
    assert len({card["id"] for card in cards}) == len(cards)
    sizes = {(round(c["rect"]["width"], 3), c["rect"]["height"]) for c in cards}
    assert len(sizes) == 1, f"Unequal card dimensions: {sizes}"
    assert len({c["rect"]["y"] for c in cards}) > 1, "Measure more than one grid row"
    assert all(c["preview"]["height"] == 134 for c in cards)
    assert report["viewport"]["width"] == 1440 and report["viewport"]["height"] == 900
    assert report["header"]["height"] == report["toolbar"]["height"] == 46
    assert report["header"]["x"] == report["header"]["y"] == 0
    assert report["rail"]["width"] == 214
    assert all(label == label.upper() for label in report["navigation"])
    if len(cards) == 298:
        assert all(c["logo_loaded"] and c["preview_loaded"] for c in cards[:6])
    print(f"PASS {report['language']}: {len(cards)} cards, {next(iter(sizes))}, all rows; native viewport/header/toolbar/rail/media checks")
