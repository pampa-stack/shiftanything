"""Deterministic evidence comparison; contract checked against the Rust core."""
import unicodedata
from functools import lru_cache


_TRADEMARKS = str.maketrans('', '', '™®℠')


@lru_cache(maxsize=2048)
def normalize(value):
    value = value.translate(_TRADEMARKS)
    return ''.join(c.lower() for c in unicodedata.normalize('NFKD', value)
                   if not unicodedata.combining(c) and c.isalnum())


def profile_evidence(profile):
    evidence = {}
    for candidate in profile['candidates']:
        evidence.setdefault(candidate['app_id'], candidate['evidence'])
    return evidence


def compare(profile, apps, selected, required):
    allowed = {feature['id'] for feature in profile['features']}
    selected_set, required_set = set(selected), set(required)
    if len(selected_set) != len(selected) or len(required_set) != len(required):
        raise ValueError('Feature IDs must be unique')
    if not selected_set <= allowed or not required_set <= selected_set:
        raise ValueError('Unknown feature or required feature outside the selection')
    return _score_apps(apps, profile_evidence(profile), selected, required)


def _score_apps(apps, evidence, selected, required):
    selected_count = len(selected)
    app_names = {app['id']: normalize(app['name']) for app in apps}
    results = []
    for app in apps:
        app_id = app['id']
        states = evidence.get(app_id)
        supported = partial = unsupported = 0
        missing_required = list(required)
        if states:
            status = [states.get(feature, {}).get('state') for feature in selected]
            supported, partial, unsupported = (status.count(kind) for kind in ('supported', 'partial', 'unsupported'))
            missing_required = [feature for feature in required if states.get(feature, {}).get('state') != 'supported']
        unverified = selected_count - supported - partial - unsupported
        coverage = None
        if unverified < selected_count:
            coverage = (200 * supported + selected_count) // (2 * selected_count)
        results.append(dict(app_id=app_id, supported=supported, partial=partial, unsupported=unsupported,
                            unverified=unverified, coverage=coverage, missing_required=missing_required))
    results.sort(key=lambda row: (len(row['missing_required']),
                                 -(row['coverage'] if row['coverage'] is not None else -1),
                                 app_names[row['app_id']], row['app_id']))
    return results


def manual_compare(data, apps, app_ids, selected, required):
    wanted = set(app_ids)
    if len(wanted) != len(app_ids) or (wanted and not wanted <= {app['id'] for app in apps}):
        raise ValueError('Unknown or duplicate app')
    selected_set, required_set = set(selected), set(required)
    if len(selected_set) != len(selected) or len(required_set) != len(required) or not required_set <= selected_set:
        raise ValueError('Invalid feature selection')
    known_profiles = {}
    for profile in data['profiles']:
        known_profiles.setdefault(profile['id'], profile)
    qualified = {}
    for key in selected:
        profile_id, separator, feature = key.partition(':')
        if separator and profile_id in known_profiles:
            qualified.setdefault(profile_id, {})[feature] = key
    evidence = {}
    for profile_id, keys in qualified.items():
        profile = known_profiles[profile_id]
        declared = {feature['id'] for feature in profile['features']}
        for app_id, entries in profile_evidence(profile).items():
            for feature in keys.keys() & declared & entries.keys():
                evidence.setdefault(app_id, {})[keys[feature]] = entries[feature]
    scope = [app for app in apps if app['id'] in wanted] if wanted else apps
    return _score_apps(scope, evidence, selected, required), evidence
