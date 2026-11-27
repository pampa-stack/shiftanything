"""Deterministic evidence comparison; contract checked against the Rust core."""
import unicodedata


def normalize(value):
    value = value.translate(str.maketrans('', '', '™®℠'))
    return ''.join(c.lower() for c in unicodedata.normalize('NFKD', value)
                   if not unicodedata.combining(c) and c.isalnum())


def compare(profile, apps, selected, required):
    allowed = {feature['id'] for feature in profile['features']}
    if len(set(selected)) != len(selected) or len(set(required)) != len(required):
        raise ValueError('Feature IDs must be unique')
    if not set(selected) <= allowed or not set(required) <= set(selected):
        raise ValueError('Unknown feature or required feature outside the selection')
    names = {normalize(name) for name in [profile['name'], *profile['aliases']]}
    evidence = {candidate['app_id']: candidate['evidence'] for candidate in profile['candidates']}
    results = []
    app_names = {app['id']: normalize(app['name']) for app in apps}
    for app in apps:
        if app['id'] not in evidence and not names.intersection(normalize(name) for name in app['alternatives']):
            continue
        states = evidence.get(app['id'], {})
        state = lambda feature: states.get(feature, {}).get('state', 'unverified')
        counts = {kind: sum(state(feature) == kind for feature in selected)
                  for kind in ['supported', 'partial', 'unsupported', 'unverified']}
        coverage = None
        if selected and counts['unverified'] < len(selected):
            coverage = (200 * counts['supported'] + len(selected)) // (2 * len(selected))
        results.append(dict(app_id=app['id'], **counts, coverage=coverage,
                            missing_required=[feature for feature in required if state(feature) != 'supported']))
    results.sort(key=lambda row: (len(row['missing_required']),
                                 -(row['coverage'] if row['coverage'] is not None else -1),
                                 app_names[row['app_id']], row['app_id']))
    return results


def manual_compare(data, apps, app_ids, selected, required):
    if len(set(app_ids)) != len(app_ids) or any(app_id not in {app['id'] for app in apps} for app_id in app_ids):
        raise ValueError('Unknown or duplicate app')
    if len(set(selected)) != len(selected) or len(set(required)) != len(required) or not set(required) <= set(selected):
        raise ValueError('Invalid feature selection')
    evidence = {}
    for profile in data['profiles']:
        for candidate in profile['candidates']:
            target = evidence.setdefault(candidate['app_id'], {})
            for feature, value in candidate['evidence'].items():
                target[profile['id'] + ':' + feature] = value
    # Use the identical score/sort path, with explicitly selected catalog apps.
    profile = dict(name='', aliases=[], features=[{'id': feature} for feature in selected],
                   candidates=[dict(app_id=app_id, evidence=evidence.get(app_id, {})) for app_id in app_ids])
    return compare(profile, [app for app in apps if app['id'] in app_ids], selected, required), evidence
