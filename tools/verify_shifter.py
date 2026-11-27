"""Verify full and projected MCP comparisons against the Rust contract JSON."""
import json
import sys
from pathlib import Path
from shifter import compare, manual_compare, normalize

root = Path(__file__).resolve().parents[1]
data = json.loads((root / 'crates/core/shifter.json').read_text())
apps = json.loads((root / 'crates/core/curated.json').read_text())
profiles = {profile['id']: profile for profile in data['profiles']}
cases = json.loads(Path(sys.argv[1]).read_text())
for index, case in enumerate(cases):
    actual = (manual_compare(data, apps, case.get('app_ids', []), case['selected'], case['required'])[0]
              if case.get('manual') else compare(profiles[case['profile_id']], apps, case['selected'], case['required']))
    expected_count = len(case.get('app_ids') or apps) if case.get('manual') else len(apps)
    assert len(actual) == expected_count, f'Incomplete comparison in case {index}: {len(actual)} of {expected_count}'
    if 'result_app_ids' in case:
        scope = set(case['result_app_ids'])
        actual = [row for row in actual if row['app_id'] in scope]
        assert {row['app_id'] for row in actual} == scope, f'Invalid result_app_ids in case {index}'
    expected = case['results']
    if actual != expected:
        mismatch = next(((a, b) for a, b in zip(actual, expected) if a != b), None)
        raise AssertionError(f"Case {index} ({case.get('profile_id', 'manual')}): "
                             f"{len(actual)} Python rows vs {len(expected)} Rust rows; first difference: {mismatch}")
assert normalize('PHÓTOSHÓP®') == normalize('Photoshop')
assert normalize('Adobe™ Photoshop') == normalize('Adobe Photoshop')
print(f'PASS Rust/MCP scoring parity for {len(cases)} selected/required subsets')
