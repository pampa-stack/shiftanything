"""Read-only export of Promptica's approved Doop design, with local, verified media.

No Doop API is imported or called. Generated pages are committed, so runtime/build
never needs Regia. Run with PYTHONDONTWRITEBYTECODE=1 and explicit source/cache dirs.
"""
import hashlib
import importlib
import json
from pathlib import Path
import re
import shutil
import sys
import types
from html import escape

root = Path(__file__).resolve().parents[1]
source, media = map(Path, sys.argv[1:3])
out = root / 'apps/shiftanything-tauri-proof/frontend'
out.mkdir(parents=True, exist_ok=True)
api = types.ModuleType('doop_api')
api.D = None
api.unpack = None
sys.modules['doop_api'] = api
sys.dont_write_bytecode = True
sys.path.insert(0, str(source))
design = importlib.import_module('doop_catalogo')
build = importlib.import_module('doop_build')
manifest = []
for name in ['Archivo', 'Inter', 'GeistMono']:
    (out / 'fonts').mkdir(exist_ok=True)
    shutil.copyfile(root / f'assets/fonts/{name}-Variable.ttf', out / f'fonts/{name}-Variable.ttf')
    shutil.copyfile(root / f'assets/fonts/OFL-{name}.txt', out / f'fonts/OFL-{name}.txt')

mapping = {}
for filename, url in [('icon.png', build.LAUNCHER_LOGO_URL), ('promptica.png', build.PROMPTICA_LOGO_URL)]:
    (out / 'brand').mkdir(exist_ok=True)
    shutil.copyfile(root / 'assets/shiftanything' / filename, out / 'brand' / filename)
    mapping[url] = f'brand/{filename}'
for system, url in design.SYSTEM_LOGOS.items():
    filename = {'macos': 'macos.svg', 'windows': 'windows.png', 'linux': 'linux.png'}[system]
    shutil.copyfile(root / 'assets/shiftanything' / filename, out / 'brand' / filename)
    mapping[url] = f'brand/{filename}'
for app in design.APPS[:6]:
    for slot, url in [('logo', app['logo']), *[(f'shot-{i}', u) for i, u in enumerate(app['screenshots'])]]:
        data = (media / f"{app['id']}-catalog-{slot}.image").read_bytes()
        # Native probe has already applied format/dimension/byte budgets to these bytes.
        extension = 'png' if data.startswith(b'\x89PNG') else 'jpg' if data.startswith(b'\xff\xd8') else None
        if not extension:
            raise ValueError(f'Unexpected native image format: {app["id"]}/{slot}')
        name = f"media/{app['id']}-{slot}.{extension}"
        (out / 'media').mkdir(exist_ok=True)
        (out / name).write_bytes(data)
        mapping[url] = name
        manifest.append(dict(app=app['id'], slot=slot, source=url, file=name, sha256=hashlib.sha256(data).hexdigest()))
# Unknown assets are visibly absent in this six-app proof. Never fetch remote images in the webview.
missing = 'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" width="1" height="1"/%3E'
def local_image(url, cls, alt='', *, lazy=False):
    return f'<img class="{escape(cls)}" src="{escape(mapping.get(url, missing))}" alt="{escape(alt)}" loading="{"lazy" if lazy else "eager"}">'
design.image = local_image
build.MARK = lambda size=26, cls='mark', **kw: f'<img class="{cls}" width="{size}" height="{size}" src="{mapping[kw.get("url", build.LAUNCHER_LOGO_URL)]}" alt="{ "Promptica" if kw.get("url") == build.PROMPTICA_LOGO_URL else "Sh*ftAnything"}">'
design.MARK = build.MARK
# Explicitly read-only at this checkpoint: no preview/simulated installs or generic command bridge.
def action(lang, app):
    label = 'Installa' if lang == 'it' else 'Install'
    reason = 'Prova di sola lettura' if lang == 'it' else 'Read-only proof'
    return f'<button class="btn" type="button" data-native-app="{app["id"]}" disabled title="{reason}">{label}</button>'
design.action = action
def native_catalog_script(script):
    # Keep profile persistence in the typed Rust store when regenerating Doop pages.
    script, removed = re.subn(r"  const storageKey = .*?\n", "", script, count=1)
    assert removed == 1
    script, removed = re.subn(r"  try \{\n    const saved = .*?  \} catch \{\}\n", "", script, count=1, flags=re.S)
    assert removed == 1
    begin = script.index('  function updateHearts()')
    end = script.index("  document.addEventListener('error'", begin)
    script = script[:begin] + """  function updateHearts() {
    for (const heart of document.querySelectorAll('[data-favorite-app]')) {
      const active=favorites.has(heart.dataset.favoriteApp);
      heart.setAttribute('aria-pressed',active);
      const name=heart.getAttribute('aria-label').split(' · ').slice(1).join(' · ');
      const label=document.documentElement.lang==='it' ? (active?'Rimuovi dai preferiti':'Salva nei preferiti') : (active?'Remove from favorites':'Save to favorites');
      heart.setAttribute('aria-label',label+' · '+name);
      heart.title=label+' · '+name;
    }
  }
""" + script[end:]
    begin = script.index("  document.addEventListener('click', event => {\n    const heart")
    end = script.index('  if (!catalog) return;', begin)
    return script[:begin] + """  document.addEventListener('click', event => {
    const heart = event.target.closest('[data-favorite-app]');
    if (!heart) return;
    const id = heart.dataset.favoriteApp;
    window.dispatchEvent(new CustomEvent('native-favorite-request',{detail:{id,favorite:!favorites.has(id)}}));
  });
  window.addEventListener('native-favorites',event=>{
    favorites=new Set(event.detail);
    updateHearts();
    if (catalog) catalog.dispatchEvent(new Event('favorites-changed'));
  });
""" + script[end:]

for language in ['it', 'en']:
    page = design.listing(language, 'all_apps')
    styles = '\n'.join(re.findall(r'<style>(.*?)</style>', page, re.S))
    styles = styles.replace(build.FONT_ROOT + '/', 'fonts/')
    scripts = re.findall(r'<script>(.*?)</script>', page, re.S)
    assert len(scripts) == 1
    page = re.sub(r'<style>.*?</style>', '<link rel="stylesheet" href="design.css"><link rel="stylesheet" href="proof.css">', page, flags=re.S)
    page = re.sub(r'<script>.*?</script>', '<script src="catalog.js"></script><script defer src="native.js"></script><script defer src="management.js"></script>', page, flags=re.S)
    page = page.replace('<meta charset="utf-8">', '<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">')
    # Brand URLs are generated inside the credit function, separately from catalog image().
    for url, target in mapping.items():
        page = page.replace(url, target)
    for app in design.APPS:
        page = page.replace(f'<div class="c-name">{escape(app["name"])}</div>', f'<div class="c-name" title="{escape(app["name"])}">{escape(app["name"])}</div>')
    page = page.replace('</span></footer>', '<a href="https://www.reddit.com/user/Comfortable_Ice_4281/" target="_blank" rel="noopener noreferrer" aria-label="Reddit · Promptica" title="Reddit · Promptica"><svg width="13" height="13" aria-hidden="true" fill="currentColor" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>Reddit</title><path d="M12 0C5.373 0 0 5.373 0 12c0 3.314 1.343 6.314 3.515 8.485l-2.286 2.286C.775 23.225 1.097 24 1.738 24H12c6.627 0 12-5.373 12-12S18.627 0 12 0Zm4.388 3.199c1.104 0 1.999.895 1.999 1.999 0 1.105-.895 2-1.999 2-.946 0-1.739-.657-1.947-1.539v.002c-1.147.162-2.032 1.15-2.032 2.341v.007c1.776.067 3.4.567 4.686 1.363.473-.363 1.064-.58 1.707-.58 1.547 0 2.802 1.254 2.802 2.802 0 1.117-.655 2.081-1.601 2.531-.088 3.256-3.637 5.876-7.997 5.876-4.361 0-7.905-2.617-7.998-5.87-.954-.447-1.614-1.415-1.614-2.538 0-1.548 1.255-2.802 2.803-2.802.645 0 1.239.218 1.712.585 1.275-.79 2.881-1.291 4.64-1.365v-.01c0-1.663 1.263-3.034 2.88-3.207.188-.911.993-1.595 1.959-1.595Zm-8.085 8.376c-.784 0-1.459.78-1.506 1.797-.047 1.016.64 1.429 1.426 1.429.786 0 1.371-.369 1.418-1.385.047-1.017-.553-1.841-1.338-1.841Zm7.406 0c-.786 0-1.385.824-1.338 1.841.047 1.017.634 1.385 1.418 1.385.785 0 1.473-.413 1.426-1.429-.046-1.017-.721-1.797-1.506-1.797Zm-3.703 4.013c-.974 0-1.907.048-2.77.135-.147.015-.241.168-.183.305.483 1.154 1.622 1.964 2.953 1.964 1.33 0 2.47-.81 2.953-1.964.057-.137-.037-.29-.184-.305-.863-.087-1.795-.135-2.769-.135Z"/></svg></a><a href="https://x.com/Pampaeyes" target="_blank" rel="noopener noreferrer" aria-label="X · Promptica" title="X · Promptica"><svg width="13" height="13" aria-hidden="true" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><title>X</title><path d="M14.234 10.162 22.977 0h-2.072l-7.591 8.824L7.251 0H.258l9.168 13.343L.258 24H2.33l8.016-9.318L16.749 24h6.993zm-2.837 3.299-.929-1.329L3.076 1.56h3.182l5.965 8.532.929 1.329 7.754 11.09h-3.182z"/></svg></a>' + '</span></footer>', 1)
    page = '\n'.join(line.rstrip() for line in page.splitlines()) + '\n'
    (out / f'{language}.html').write_text(page)
    (out / 'design.css').write_text(styles)
    (out / 'catalog.js').write_text(native_catalog_script(scripts[0]))
(out / 'index.html').write_text((out / 'it.html').read_text())
(root / 'apps/shiftanything-tauri-proof/media-provenance.json').write_text(json.dumps(dict(
    design={name: hashlib.sha256((source/name).read_bytes()).hexdigest() for name in ['doop_build.py', 'doop_catalogo.py', 'doop_catalogo.js']},
    media=manifest), indent=2) + '\n')
print(f'Exported IT/EN, {len(design.APPS)} apps, {len(manifest)} verified local media slots')
