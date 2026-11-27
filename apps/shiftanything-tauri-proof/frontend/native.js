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
    if (!current?.isolated_profile || !document.querySelector('.body').getClientRects().length) return;
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
  window.showDownloadProgress = showDownloadProgress;
  let paintKey;
  let requesting = false;
  const confirmation = document.createElement('dialog');
  confirmation.className = 'native-confirmation';
  confirmation.setAttribute('aria-labelledby', 'native-confirmation-title');
  confirmation.innerHTML = `<form method="dialog"><h2 id="native-confirmation-title"></h2><p></p><label class="native-first-open" hidden><input type="checkbox"> ${it?'Apri dopo l’installazione e mostra nel Dock':'Open after installation and show in the Dock'}</label><div class="native-decision"><button type="button" data-reject>${it?'Rifiuta':'Reject'}</button><button type="button" data-approve>${it?'Approva':'Approve'}</button></div></form>`;
  const firstOpen = confirmation.querySelector('.native-first-open');
  const launchCheckbox = firstOpen.querySelector('input');
  document.body.append(confirmation);
  const labels = it ? {install:'Installa',update:'Aggiorna',open:'Apri',remove:'Rimuovi',cancel:'Annulla operazione',update_all:'Aggiorna tutte'} : {install:'Install',update:'Update',open:'Open',remove:'Remove',cancel:'Cancel operation',update_all:'Update all'};
  let pendingId;
  const sizeFormatter = new Intl.NumberFormat(document.documentElement.lang, {maximumFractionDigits: 1});
  const platformLabels = {MacOs:'macOS', Windows:'Windows', Linux:'Linux'};
  const unavailableSize = it ? 'Dimensione non disponibile' : 'Size unavailable';
  const https = url => typeof url === 'string' && url.startsWith('https://');
  const projectGuide = entry => [entry?.review?.source, entry?.website, entry?.repo].find(https);
  function manualDownload(entry, platform) {
    const release = entry?.release?.source;
    if (https(release) && /^https:\/\/github\.com\/[^/]+\/[^/]+\/releases(?:[/?#]|$)/.test(release)) {
      return {label:it?'Rilasci GitHub':'GitHub releases', target:release};
    }
    const channel = entry?.distribution?.find(item => item.platform === platform && https(item.source));
    if (channel) return {label:it?'Fonte ufficiale':'Official source', target:channel.source};
    const target = [entry?.repo, entry?.website].find(https);
    if (!target) return {label:it?'Download non disponibile':'Download unavailable'};
    return {label:target.startsWith('https://github.com/') ? 'GitHub' : (it?'Fonte ufficiale':'Official source'), target};
  }
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
    const documentedVersion = versionKey(documented?.version);
    const sameRelease = Boolean(documentedVersion)
      && [live.tag, live.version].some(value => versionKey(value) === documentedVersion);
    const timestamp = live.published_at === null ? null : new Date(live.published_at * 1000);
    const date = timestamp || (sameRelease && documented?.date ? new Date(documented.date + 'T00:00:00Z') : null);
    const day = date && Number.isFinite(date.getTime()) ? date.toISOString().slice(0,10) : '';
    const size = live.bytes > 0
      ? `${sizeFormatter.format(live.bytes / 1000000)} MB · ${platformLabels[snapshot.platform.os]}`
      : unavailableSize;
    return {version:live.version, day, dateLabel:day ? dateFormatter.format(date) : unavailableDate, size, sameRelease};
  }
  function documentedPackage(appId, os) {
    if (!platformLabels[os]) return;
    const entry = catalogById.get(appId);
    const platform = os.toLowerCase();
    const flatpak = entry?.distribution?.find(channel =>
      channel.platform === platform && channel.manager === 'flatpak' && channel.download_size > 0);
    if (flatpak) return {
      size:`${sizeFormatter.format(flatpak.download_size / 1000000)} MB · Linux · Flatpak`,
      source:flatpak.download_size_source || flatpak.source,
    };

    const size = entry?.release?.packages?.find(pkg => pkg.platform === platform && pkg.size_min > 0);
    if (!size) return;
    const low = sizeFormatter.format(size.size_min / 1000000);
    const high = sizeFormatter.format((size.size_max ?? size.size_min) / 1000000);
    return {size:`${low === high ? low : low + '–' + high} MB · ${platformLabels[os]}`, source:entry.release.source};
  }
  function applyDetailPackages(output, snapshot, appId, release) {
    if (!output) return;
    const currentOs = snapshot.platform?.os;
    const useDocumented = !release || release.sameRelease;
    const platforms = useDocumented ? Object.keys(platformLabels) : [currentOs];
    const releaseSource = useDocumented ? catalogById.get(appId)?.release?.source : undefined;
    const packages = platforms.flatMap(os => {
      const documented = useDocumented ? documentedPackage(appId, os) : undefined;
      if (os === currentOs && release && release.size !== unavailableSize) return [{size:release.size, source:releaseSource}];
      return documented ? [documented] : [];
    });
    const sizeMissing = missingDownloadSize(snapshot, appId);
    const key = JSON.stringify([packages, sizeMissing]);
    if (output.dataset.packageSizes === key) return;
    output.dataset.packageSizes = key;
    if (!packages.length) { setText(output, sizeMissing); return; }
    output.replaceChildren(...packages.map(pkg => {
      const row = document.createElement('div');
      if (pkg.source) {
        const source = document.createElement('a');
        source.className = 'size-source';
        source.href = pkg.source;
        source.dataset.catalogLink = appId;
        source.textContent = pkg.size;
        row.append(source);
      } else {
        row.textContent = pkg.size;
      }
      return row;
    }));
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
  function downloadPercent(progress) {
    if (!(progress?.total > 0) || !Number.isFinite(progress.done)) return;
    return Math.max(0, Math.min(100, Math.floor(progress.done * 100 / progress.total)));
  }
  function progressLabel(progress) {
    const label = phases[progress?.phase] || '';
    const percent = downloadPercent(progress);
    return percent === undefined ? label : `${label} · ${percent}%`;
  }
  function showDownloadProgress(container, progress, appName, fallback = '') {
    const label = progressLabel(progress);
    let output = container.querySelector('.download-label');
    if (!output) {
      output = document.createElement('span');
      output.className = 'download-label';
      container.replaceChildren(output);
    }
    setText(output, label || fallback);
    let bar = container.querySelector('progress');
    const phase = progress?.phase;
    if (phase !== 'downloading' && phase !== 'reconnecting' && phase !== 'installing') {
      bar?.remove();
      return;
    }
    if (!bar) {
      bar = document.createElement('progress');
      bar.max = 100;
      container.append(bar);
    }
    bar.setAttribute('aria-label', `${phases[phase]} · ${appName}`);
    const percent = phase === 'installing' ? undefined : downloadPercent(progress);
    if (percent === undefined) bar.removeAttribute('value');
    else if (bar.position < 0 || bar.value !== percent) bar.value = percent;
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
  function missingDownloadSize(snapshot, appId) {
    if (catalogById.get(appId)?.delivery === 'self-hosted') return it?'Dipende dalla configurazione':'Varies by setup';
    const system = platformLabels[snapshot.platform?.os];
    return !system ? unavailableSize : it?`Dimensione non rilevata · ${system}`:`Size unavailable · ${system}`;
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
      const documented = !release || release.sameRelease ? documentedPackage(node.dataset.app, snapshot.platform?.os) : undefined;
      const size = release && release.size !== unavailableSize ? release.size
        : documented?.size ?? missingDownloadSize(snapshot, node.dataset.app);
      if (size) setText(node.querySelector('.release-meta > div:last-child > div:last-child'), size);
    }
    if (datesChanged) document.querySelector('[data-catalog]').dispatchEvent(new Event('releases-changed'));
  }
  function primaryAction(snapshot, appId, app, verified) {
    const entry = catalogById.get(appId);
    const platform = snapshot.platform?.os?.toLowerCase();
    if (app?.job) return {label:it?'In corso…':'Working…', enabled:false, busy:true};
    if (app?.installed) {
      const available = app.detected && snapshot.lifecycle_enabled;
      if (app.update_available) return {label:labels.update, enabled:Boolean(available && app.compatible && app.installable), operation:'update'};
      return {label:labels.open, enabled:Boolean(available && app.can_open !== false), operation:'open'};
    }
    if (platform && entry?.platform_scope_complete && !entry.platform_support?.[platform]
      && !entry.release?.packages?.some(pkg => pkg.platform === platform)
      && !entry.distribution?.some(channel => channel.platform === platform)) {
      return {label:it?'🔒 Non supportata':'🔒 Unsupported', reason:it?'Il progetto non supporta questo sistema.':'The project does not support this system.'};
    }
    if (verified && app?.detected && app.compatible && app.installable) return {label:labels.install, enabled:snapshot.lifecycle_enabled, operation:'install'};
    const guide = entry?.delivery === 'self-hosted' && projectGuide(entry);
    if (guide) {
      return {
        label:it?'Guida per il server':'Server setup guide',
        reason:it?'Si configura su un server.':'Requires setup on a server.',
        target:guide,
      };
    }
    if (snapshot.install_checks_pending?.includes(appId)) {
      return {label:labels.install, enabled:false, busy:true, reason:it?'Controllo del pacchetto…':'Checking package…'};
    }
    return {
      ...manualDownload(entry, platform),
      reason:verified ? (it?'Pacchetto compatibile non disponibile ora.':'Compatible package currently unavailable.')
        : (it?'Installazione non ancora verificata su questo sistema.':'Installation not yet verified on this system.'),
    };
  }
  function showInstallStatus(button, action, verified, platformReady, progress) {
    const surface = button.closest('.cell, .catalog-row, [data-app-content]');
    const heart = surface?.querySelector('button[data-favorite-app]');
    if (heart && platformReady) {
      let badge = heart.previousElementSibling;
      if (!badge?.classList.contains('install-indicator')) {
        badge = document.createElement('span');
        badge.className = 'install-indicator';
        heart.before(badge);
      }
      const ready = verified && action.enabled && Boolean(action.operation);
      const label = ready ? (it?'Installazione verificata':'Installation verified')
        : verified ? (it?'Installazione provata. Controlla lo stato del pacchetto.':'Installation tested. Check package status.')
          : (it?'Installazione non ancora verificata':'Installation not yet verified');
      badge.dataset.verified = String(ready);
      badge.title = label;
      badge.setAttribute('role','img');
      badge.setAttribute('aria-label',label);
      setText(badge, verified?'✓':'–');
    }
    const actions = button.parentElement;
    let reason = actions.querySelector('.install-reason');
    const reserveSpace = surface?.classList.contains('cell');
    if (!reason && (action.reason || reserveSpace || progress)) {
      reason = document.createElement('span');
      reason.className = 'install-reason download-progress';
      actions.append(reason);
    }
    if (reason) {
      reason.hidden = !action.reason && !reserveSpace && !progress;
      const appId = button.dataset.nativeApp;
      showDownloadProgress(reason, progress, catalogById.get(appId)?.name || appId, action.reason);
    }
  }
  function render(snapshot, {force = false} = {}) {
    const firstPaint = !current;
    const {updated_at: _, ...inventoryState} = snapshot.inventory;
    const nextKey = JSON.stringify([snapshot.lifecycle_enabled, snapshot.platform, inventoryState, snapshot.release_assets,
      snapshot.pending, snapshot.offer_open_after_install, snapshot.progress, snapshot.favorites, snapshot.verified_installs, snapshot.install_checks_pending]);
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
    const verified = new Set(snapshot.verified_installs);
    const installableIds = snapshot.inventory.apps.filter(app => {
      const action = primaryAction(snapshot, app.id, app, verified.has(app.id));
      return action.enabled && action.operation === 'install';
    }).map(app => app.id);
    window.dispatchEvent(new CustomEvent('native-installable', {detail:{available:snapshot.lifecycle_enabled, ids:installableIds}}));
    const detail = document.querySelector('.catalog-detail:not([hidden]) [data-app-content]');
    const primary = detail?.querySelector('[data-native-app]');
    if (primary && !detail.querySelector('.native-management')) {
      const controls = document.createElement('div');
      controls.className = 'native-management';
      for (const operation of ['remove','cancel']) {
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
      }
      applyDetailPackages(detail.querySelector('.detail-facts > div:nth-child(3) > div:last-child'), snapshot, appId, release);
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
        const proven = verified.has(button.dataset.nativeApp);
        const action = primaryAction(snapshot, button.dataset.nativeApp, app, proven);
        if (button.textContent !== action.label) button.textContent = action.label;
        button.disabled = !(action.enabled || action.target);
        button.title = progressLabel(snapshot.progress[app?.id]) || action.reason || action.label;
        button.classList.toggle('solid', action.operation === 'install' && action.enabled === true);
        if (action.operation) button.dataset.nativePrimaryOperation = action.operation;
        else delete button.dataset.nativePrimaryOperation;
        if (action.busy) button.setAttribute('aria-busy', 'true');
        else button.removeAttribute('aria-busy');
        if (action.target) button.dataset.externalTarget = action.target;
        else delete button.dataset.externalTarget;
        showInstallStatus(button, action, proven, Boolean(snapshot.platform), snapshot.progress[button.dataset.nativeApp]);
        continue;
      }
      button.textContent = labels[operation];
      const allowed = app?.detected && (operation === 'cancel' ? !!app.job : !app.job && ({install:!app.installed && app.compatible && app.installable,open:app.installed && app.can_open !== false,remove:app.installed && app.can_remove !== false,update:app.installed && app.compatible && app.installable && app.update_available})[operation]);
      const cancelling = operation === 'cancel' && snapshot.progress[app?.id]?.phase === 'cancelling';
      button.disabled = !snapshot.lifecycle_enabled || !allowed || cancelling;
      const visible = operation === 'open' ? app?.installed
        : operation === 'update' ? app?.installed && app.update_available && !app.job : allowed;
      button.hidden = !snapshot.lifecycle_enabled || !visible;
      button.title = progressLabel(snapshot.progress[app?.id]) || (allowed ? labels[operation] : it?'Azione non disponibile nello stato nativo corrente':'Action unavailable in the current native state');
    }
    document.documentElement.dataset.nativeStarted = '';
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
    try {
      if (button.dataset.externalTarget) {
        await invoke('open_catalog_link', {appId:button.dataset.nativeApp,target:button.dataset.externalTarget});
      } else {
        const operation = button.dataset.nativeOperation || button.dataset.nativePrimaryOperation;
        if (!operation) return;
        await invoke('request_native_action',{operation,apps:[button.dataset.nativeApp]});
        render(await invoke('native_snapshot'));
      }
    } catch(error) {actionError=String(error);status.textContent=actionError;}
    finally {requesting=false;}
  },true);
  async function startLauncherUpdater() {
    const banner = document.createElement('aside');
    banner.className = 'launcher-update';
    banner.hidden = true;
    banner.setAttribute('aria-label', it ? 'Aggiornamento di Sh*ftAnything' : 'Sh*ftAnything update');
    const message = document.createElement('div');
    message.setAttribute('role', 'status');
    const title = document.createElement('strong');
    const detail = document.createElement('p');
    message.append(title, detail);
    const action = document.createElement('button');
    action.type = 'button';
    banner.append(message, action);
    document.querySelector('.app > .top').after(banner);

    let update;
    let eventCount = 0;
    let pending = '';
    let commandError = '';
    function renderUpdate() {
      if (!update) return;
      const {available, status: phase, error} = update;
      const hiddenError = !commandError && ['channel_disabled', 'installation_unsupported', 'check_failed'].includes(error);
      banner.hidden = hiddenError || (!available && !['downloading', 'ready', 'error'].includes(phase) && !commandError);
      title.textContent = available
        ? (it ? `Aggiornamento Sh*ftAnything ${available.version}` : `Sh*ftAnything ${available.version} update`)
        : (it ? 'Aggiornamento di Sh*ftAnything' : 'Sh*ftAnything update');

      const failed = phase === 'error' || Boolean(commandError);
      if (failed) {
        const errors = {
          installation_cancelled: ['Installazione annullata.', 'Installation cancelled.'],
          installation_permissions: ['Serve il permesso per aggiornare Sh*ftAnything.', 'Permission is required to update Sh*ftAnything.'],
          installation_recovery_required: ['Impossibile sostituire il launcher. La copia precedente è stata conservata.', 'The launcher could not be replaced. The previous copy was preserved.'],
          download_or_signature_failed: ['Download interrotto o firma non valida.', 'Download interrupted or invalid signature.'],
          release_invalid: ['Il pacchetto di aggiornamento non ha superato la verifica.', 'The update package failed verification.'],
        };
        const text = errors[error] || ['Aggiornamento non riuscito.', 'Update failed.'];
        detail.textContent = text[it ? 0 : 1];
      } else if (phase === 'ready') {
        detail.textContent = it ? 'Aggiornamento installato. Riavvia Sh*ftAnything per applicarlo.'
          : 'Update installed. Restart Sh*ftAnything to apply it.';
      } else if (phase === 'checking' || pending === 'launcher_update_check') {
        detail.textContent = it ? 'Ricerca aggiornamenti in corso…' : 'Checking for updates…';
      } else if (phase === 'downloading' || pending === 'launcher_update_install') {
        detail.textContent = it ? 'Download e installazione in corso…' : 'Downloading and installing…';
      } else {
        detail.textContent = it ? `Versione in uso: ${update.current}` : `Current version: ${update.current}`;
      }
      action.hidden = !failed && phase !== 'available' && phase !== 'downloading';
      action.disabled = Boolean(pending) || (!failed && (phase === 'downloading' || phase === 'checking'));
      action.textContent = failed ? (it ? 'Riprova' : 'Retry') : (it ? 'Installa' : 'Install');
    }

    function acceptUpdate(next) {
      update = next;
      commandError = '';
      renderUpdate();
    }

    async function runUpdate(command) {
      if (pending) return;
      pending = command;
      commandError = '';
      renderUpdate();
      const previousEventCount = eventCount;
      try {
        const result = await invoke(command);
        if (eventCount === previousEventCount) {
          const next = result?.status ? result : await invoke('launcher_update_state');
          if (eventCount === previousEventCount) acceptUpdate(next);
        }
      } catch (error) {
        if (eventCount === previousEventCount || update.status !== 'error') {
          commandError = String(error);
        }
      } finally {
        pending = '';
        renderUpdate();
      }
    }

    action.addEventListener('click', () => {
      if (commandError || update?.status === 'error') {
        void runUpdate('launcher_update_check');
      } else if (update?.status === 'available') {
        void runUpdate('launcher_update_install');
      }
    });

    const previousEventCount = eventCount;
    const listen = window.__TAURI__.event?.listen;
    if (listen) {
      try {
        await listen('shift-launcher-update', ({payload}) => {
          eventCount++;
          acceptUpdate(payload);
        });
      } catch (error) {
        console.warn('Launcher update events unavailable', error);
      }
    }
    const initial = await invoke('launcher_update_state');
    if (eventCount === previousEventCount) acceptUpdate(initial);
    if (update.status === 'idle') await runUpdate('launcher_update_check');
  }

  const catalogData = window.shiftCatalogData = invoke('catalog_snapshot');
  // Begin local IPC now, then paint after the other deferred modules have registered their listeners.
  const listenersReady = new Promise(resolve => document.addEventListener('DOMContentLoaded', resolve, {once:true}));

  async function start() {
    status.textContent = it ? 'Caricamento del catalogo…' : 'Loading catalog…';
    const startTime = performance.now();
    const [catalog, firstSnapshot] = await Promise.all([catalogData, invoke('native_snapshot'), listenersReady]);
    for (const app of catalog) catalogById.set(app.id, app);
    const ids = new Set([...document.querySelectorAll('.catalog-grid [data-app]')].map(node => node.dataset.app));
    if (catalog.length !== ids.size || catalog.some(app => !ids.has(app.id))) throw Error('Native catalog differs from the local template');

    function poll(snapshot) {
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
      const interval = snapshot.install_checks_pending?.length && performance.now() - startTime < 2000 ? 200 : 1000;
      setTimeout(() => invoke('native_snapshot').then(poll).catch(error => {status.textContent=String(error);}), interval);
    }

    poll(firstSnapshot);
  }
  startLauncherUpdater().catch(error => console.warn('Launcher update unavailable', error));
  start().catch(error => { status.textContent=String(error); });
})();
