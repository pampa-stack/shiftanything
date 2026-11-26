/* Typed native operations only; all mutations require the Rust approval contract. */
(() => {
  const invoke = window.__TAURI__.core.invoke;
  const it = document.documentElement.lang === 'it';
  const status = document.querySelector('.status > span');
  window.addEventListener('error', event => {status.textContent = `Proof error: ${event.message}`;});
  for (const node of document.querySelector('.bar .seg').children) node.textContent = node.textContent.toUpperCase();
  const note = it ? 'Prova di sola lettura' : 'Read-only proof';
  for (const node of document.querySelectorAll('[data-view]')) {
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
    status.textContent = `${note} · ${link.href}`;
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
    const cards = [...document.querySelectorAll('.catalog-grid .cell:not([hidden])')];
    const report = {
      language: document.documentElement.lang,
      viewport: {x:0,y:0,width:innerWidth,height:innerHeight},
      header:rect(document.querySelector('.top')), toolbar:rect(document.querySelector('.bar')), rail:rect(document.querySelector('.rail')),
      navigation:[...document.querySelector('.bar .seg').children].map(node => node.textContent.trim()),
      cards:cards.map(cell => ({id:cell.dataset.app,rect:rect(cell),preview:rect(cell.querySelector('.preview-shot')),
        logo_loaded:cell.querySelector('.product-icon').naturalWidth > 1,
        preview_loaded:cell.querySelector('.preview-shot img').naturalWidth > 1})),
    };
    await invoke('record_visual_metrics', {report});
  }
  let debounce;
  function measureSoon() { clearTimeout(debounce); debounce=setTimeout(() => metrics().catch(error => {status.textContent=String(error);}),300); }
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
  let current;
  let requesting = false;
  const confirmation = document.createElement('dialog');
  confirmation.className = 'native-confirmation';
  confirmation.innerHTML = `<form method="dialog"><h2></h2><p></p><div class="native-decision"><button type="button" data-reject>${it?'Rifiuta':'Reject'}</button><button type="button" data-approve>${it?'Approva':'Approve'}</button></div></form>`;
  document.body.append(confirmation);
  const labels = it ? {install:'Installa',update:'Aggiorna',open:'Apri',remove:'Rimuovi',cancel:'Annulla download',update_all:'Aggiorna tutte'} : {install:'Install',update:'Update',open:'Open',remove:'Remove',cancel:'Cancel download',update_all:'Update all'};
  let pendingId;
  function render(snapshot) {
    current = snapshot;
    const inventory = new Map(snapshot.inventory.apps.map(app => [app.id,app]));
    for (const button of document.querySelectorAll('[data-native-app]')) {
      const app = inventory.get(button.dataset.nativeApp);
      const operation = button.dataset.nativeOperation || (app?.installed ? 'open' : 'install');
      button.textContent = labels[operation];
      const allowed = app?.detected && (operation === 'cancel' ? !!app.job : !app.job && ({install:!app.installed && app.installable,open:app.installed,remove:app.installed,update:app.update_available})[operation]);
      button.disabled = !snapshot.lifecycle_enabled || !allowed;
      button.title = app?.job || (allowed ? labels[operation] : it?'Azione non disponibile nello stato nativo corrente':'Action unavailable in the current native state');
    }
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
      const app = inventory.get(primary?.dataset.nativeApp);
      const output = detail.querySelector('.native-result');
      if (output) output.textContent = `${app?.installed_version || '—'} → ${app?.latest_version || '—'} · ${actionError || snapshot.progress[app?.id]?.phase || snapshot.inventory.results['app:'+app?.id] || ''}`;
    }
    if (snapshot.pending) {
      pendingId = snapshot.pending.id;
      confirmation.querySelector('h2').textContent = labels[snapshot.pending.operation];
      confirmation.querySelector('p').textContent = snapshot.pending.apps.join(', ') + (it?' · Azione nel profilo temporaneo.':' · Action in the temporary profile.');
      if (!confirmation.open) confirmation.showModal();
    } else { pendingId=undefined; if (confirmation.open) confirmation.close(); }
  }
  async function decide(approve) {
    if (!pendingId || requesting) return;
    requesting=true;
    try { await invoke('decide_native_action',{requestId:pendingId,approve}); render(await invoke('native_snapshot')); }
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
      await invoke('request_native_action',{operation:button.dataset.nativeOperation || (app?.installed?'open':'install'),apps:[button.dataset.nativeApp]});
      render(await invoke('native_snapshot'));
    } catch(error) {actionError=String(error);status.textContent=actionError;}
    finally {requesting=false;}
  },true);
  async function start() {
    status.textContent = `${note} · Rust…`;
    const catalog = await invoke('catalog_snapshot');
    const ids = new Set([...document.querySelectorAll('.catalog-grid [data-app]')].map(node => node.dataset.app));
    if (catalog.length !== ids.size || catalog.some(app => !ids.has(app.id))) throw Error('Native catalog differs from the local template');
    async function poll() {
    const snapshot = await invoke('native_snapshot');
    render(snapshot);
    status.textContent = `${snapshot.lifecycle_enabled?(it?'Profilo temporaneo':'Temporary profile'):note} · Rust ${catalog.length} · ${snapshot.platform.os}/${snapshot.platform.arch} · ${Object.keys(snapshot.detected).length} ${it?'stati rilevati':'detected states'} · ${actionError || Object.entries(snapshot.progress).map(([id,p])=>id+': '+p.phase).join(' · ') || Object.entries(snapshot.inventory.results).filter(([id])=>id.startsWith('app:')).map(([id,result])=>id.slice(4)+': '+result).slice(-1).join('')}`;
    setTimeout(() => poll().catch(error => {status.textContent=String(error);}),1000);
    }
    await poll();
  }
  start().catch(error => { status.textContent=String(error); });
})();
