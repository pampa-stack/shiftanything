/* Typed native operations only; all mutations require the Rust approval contract. */
(() => {
  const invoke = window.__TAURI__.core.invoke;
  const it = document.documentElement.lang === 'it';
  const status = document.querySelector('.status > span');
  window.addEventListener('error', event => {status.textContent = `Proof error: ${event.message}`;});
  for (const node of document.querySelector('.bar .seg').children) node.textContent = node.textContent.toUpperCase();
  const note = it ? 'Prova di sola lettura' : 'Read-only proof';
  const catalogById = new Map();
  for (const node of document.querySelectorAll('[data-view]:not([data-view="manage"])')) {
    node.dataset.proofUnavailable = '';
    node.setAttribute('aria-disabled', 'true');
    node.title = note;
    if (node instanceof HTMLButtonElement) node.disabled = true;
  }
  document.addEventListener('click', event => {
    const link = event.target.closest('a');
    if (!link) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    const social = new Map([
      ['https://promptica.it/','website'],
      ['https://www.linkedin.com/company/promptica/','linkedin'],
      ['https://www.instagram.com/promptica_ai','instagram'],
      ['https://www.reddit.com/user/Comfortable_Ice_4281/','reddit'],
      ['https://x.com/Pampaeyes','x'],
      ['https://discord.gg/Xrg2kksz2m','discord'],
    ]).get(link.href);
    const appId = link.dataset?.catalogLink || link.closest('[data-app]')?.dataset?.app || link.closest('[data-app-content]')?.querySelector?.('[data-native-app]')?.dataset.nativeApp;
    const open = social && link.closest('.footer-brand, .promptica-links') ? invoke('open_social_link', {link:social})
      : appId ? invoke('open_catalog_link', {appId,target:link.getAttribute('href')}) : null;
    if (open) {
      open.then(() => {
        status.textContent = it ? 'Link aperto nel browser esterno' : 'Link opened in external browser';
      }).catch(error => {status.textContent=String(error);});
    }
  }, true);
  for (const cell of document.querySelectorAll('.catalog-grid .cell')) {
    if (!cell.querySelector('.c-alt')) {
      const alternative = document.createElement('div');
      alternative.className = 'c-alt';
      cell.querySelector('.release-meta').before(alternative);
    }
  }
  const rect = element => {
    const r = element.getBoundingClientRect();
    return {x:r.x,y:r.y,width:r.width,height:r.height};
  };
  async function metrics() {
    if (!current?.isolated_profile || document.querySelector('.body').hidden) return;
    const cards = [...document.querySelectorAll('.catalog-grid .cell:not([hidden])')];
    const report = {
      language: document.documentElement.lang,
      viewport: {x:0,y:0,width:innerWidth,height:innerHeight},
      header:rect(document.querySelector('.top')), toolbar:rect(document.querySelector('.bar')), rail:rect(document.querySelector('.rail')),
      navigation:[...document.querySelector('.bar .seg').children].map(node => node.textContent.trim()),
      cards:cards.map(cell => ({id:cell.dataset.app,rect:rect(cell),preview:rect(cell.querySelector('.preview-shot')),
        logo_loaded:(cell.querySelector('.product-icon')?.naturalWidth ?? 0) > 1,
        preview_loaded:(cell.querySelector('.preview-shot img')?.naturalWidth ?? 0) > 1})),
    };
    await invoke('record_visual_metrics', {report});
  }
  let current;
  let debounce;
  function measureSoon() {
    if (!current?.isolated_profile) return;
    clearTimeout(debounce);
    debounce = setTimeout(() => metrics().catch(error => {
      actionError = String(error);
      status.textContent = actionError;
    }), 300);
  }
  document.addEventListener('load', measureSoon, true);
  document.addEventListener('scroll', measureSoon, true);
  document.addEventListener('input', measureSoon);
  document.addEventListener('change', measureSoon);
  document.addEventListener('click', measureSoon);
  if (document.fonts?.ready) document.fonts.ready.then(measureSoon);
  else measureSoon();
  let actionError = '';
  status.setAttribute('role','status');
  status.setAttribute('aria-live','polite');
  let favoritesKey;
  window.addEventListener('native-favorite-request',async event=>{
    try {
      const ids=await invoke('set_favorite',{appId:event.detail.id,favorite:event.detail.favorite});
      favoritesKey=JSON.stringify(ids);
      window.dispatchEvent(new CustomEvent('native-favorites',{detail:ids}));
    } catch(error) {actionError=String(error);status.textContent=actionError;}
  });
  window.nativeAppActivity = appActivity;
  let paintKey;
  let requesting = false;
  const confirmation = document.createElement('dialog');
  confirmation.className = 'native-confirmation';
  confirmation.innerHTML = `<form method="dialog"><h2></h2><p></p><label class="native-first-open" hidden><input type="checkbox"> ${it?'Apri dopo l’installazione e mostra nel Dock':'Open after installation and show in the Dock'}</label><div class="native-decision"><button type="button" data-reject>${it?'Rifiuta':'Reject'}</button><button type="button" data-approve>${it?'Approva':'Approve'}</button></div></form>`;
  const firstOpen = confirmation.querySelector('.native-first-open');
  const launchCheckbox = firstOpen.querySelector('input');
  document.body.append(confirmation);
  const labels = it ? {install:'Installa',update:'Aggiorna',open:'Apri',remove:'Rimuovi',cancel:'Annulla operazione',update_all:'Aggiorna tutte'} : {install:'Install',update:'Update',open:'Open',remove:'Remove',cancel:'Cancel operation',update_all:'Update all'};
  let pendingId;
  const sizeFormatter = new Intl.NumberFormat(document.documentElement.lang, {maximumFractionDigits: 1});
  const platformLabels = {MacOs:'macOS', Linux:'Linux', Windows:'Windows'};
  const unavailableSize = it ? 'Dimensione non disponibile' : 'Size unavailable';
  const projectGuide = entry => [entry?.review?.source, entry?.website].find(url => url?.startsWith('https://'));
  const dateFormatter = new Intl.DateTimeFormat(document.documentElement.lang, {
    year:'numeric', month:'short', day:'numeric', timeZone:'UTC'
  });
  const unavailableDate = it ? 'Data non disponibile' : 'Date unavailable';
  const phases = it ? {
    downloading:'Download in corso', reconnecting:'Connessione in ripristino', verifying:'Verifica del pacchetto',
    queued:'Download in coda', waiting:'In attesa dell’installatore', installing:'Installazione in corso', removing:'Rimozione in corso', cancelling:'Annullamento in corso',
  } : {
    downloading:'Downloading', reconnecting:'Reconnecting', verifying:'Verifying package',
    queued:'Download queued', waiting:'Waiting for installer', installing:'Installing', removing:'Removing', cancelling:'Cancelling',
  };
  const versionKey = value => (value || '').replace(/^[vV](?=\d)/, '');
  const setText = (element, value) => { if (element && element.textContent !== value) element.textContent = value; };
  function releasePresentation(snapshot, appId) {
    const live = snapshot.release_assets?.[appId];
    if (!live) return;
    const documented = catalogById.get(appId)?.release;
    const sameRelease = [live.tag, live.version].some(value => versionKey(value) === versionKey(documented?.version));
    const timestamp = live.published_at === null ? null : new Date(live.published_at * 1000);
    const date = timestamp || (sameRelease && documented?.date ? new Date(documented.date + 'T00:00:00Z') : null);
    const day = date && Number.isFinite(date.getTime()) ? date.toISOString().slice(0,10) : '';
    const size = live.bytes > 0
      ? `${sizeFormatter.format(live.bytes / 1000000)} MB · ${platformLabels[snapshot.platform.os]}`
      : unavailableSize;
    return {version:live.version, day, dateLabel:day ? dateFormatter.format(date) : unavailableDate, size};
  }
  function documentedSize(appId, os) {
    if (!platformLabels[os]) return;
    const entry = catalogById.get(appId);
    const platform = os.toLowerCase();
    const flatpak = entry?.distribution?.find(channel =>
      channel.platform === platform && channel.manager === 'flatpak' && channel.download_size > 0);
    if (flatpak) return `${sizeFormatter.format(flatpak.download_size / 1000000)} MB · Linux · Flatpak`;

    const size = entry?.release?.packages?.find(pkg => pkg.platform === platform && pkg.size_min > 0);
    if (!size) return unavailableSize;
    const low = sizeFormatter.format(size.size_min / 1000000);
    const high = sizeFormatter.format((size.size_max ?? size.size_min) / 1000000);
    return `${low === high ? low : low + '–' + high} MB · ${platformLabels[os]}`;
  }
  function applyReleaseDate(output, release) {
    if (!output) return;
    const previous = output.querySelector('time');
    if (!release.day) { setText(output, release.dateLabel); return; }
    if (previous?.dateTime === release.day) return;
    const time = document.createElement('time');
    time.dateTime = release.day;
    time.textContent = release.dateLabel;
    output.replaceChildren(time);
  }
  function progressLabel(progress) {
    if (!progress) return '';
    const label = phases[progress.phase] || '';
    if (!(progress.total > 0)) return label;
    const percent = Math.max(0, Math.min(100, Math.floor(progress.done * 100 / progress.total)));
    return `${label} · ${percent}%`;
  }
  function appActivity(snapshot, id) {
    const live = progressLabel(snapshot.progress[id]);
    if (live) return {label:live};
    const detail = snapshot.inventory.results['app:'+id];
    if (!detail) return {};
    let label;
    if (detail === 'cancelled') label = it?'Operazione annullata':'Action cancelled';
    else if (detail.startsWith('installed; could not open:')) label = it?'Installata. Apertura non riuscita.':'Installed. Could not open.';
    else if (detail.startsWith('completed:')) label = it?'Operazione completata':'Action completed';
    else if (detail.startsWith('rejected:')) label = it?'Operazione rifiutata':'Action rejected';
    else label = it?'Operazione non riuscita':'Action failed';
    return {label,detail};
  }
  function updateCatalogReleases(snapshot) {
    let datesChanged = false;
    for (const node of document.querySelectorAll('.catalog-grid .cell[data-app], .catalog-row[data-app]')) {
      const release = releasePresentation(snapshot, node.dataset.app);
      if (release) {
        if (node.dataset.release !== release.day) {
          node.dataset.release = release.day;
          datesChanged = true;
        }
        applyReleaseDate(node.querySelector('.release-meta > div:first-child > div:last-child'), release);
      }
      const size = release?.size ?? documentedSize(node.dataset.app, snapshot.platform?.os);
      if (size) setText(node.querySelector('.release-meta > div:last-child > div:last-child'), size);
    }
    if (datesChanged) document.querySelector('[data-catalog]').dispatchEvent(new Event('releases-changed'));
  }
  function primaryAction(snapshot, appId, app) {
    const entry = catalogById.get(appId);
    const platform = snapshot.platform?.os?.toLowerCase();
    if (app?.job) return {label:it?'In corso…':'Working…', enabled:false};
    if (app?.installed) return {label:labels.open, enabled:Boolean(app.detected && snapshot.lifecycle_enabled && app.can_open !== false)};
    if (platform && entry?.platform_scope_complete && !entry.platform_support?.[platform]
      && !entry.release?.packages?.some(pkg => pkg.platform === platform)
      && !entry.distribution?.some(channel => channel.platform === platform)) {
      return {label:it?'🔒 Non supportata':'🔒 Unsupported', enabled:false};
    }
    if (app?.detected && app.compatible && app.installable) return {label:labels.install, enabled:snapshot.lifecycle_enabled, operation:'install'};
    if (entry?.delivery === 'self-hosted' && projectGuide(entry)) {
      return {label:it?'Apri guida':'Open project guide', enabled:true, guide:true};
    }
    if (snapshot.lifecycle_enabled && app && !app.detected) return {label:it?'Rilevamento…':'Detecting…', enabled:false};
    return {label:it?'Pacchetto non disponibile':'Package unavailable', enabled:false};
  }
  function render(snapshot, {force = false} = {}) {
    const firstPaint = !current;
    const {updated_at: _, ...inventoryState} = snapshot.inventory;
    const nextKey = JSON.stringify([snapshot.lifecycle_enabled, snapshot.platform, inventoryState, snapshot.release_assets,
      snapshot.pending, snapshot.offer_open_after_install, snapshot.progress, snapshot.favorites]);
    current = snapshot;
    if (!force && paintKey === nextKey) return;
    paintKey = nextKey;
    if (firstPaint) measureSoon();
    window.dispatchEvent(new CustomEvent('native-snapshot',{detail:snapshot}));
    updateCatalogReleases(snapshot);
    const key=JSON.stringify(snapshot.favorites);
    if (key!==favoritesKey) {
      favoritesKey=key;
      window.dispatchEvent(new CustomEvent('native-favorites',{detail:snapshot.favorites}));
    }
    const inventory = new Map(snapshot.inventory.apps.map(app => [app.id,app]));
    const installableIds = snapshot.inventory.apps.filter(app => {
      const action = primaryAction(snapshot, app.id, app);
      return action.enabled && action.operation === 'install';
    }).map(app => app.id);
    window.dispatchEvent(new CustomEvent('native-installable', {detail:{available:snapshot.lifecycle_enabled, ids:installableIds}}));
    const detail = document.querySelector('.catalog-detail[open] [data-app-content]');
    const primary = detail?.querySelector('[data-native-app]');
    if (primary && !detail.querySelector('.native-management')) {
      const controls = document.createElement('div');
      controls.className = 'native-management';
      for (const operation of ['update','remove','cancel']) {
        const button = document.createElement('button');
        button.type='button'; button.className='btn'; button.dataset.nativeApp=primary.dataset.nativeApp; button.dataset.nativeOperation=operation;
        controls.append(button);
      }
      const result = document.createElement('p'); result.className='native-result'; result.setAttribute('role','status');
      controls.append(result); detail.append(controls);
    }
    if (detail) {
      const appId = primary?.dataset.nativeApp;
      const release = releasePresentation(snapshot, appId);
      if (release) {
        setText(detail.querySelector('.detail-facts > div:first-child > div:last-child'), release.version);
        applyReleaseDate(detail.querySelector('.detail-facts > div:nth-child(2) > div:last-child'), release);
        setText(detail.querySelector('.detail-facts > div:nth-child(3) > div:last-child'), release.size);
      }
      const output = detail.querySelector('.native-result');
      if (output) {
        const activity = appActivity(snapshot, appId);
        output.title = activity.detail || '';
        setText(output, activity.label || '');
        output.hidden = !activity.label;
      }
    }
    for (const button of document.querySelectorAll('[data-native-app]')) {
      const app = inventory.get(button.dataset.nativeApp);
      const operation = button.dataset.nativeOperation;
      if (!operation) {
        const action = primaryAction(snapshot, button.dataset.nativeApp, app);
        if (button.textContent !== action.label) button.textContent = action.label;
        button.disabled = !action.enabled;
        button.title = progressLabel(snapshot.progress[app?.id]) || action.label;
        button.classList.toggle('solid', action.enabled && action.operation === 'install');
        button.toggleAttribute('data-project-guide', action.guide === true);
        continue;
      }
      button.textContent = labels[operation];
      const allowed = app?.detected && (operation === 'cancel' ? !!app.job : !app.job && ({install:!app.installed && app.compatible && app.installable,open:app.installed && app.can_open !== false,remove:app.installed && app.can_remove !== false,update:app.installed && app.compatible && app.installable && app.update_available})[operation]);
      const cancelling = operation === 'cancel' && snapshot.progress[app?.id]?.phase === 'cancelling';
      button.disabled = !snapshot.lifecycle_enabled || !allowed || cancelling;
      button.hidden = !snapshot.lifecycle_enabled || !allowed;
      button.title = progressLabel(snapshot.progress[app?.id]) || (allowed ? labels[operation] : it?'Azione non disponibile nello stato nativo corrente':'Action unavailable in the current native state');
    }
    const controls = detail?.querySelector('.native-management');
    if (controls) controls.hidden = [...controls.children].every(child => child.hidden);
    if (snapshot.pending) {
      if (pendingId !== snapshot.pending.id) launchCheckbox.checked = Boolean(snapshot.offer_open_after_install);
      firstOpen.hidden = !snapshot.offer_open_after_install;
      pendingId = snapshot.pending.id;
      confirmation.querySelector('h2').textContent = labels[snapshot.pending.operation];
      const destination = snapshot.isolated_profile
        ? (it ? ' · Azione nel profilo temporaneo.' : ' · Action in the temporary profile.')
        : (it ? ' · Azione su questo computer.' : ' · Action on this computer.');
      confirmation.querySelector('p').textContent = snapshot.pending.apps.map(id => catalogById.get(id)?.name || id).join(', ') + destination;
      if (!confirmation.open) confirmation.showModal();
    } else { pendingId=undefined; if (confirmation.open) confirmation.close(); }
  }
  document.addEventListener('app-detail-open', () => { if (current) render(current, {force:true}); });
  async function decide(approve) {
    if (!pendingId || requesting) return;
    requesting=true;
    try {
      const openAfterInstall = approve && !firstOpen.hidden && launchCheckbox.checked;
      await invoke('decide_native_action',{requestId:pendingId,approve,openAfterInstall});
      render(await invoke('native_snapshot'));
    }
    catch(error) { actionError=String(error); status.textContent=actionError; }
    finally {requesting=false;}
  }
  confirmation.querySelector('[data-approve]').addEventListener('click',()=>decide(true));
  confirmation.querySelector('[data-reject]').addEventListener('click',()=>decide(false));
  confirmation.addEventListener('cancel',event=>{event.preventDefault();decide(false);});
  document.addEventListener('click',async event=>{
    const shot=event.target.closest('[data-shot]');
    if (shot?.dataset.shotOrigin?.startsWith('media/')) shot.dataset.shot=shot.dataset.shotOrigin;
    const button=event.target.closest('[data-native-app]');
    if (!button || button.disabled || requesting) return;
    event.preventDefault(); event.stopImmediatePropagation(); requesting=true; actionError='';
    const app=current?.inventory.apps.find(app=>app.id===button.dataset.nativeApp);
    try {
      if (button.hasAttribute('data-project-guide')) {
        const appId = button.dataset.nativeApp;
        await invoke('open_catalog_link', {appId,target:projectGuide(catalogById.get(appId))});
      } else {
        await invoke('request_native_action',{operation:button.dataset.nativeOperation || (app?.installed?'open':'install'),apps:[button.dataset.nativeApp]});
        render(await invoke('native_snapshot'));
      }
    } catch(error) {actionError=String(error);status.textContent=actionError;}
    finally {requesting=false;}
  },true);
  async function start() {
    status.textContent = `${note} · Rust…`;
    const catalog = await invoke('catalog_snapshot');
    for (const app of catalog) catalogById.set(app.id, app);
    const ids = new Set([...document.querySelectorAll('.catalog-grid [data-app]')].map(node => node.dataset.app));
    if (catalog.length !== ids.size || catalog.some(app => !ids.has(app.id))) throw Error('Native catalog differs from the local template');
    async function poll() {
    const snapshot = await invoke('native_snapshot');
    render(snapshot);
    const progress = Object.entries(snapshot.progress).map(([id, job]) =>
      `${catalogById.get(id)?.name || id} · ${progressLabel(job)}`).join(' · ');
    if (snapshot.isolated_profile) {
      const profile = snapshot.lifecycle_enabled ? (it?'Profilo temporaneo':'Temporary profile') : note;
      setText(status, `${profile} · Rust ${catalog.length} · ${snapshot.platform.os}/${snapshot.platform.arch} · ${Object.keys(snapshot.detected).length} ${it?'stati rilevati':'detected states'}${progress ? ' · ' + progress : ''}`);
    } else {
      setText(status, progress || (actionError && (it?'Azione non riuscita':'Action failed')) || (it?'Pronto':'Ready'));
    }
    status.title = actionError;
    setTimeout(() => poll().catch(error => {status.textContent=String(error);}),1000);
    }
    await poll();
  }
  start().catch(error => { status.textContent=String(error); });
})();
