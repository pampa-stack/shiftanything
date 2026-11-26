/* Narrow read-only bridge. No arbitrary Rust command, path, URL opener or shell is exposed. */
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
  async function start() {
    status.textContent = `${note} · Rust…`;
    const catalog = await invoke('catalog_snapshot');
    const ids = new Set([...document.querySelectorAll('.catalog-grid [data-app]')].map(node => node.dataset.app));
    if (catalog.length !== ids.size || catalog.some(app => !ids.has(app.id))) throw Error('Native catalog differs from the local template');
    async function poll() {
    const snapshot = await invoke('native_snapshot');
    status.textContent = `${note} · Rust ${catalog.length} · ${snapshot.platform.os}/${snapshot.platform.arch} · ${Object.keys(snapshot.detected).length} ${it?'stati rilevati':'detected states'}`;
    setTimeout(() => poll().catch(error => {status.textContent=String(error);}),1000);
    }
    await poll();
  }
  start().catch(error => { status.textContent=String(error); });
})();
