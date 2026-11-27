(() => {
  const invoke = window.__TAURI__.core.invoke;
  const language = document.documentElement.lang;
  const it = language === 'it';
  const label = value => value?.[language] || value?.en || '';
  const normalize = value => value.normalize('NFD').replace(/\p{Diacritic}/gu, '').toLowerCase().replace(/[^a-z0-9]+/g, ' ').trim();
  const showEvent = 'shiftanything:surface';
  const catalogBody = document.querySelector('.body');
  const tabs = [...document.querySelectorAll('.bar .seg > *')];
  const findTab = tabs.find(tab => tab.dataset.view === 'find_apps');
  const kitsTab = tabs.find(tab => tab.dataset.view === 'kits');
  if (!findTab || !kitsTab) return;

  function node(tag, value, className) {
    const result = document.createElement(tag);
    if (value) result.textContent = value;
    if (className) result.className = className;
    return result;
  }

  function button(value, action, className = 'btn') {
    const result = node('button', value, className);
    result.type = 'button';
    if (action) result.addEventListener('click', action);
    return result;
  }

  function source(text, id, url) {
    const link = node('a', text, 'source-link');
    link.href = url;
    link.dataset.catalogLink = id;
    return link;
  }

  function details(id, value = it ? 'Scheda' : 'Details') {
    const result = button(value);
    result.dataset.appDetail = id;
    return result;
  }

  function panel(title) {
    const result = node('main', '', 'main discovery-panel');
    result.hidden = true;
    const heading = node('h1', title);
    heading.tabIndex = -1;
    result.append(heading);
    catalogBody.after(result);
    return result;
  }

  const find = panel(it ? 'Trova app' : 'Find apps');
  const kits = panel(it ? 'Kit di lavoro' : 'Workflow Kits');
  let snapshot;
  let renderKits;
  let system;
  let renderFind;
  let choseSystem = false;
  const activeView = () => !find.hidden ? 'find_apps' : !kits.hidden ? 'kits' : null;

  function close() { find.hidden = kits.hidden = true; }
  function show(tab, content) {
    close();
    catalogBody.hidden = true;
    document.querySelector('.native-apps').hidden = true;
    document.querySelector('[data-catalog-search]').disabled = true;
    for (const control of document.querySelectorAll('[data-layout]')) control.disabled = true;
    for (const choice of tabs) {
      choice.setAttribute('aria-pressed', String(choice === tab));
      choice.classList.toggle('on', choice === tab);
    }
    content.hidden = false;
    window.dispatchEvent(new CustomEvent(showEvent, {detail:tab.dataset.view}));
    content.querySelector('h1').focus();
  }

  for (const tab of tabs) {
    if (tab !== findTab && tab !== kitsTab) tab.addEventListener('click', close);
  }
  window.addEventListener(showEvent, event => {
    if (event.detail !== activeView()) close();
  });
  window.addEventListener('native-snapshot', event => {
    const previous = snapshot;
    snapshot = event.detail;
    const platform = snapshot.platform?.os?.toLowerCase();
    if (system && !choseSystem && platform && system.value !== platform && [...system.options].some(option => option.value === platform)) {
      system.value = platform;
      renderFind();
    }
    if (JSON.stringify(previous?.inventory.apps) !== JSON.stringify(snapshot.inventory.apps) || Boolean(previous?.pending) !== Boolean(snapshot.pending)) {
      renderKits?.updateSelection();
    }
  });

  function buildFind(intents, catalog) {
    const prompt = node('label', it ? 'Che cosa vuoi fare?' : 'What do you want to do?');
    const query = node('input');
    query.type = 'search';
    query.id = 'find-app-query';
    query.maxLength = 240;
    query.placeholder = label(intents[0]?.example);
    prompt.htmlFor = query.id;
    const controls = node('div', '', 'discovery-controls');
    const osLabel = node('label', it ? 'Sistema per app desktop' : 'Desktop operating system');
    system = node('select');
    system.id = 'find-app-system';
    osLabel.htmlFor = system.id;
    for (const [key, value] of [['all',it?'Tutti':'All'],['macos','macOS'],['windows','Windows'],['linux','Linux'],['freebsd','FreeBSD']]) {
      const option = node('option', value);
      option.value = key;
      system.append(option);
    }
    system.value = [...system.options].find(option => option.value === snapshot?.platform?.os?.toLowerCase())?.value || 'all';
    controls.append(prompt, query, osLabel, system);
    const examples = node('div', '', 'discovery-examples');
    const results = node('div', '', 'discovery-results');
    const browserResults = node('div', '', 'discovery-results');
    const browserSection = node('section', '', 'discovery-browser');
    browserSection.hidden = true;
    browserSection.append(node('h2', it?'Utilizzabili via browser':'Available in a browser'), browserResults);
    const feedback = node('p', '', 'discovery-feedback');
    feedback.setAttribute('role', 'status');
    find.append(controls, examples, feedback, results, browserSection);
    renderFind = () => {
      const phrase = normalize(query.value);
      const includes = term => (' ' + phrase).includes(' ' + term);
      const matched = phrase ? intents.filter(intent => !intent.exclude?.some(includes) && intent.match.some(group => group.every(includes))) : [];
      const seen = new Set();
      const rows = [];
      const browserRows = [];
      for (const intent of matched) for (const result of intent.apps) {
        const app = catalog.get(result.id);
        const os = system.value;
        if (!app || seen.has(app.id)) continue;
        const browser = app.delivery === 'self-hosted' || app.delivery === 'web';
        if (!browser && os !== 'all' && !app.platform_support?.[os] && !app.release?.packages?.some(pkg => pkg.platform === os)
          && !app.distribution?.some(channel => channel.platform === os)) continue;
        seen.add(app.id);
        const row = node('article', '', 'discovery-tile');
        const actions = node('div', '', 'discovery-actions');
        actions.append(details(app.id), source(it?'Fonte':'Source', app.id, result.source));
        row.append(node('h2', app.name), node('p', label(result.reason)));
        if (app.delivery === 'self-hosted') {
          row.append(node('p', it ? 'Da configurare sul proprio computer o server' : 'Set up on your own computer or server'));
        }
        row.append(actions);
        (browser ? browserRows : rows).push(row);
      }
      results.replaceChildren(...rows);
      browserResults.replaceChildren(...browserRows);
      browserSection.hidden = !browserRows.length;
      const count = rows.length + browserRows.length;
      feedback.textContent = !phrase ? '' : count ? `${count} ${it?'risultati':'results'}` : matched.length
        ? (it?'Nessun risultato documentato per questo sistema.':'No documented result for this system.')
        : (it?'Nessuna corrispondenza. Prova con un problema più specifico.':'No matches. Try a more specific problem.');
    };
    for (const intent of intents) examples.append(button(label(intent.example), () => { query.value = label(intent.example); query.focus(); renderFind(); }));
    query.addEventListener('input', renderFind);
    system.addEventListener('change', () => { choseSystem = true; renderFind(); });
  }

  function buildKits(work, catalog) {
    const select = node('select');
    select.id = 'work-kit-role';
    const role = node('label', it ? 'Ruolo' : 'Role');
    role.htmlFor = select.id;
    for (const kit of work.kits) {
      const option = node('option', label(kit.name));
      option.value = kit.id;
      select.append(option);
    }
    const description = node('p');
    const steps = node('div', '', 'discovery-results');
    const feedback = node('p', '', 'discovery-feedback');
    feedback.setAttribute('role', 'status');
    const compare = id => button(it?'Confronta':'Compare', () => window.dispatchEvent(new CustomEvent('shiftanything:compare', {detail:id})));
    let checked = [];
    let requesting = false;
    const request = button(it?'Installa selezione':'Install selected', async () => {
      if (request.disabled || requesting) return;
      const apps = [...new Set(checked.filter(input => input.checked && !input.disabled).map(input => input.value))];
      requesting = true;
      updateSelection();
      try {
        await invoke('request_native_action', {operation:'install',apps});
        feedback.textContent = it?'Controlla la richiesta di installazione.':'Review the installation request.';
      } catch (error) { feedback.textContent = String(error); }
      finally { requesting = false; updateSelection(); }
    }, 'btn solid');
    const controls = node('div', '', 'discovery-controls');
    controls.append(role, select, request);
    const ranking = node('section', '', 'discovery-rankings');
    ranking.append(node('h2', it?'Classifica':'Rankings'));
    function updateSelection() {
      const inventory = new Map(snapshot?.inventory.apps.map(app => [app.id, app]) || []);
      for (const input of checked) {
        const app = inventory.get(input.value);
        input.disabled = !(snapshot?.lifecycle_enabled && app?.detected && app.compatible && app.installable && !app.installed && !app.job);
      }
      request.disabled = requesting || Boolean(snapshot?.pending) || !checked.some(input => input.checked && !input.disabled);
    }
    function render() {
      const kit = work.kits.find(kit => kit.id === select.value);
      if (!kit) return;
      checked = [];
      description.textContent = label(kit.description);
      feedback.textContent = '';
      steps.replaceChildren(...kit.steps.map(step => {
        const card = node('article', '', 'discovery-tile');
        card.append(node('h2', label(step.name)), node('p', (it?'A pagamento · ':'Paid · ') + step.paid.join(', ')));
        if (step.compare) card.append(compare(step.compare));
        for (const option of step.options) {
          const app = catalog.get(option.app);
          if (!app) continue;
          const row = node('div', '', 'kit-choice');
          const toggle = node('input'); toggle.type = 'checkbox'; toggle.value = app.id;
          const name = node('label');
          name.append(toggle, node('strong', app.name));
          toggle.setAttribute('aria-label', (it?'Seleziona per installazione · ':'Select for installation · ') + app.name);
          checked.push(toggle);
          toggle.addEventListener('change', updateSelection);
          row.append(name, node('p', label(option.use)), details(app.id), source(it?'Fonte':'Source', app.id, option.source));
          card.append(row);
        }
        return card;
      }));
      updateSelection();
    }
    for (const sample of work.rankings) {
      const card = node('article', '', 'discovery-tile');
      card.append(node('h3', label(sample.name)), node('p', `${label(sample.metric)} · ${label(sample.scope)}`), source(sample.publisher, 'survey:'+sample.id, sample.source));
      for (const entry of sample.entries) {
        const row = node('div', '', 'kit-choice');
        row.append(node('strong', entry.paid), node('span', entry.value.toLocaleString(language)));
        for (const id of entry.apps) if (catalog.has(id)) {
          const app = catalog.get(id);
          const stars = app.stars?.count;
          row.append(details(id, app.name + (stars ? ' · ★ '+stars.toLocaleString(language) : '')));
        }
        if (entry.compare) row.append(compare(entry.compare));
        card.append(row);
      }
      ranking.append(card);
    }
    select.addEventListener('change', render);
    kits.append(controls, description, feedback, steps, ranking);
    render();
    renderKits = {updateSelection};
  }

  Promise.all([invoke('discovery_data'), invoke('catalog_snapshot')]).then(([data, entries]) => {
    const catalog = new Map(entries.map(app => [app.id, app]));
    buildFind(data.intents, catalog);
    buildKits(data.work, catalog);
    for (const [tab, content] of [[findTab,find],[kitsTab,kits]]) {
      tab.disabled = false;
      tab.removeAttribute('aria-disabled');
      tab.removeAttribute('data-proof-unavailable');
      tab.removeAttribute('title');
      tab.addEventListener('click', () => show(tab, content));
    }
  }).catch(error => {document.querySelector('.status > span').textContent = String(error);});
})();
