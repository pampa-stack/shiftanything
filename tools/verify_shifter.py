"""Verify MCP comparison against all Rust-exported subsets. Takes the contract JSON path."""
import json
import sys
from pathlib import Path
from shifter import compare, manual_compare, normalize

root = Path(__file__).resolve().parents[1]
data = json.loads((root / 'crates/core/shifter.json').read_text())
apps = json.loads((root / 'crates/core/curated.json').read_text())
profiles = {profile['id']: profile for profile in data['profiles']}
cases = json.loads(Path(sys.argv[1]).read_text())
for case in cases:
    actual = (manual_compare(data, apps, case['app_ids'], case['selected'], case['required'])[0]
              if case.get('manual') else compare(profiles[case['profile_id']], apps, case['selected'], case['required']))
    assert actual == case['results'], case
assert normalize('PHÓTOSHÓP®') == normalize('Photoshop')
assert normalize('Adobe™ Photoshop') == normalize('Adobe Photoshop')
print(f'PASS Rust/MCP scoring parity for {len(cases)} selected/required subsets')
