/* Installed apps and ongoing actions come from NativeBackend's inventory. */
(() => {
  const invoke = window.__TAURI__.core.invoke;
  const it = document.documentElement.lang === 'it';
  const catalogBody = document.querySelector('.body');
  const panel = document.createElement('main');
  panel.className = 'main native-apps';
  panel.hidden = true;
  panel.innerHTML = `<div class="mhead"><h1 tabindex="-1">${it?'Le mie app':'My apps'}</h1><div class="manage-tools"><button class="btn" data-native-check>${it?'Cerca aggiornamenti':'Check for updates'}</button><button class="btn solid" data-native-bulk>${it?'Aggiorna tutte':'Update all'}</button></div></div><p class="manage-status" role="status" aria-live="polite"></p><div class="manage-group" data-native-pending-group hidden><h2>${it?'Installazioni in corso':'Installations in progress'} · <span data-native-pending-count>0</span></h2><div data-native-installs></div></div><div class="manage-group"><h2>${it?'Aggiornamenti':'Updates'} · <span data-native-update-count>0</span></h2><div data-native-updates></div></div><div class="manage-group"><h2>${it?'Installate senza aggiornamenti':'Installed without updates'} · <span data-native-installed-count>0</span></h2><div data-native-installed></div></div>`;
  catalogBody.after(panel);
  document.body.append(document.querySelector('.catalog-detail'));
  let snapshot;
  let catalog;
  let key;
  let ready = [];
  function show(manage, target) {
    panel.hidden = !manage;
    catalogBody.hidden = manage;
    for (const node of document.querySelectorAll('.bar .seg > *')) {
      const active = manage ? node.dataset.view === 'manage' : node === target;
      node.setAttribute('aria-pressed', String(active));
      node.classList.toggle('on', active);
    }
    document.querySelector('[data-catalog-search]').disabled = manage;
    for (const button of document.querySelectorAll('[data-layout]')) button.disabled = manage;
    if (manage) {
      window.dispatchEvent(new CustomEvent('shiftanything:surface', {detail:'manage'}));
      panel.querySelector('h1').focus();
    }
  }
  for (let node of document.querySelectorAll('[data-view="manage"]')) {
    if (node.tagName !== 'BUTTON') {
      const button = document.createElement('button');
      button.type = 'button';
      button.dataset.view = 'manage';
      button.setAttribute('aria-pressed','false');
      button.textContent = node.textContent;
      node.replaceWith(button);
      node = button;
    }
    node.addEventListener('click', () => show(true));
  }
  for (const button of document.querySelectorAll('[data-catalog-home],[data-favorites-only]')) {
    button.addEventListener('click', () => show(false, button));
  }
  function text(tag, value, cls) {
    const node = document.createElement(tag);
    node.textContent = value;
    if (cls) node.className = cls;
    return node;
  }
  function action(app, operation, label) {
    const button = text('button', label, 'btn');
    button.type = 'button';
    button.dataset.nativeApp = app.id;
    button.dataset.nativeOperation = operation;
    button.disabled = !snapshot.lifecycle_enabled || (operation === 'cancel' ? !app.job || snapshot.progress[app.id]?.phase === 'cancelling' : !!app.job);
    return button;
  }
  function row(app) {
    const entry = catalog.find(item => item.id === app.id);
    const row = document.createElement('div');
    row.className = 'table-row';
    const identity = document.createElement('div');
    identity.className = 'nm';
    const original = [...document.querySelectorAll('.catalog-grid [data-app]')].find(item => item.dataset.app === app.id);
    const logo = original?.querySelector('.product-icon');
    if (logo) identity.append(logo.cloneNode(true));
    identity.append(text('strong', entry?.name || app.id));
    row.append(identity, text('span', app.installed_version || '—'), text('span', app.update_available ? (app.latest_version || (it?'Nuova revisione':'New revision')) : '—'));
    const activity = window.nativeAppActivity(snapshot, app.id);
    const state = text('span', activity.label || (app.installed ? (it?'Installata':'Installed') : (it?'In attesa':'Waiting')));
    state.title = activity.detail || '';
    state.className = 'native-app-result';
    row.append(state);
    const actions = document.createElement('div');
    actions.className = 'more-actions';
    if (app.installed && app.update_available) {
      const update = action(app, 'update', it?'Aggiorna':'Update');
      update.disabled ||= !app.installable || !app.compatible;
      actions.append(update);
    }
    if (app.installed) {
      const open = action(app, 'open', it?'Apri':'Open');
      open.disabled ||= app.can_open === false;
      actions.append(open);
    }
    const details = text('button', it?'Scheda':'Details', 'btn');
    details.type = 'button';
    details.dataset.appDetail = app.id;
    actions.append(details);
    if (app.installed) {
      const remove = action(app, 'remove', it?'Rimuovi':'Remove');
      remove.disabled ||= app.can_remove === false;
      actions.append(remove);
    }
    if (app.job) actions.append(action(app, 'cancel', it?'Annulla operazione':'Cancel operation'));
    row.append(actions);
    return row;
  }
  function paintRows(container, apps, emptyState) {
    const heading = document.createElement('div');
    heading.className = 'table-row table-labels mono';
    const labels = it ? ['App','Versione','Disponibile','Stato','Azioni'] : ['App','Version','Available','Status','Actions'];
    for (const label of labels) heading.append(text('span', label));
    container.replaceChildren(heading, ...apps.map(row));
    if (!apps.length && emptyState) container.append(text('p', emptyState));
  }
  function render() {
    if (!snapshot || !catalog) return;
    const apps = snapshot.inventory.apps.filter(app => (app.installed || app.job) && catalog.some(entry => entry.id === app.id));
    const installed = apps.filter(app => app.installed);
    const installing = apps.filter(app => !app.installed);
    const updates = installed.filter(app => app.update_available);
    const upToDate = installed.filter(app => !app.update_available);
    ready = updates.filter(app => app.detected && app.installable && app.compatible && !app.job);
    const detecting = snapshot.inventory.apps.some(app => !app.detected);
    const nextKey = JSON.stringify([snapshot.lifecycle_enabled,apps,snapshot.progress,snapshot.inventory.results,detecting]);
    if (nextKey !== key) {
      const focused = document.activeElement;
      const hadFocus = panel.contains(focused);
      key = nextKey;
      const emptyWhileDetecting = detecting ? (it?'Rilevamento in corso…':'Detecting apps…') : '';
      paintRows(panel.querySelector('[data-native-installs]'), installing);
      paintRows(panel.querySelector('[data-native-updates]'), updates, emptyWhileDetecting
        || (it?'Nessun aggiornamento disponibile rilevato.':'No available updates detected.'));
      paintRows(panel.querySelector('[data-native-installed]'), upToDate, emptyWhileDetecting
        || (updates.length
          ? (it?'Le app con aggiornamenti sono elencate sopra.':'Apps with available updates are listed above.')
          : (it?'Nessuna app installata nel profilo.':'No apps installed in this profile.')));
      panel.querySelector('[data-native-pending-group]').hidden = !installing.length;
      panel.querySelector('[data-native-pending-count]').textContent = installing.length;
      panel.querySelector('[data-native-update-count]').textContent = updates.length;
      panel.querySelector('[data-native-installed-count]').textContent = upToDate.length;
      if (hadFocus && !focused.isConnected && !panel.hidden) {
        const focusTarget = [...panel.querySelectorAll('button:enabled')].find(button =>
          (focused.dataset.appDetail && focused.dataset.appDetail === button.dataset.appDetail)
          || (focused.dataset.nativeApp && focused.dataset.nativeApp === button.dataset.nativeApp
            && focused.dataset.nativeOperation === button.dataset.nativeOperation));
        (focusTarget || panel.querySelector('h1')).focus();
      }
    }
    panel.querySelector('[data-native-bulk]').disabled = !snapshot.lifecycle_enabled || !ready.length || !!snapshot.pending;
    panel.querySelector('[data-native-check]').disabled = !snapshot.lifecycle_enabled;
  }
  panel.querySelector('[data-native-check]').addEventListener('click', async () => {
    const status = panel.querySelector('.manage-status');
    try {
      await invoke('check_native_updates');
      status.textContent = it?'Controllo richiesto al motore nativo.':'Check requested from the native backend.';
    } catch (error) { status.textContent = String(error); }
  });
  panel.querySelector('[data-native-bulk]').addEventListener('click', async () => {
    const status = panel.querySelector('.manage-status');
    status.textContent = '';
    try {
      await invoke('request_native_action',{operation:'update_all',apps:ready.map(app=>app.id)});
    } catch (error) { status.textContent = String(error); }
  });
  window.addEventListener('native-snapshot', event => {snapshot=event.detail;render();});
  invoke('catalog_snapshot').then(data => {catalog=data;render();}).catch(error => {panel.querySelector('.manage-status').textContent=String(error);});
})();
