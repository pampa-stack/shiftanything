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

  const icons = {
    ranking: 'M3 13V8M8 13V3M13 13V6',
    expand: 'M3 6l5 5 5-5',
  };
  // Commands (not filters) share the accent colour, an icon and their own place in the layout.
  function command(value, icon, action) {
    const result = button('', action, 'btn btn-command');
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('viewBox', '0 0 16 16');
    svg.setAttribute('aria-hidden', 'true');
    const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    path.setAttribute('d', icons[icon]);
    svg.append(path);
    const text = node('span', value);
    result.append(svg, text);
    result.label = text;
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

  function supportsPlatform(app, os) {
    return app.platform_support?.[os] || app.release?.packages?.some(pkg => pkg.platform === os)
      || app.distribution?.some(channel => channel.platform === os);
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
    if (JSON.stringify(previous?.inventory.apps) !== JSON.stringify(snapshot.inventory.apps)
      || JSON.stringify(previous?.install_checks_pending) !== JSON.stringify(snapshot.install_checks_pending)
      || Boolean(previous?.pending) !== Boolean(snapshot.pending)) {
      renderKits?.updateSelection();
    }
  });

  function buildFind(intents, catalog) {
    const platforms = [
      ['macos', 'macOS', 'brand/macos.svg'],
      ['windows', 'Windows', 'brand/windows.png'],
      ['linux', 'Linux', 'brand/linux.png'],
      ['freebsd', 'FreeBSD'],
    ];
    const compact = new Intl.NumberFormat(language, {notation:'compact', maximumFractionDigits:1});
    const fullNumber = new Intl.NumberFormat(language);
    const content = node('div', '', 'find-content');
    find.classList.add('find-panel');
    content.append(find.firstElementChild);
    find.append(content);
    const prompt = node('label', it ? 'Cosa vuoi fare?' : 'What do you want to do?');
    const query = node('input');
    query.type = 'search';
    query.id = 'find-app-query';
    query.maxLength = 240;
    query.placeholder = label(intents[0]?.example);
    prompt.htmlFor = query.id;
    const controls = node('div', '', 'discovery-controls');
    const osLabel = node('label', it ? 'Sistema operativo' : 'Operating system');
    system = node('select');
    system.id = 'find-app-system';
    osLabel.htmlFor = system.id;
    for (const [key, value] of [['all',it?'Tutti':'All'], ...platforms]) {
      const option = node('option', value);
      option.value = key;
      system.append(option);
    }
    system.value = [...system.options].find(option => option.value === snapshot?.platform?.os?.toLowerCase())?.value || 'all';
    prompt.append(query);
    osLabel.append(system);
    controls.append(prompt, osLabel);
    const examples = node('div', '', 'discovery-examples');
    const results = node('div', '', 'discovery-results');
    const browserResults = node('div', '', 'discovery-results');
    const browserSection = node('section', '', 'discovery-browser');
    browserSection.hidden = true;
    browserSection.append(node('h2', it?'Utilizzabili via browser':'Available in a browser'), browserResults);
    const feedback = node('p', '', 'discovery-feedback');
    feedback.setAttribute('role', 'status');
    content.append(controls, examples, feedback, results, browserSection);

    function resultRow(app, result, browser) {
      const row = node('article', '', 'discovery-tile');
      const logo = node('span', [...app.name].slice(0, 2).join('').toUpperCase(), 'find-logo find-initials');
      logo.setAttribute('aria-hidden', 'true');
      logo.dataset.findLogo = app.id;
      const summary = node('div', '', 'find-app-summary');
      const byline = node('div', '', 'find-byline');
      byline.append(node('span', app.author));
      if (app.stars?.count) {
        const stars = node('span', `★ ${compact.format(app.stars.count)}`, 'star-count');
        stars.title = `${fullNumber.format(app.stars.count)} · ${it?'Stelle GitHub':'GitHub stars'}`;
        byline.append(stars);
      } else if (app.popularity?.kind === 'flathub_installs') {
        byline.append(node('span', `${compact.format(app.popularity.count)} Flathub`, 'star-count'));
      }
      if (app.repo?.startsWith('https://github.com/')) byline.append(source('GitHub', app.id, app.repo));
      const description = node('p', label(app.description), 'find-description');
      const reason = node('p', label(result.reason), 'find-reason');
      summary.append(node('h2', app.name), byline, description);
      if (app.delivery === 'self-hosted') {
        summary.append(node('p', it ? 'Da configurare sul proprio computer o server' : 'Set up on your own computer or server'));
      }
      const side = node('div', '', 'find-result-side');
      const systems = node('div', '', 'platform-badges');
      if (!browser) {
        for (const [key, name, image] of platforms) {
          if (!supportsPlatform(app, key)) continue;
          const badge = node('span', image ? '' : name, 'bdg');
          badge.title = name;
          if (image) {
            const mark = node('img');
            mark.src = image;
            mark.alt = name;
            badge.append(mark);
          }
          systems.append(badge);
        }
      }
      const actions = node('div', '', 'discovery-actions');
      const detail = details(app.id, it?'Vedi scheda':'View app');
      detail.setAttribute('aria-label', `${detail.textContent} · ${app.name}`);
      const evidence = source(it?'Fonte':'Source', app.id, result.source);
      evidence.setAttribute('aria-label', `${evidence.textContent} · ${app.name}`);
      actions.append(evidence, detail);
      if (systems.childElementCount) side.append(systems);
      side.append(actions);
      row.append(logo, summary, reason, side);
      return row;
    }

    const moreExamples = node('div', '', 'find-more-examples');
    moreExamples.id = 'find-more-examples';
    moreExamples.hidden = true;
    const more = command(it ? 'Altri esempi' : 'More examples', 'expand', () => {
      moreExamples.hidden = !moreExamples.hidden;
      more.setAttribute('aria-expanded', String(!moreExamples.hidden));
      more.label.textContent = moreExamples.hidden ? (it ? 'Altri esempi' : 'More examples') : (it ? 'Meno esempi' : 'Fewer examples');
    });
    more.setAttribute('aria-controls', moreExamples.id);
    more.setAttribute('aria-expanded', 'false');
    const chips = intents.map((intent, index) => {
      const example = label(intent.example);
      const chip = button(example, () => { query.value = example; query.focus(); renderFind(); });
      chip.setAttribute('aria-pressed', 'false');
      (index < 7 ? examples : moreExamples).append(chip);
      return {phrase:normalize(example), chip};
    });
    if (moreExamples.childElementCount) examples.append(more, moreExamples);
    renderFind = () => {
      const phrase = normalize(query.value);
      for (const example of chips) example.chip.setAttribute('aria-pressed', String(phrase === example.phrase));
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
        if (!browser && os !== 'all' && !supportsPlatform(app, os)) continue;
        seen.add(app.id);
        (browser ? browserRows : rows).push(resultRow(app, result, browser));
      }
      results.replaceChildren(...rows);
      browserResults.replaceChildren(...browserRows);
      browserSection.hidden = !browserRows.length;
      for (const group of [results, browserResults]) group.dispatchEvent(new Event('find-logos-rendered', {bubbles:true}));
      const count = rows.length + browserRows.length;
      const resultLabel = count === 1 ? (it ? 'risultato' : 'result') : (it ? 'risultati' : 'results');
      feedback.textContent = !phrase ? '' : count ? `${count} ${resultLabel}` : matched.length
        ? (it?'Nessun risultato documentato per questo sistema.':'No documented result for this system.')
        : (it?'Nessuna corrispondenza. Prova con un problema più specifico.':'No matches. Try a more specific problem.');
    };
    query.addEventListener('input', renderFind);
    system.addEventListener('change', () => { choseSystem = true; renderFind(); });
  }

  function buildKits(work, catalog) {
    const phaseHeading = it ? 'Fase' : 'Stage';
    const paidHeading = it ? 'Software con piani a pagamento' : 'Software with paid plans';
    const freeHeading = it ? 'Alternative open source' : 'Open-source alternatives';
    const rankingHeading = it ? 'Classifica' : 'Rankings';
    const compact = new Intl.NumberFormat(language, {notation:'compact', maximumFractionDigits:1});
    const percentage = new Intl.NumberFormat(language, {style:'percent', maximumFractionDigits:1});
    const detailLabel = it ? 'Scheda' : 'App details';
    kits.classList.add('kits-panel');
    const heading = node('h2');
    const description = node('p', '', 'kit-intro');
    const steps = node('div', '', 'kit-table-wrap');
    const feedback = node('p', '', 'discovery-feedback');
    feedback.setAttribute('role', 'status');
    const picked = node('span');
    picked.setAttribute('aria-live', 'polite');
    const compare = id => button(it?'Confronta in SH*FTER':'Compare in SH*FTER', () => window.dispatchEvent(new CustomEvent('shiftanything:compare', {detail:id})));
    let choices = [];
    let currentKit;
    let requesting = false;
    const request = button(it?'Rivedi installazione':'Review install', async () => {
      if (request.disabled || requesting) return;
      const apps = selectedApps();
      requesting = true;
      updateSelection();
      try {
        await invoke('request_native_action', {operation:'install',apps});
        feedback.textContent = it?'Controlla la richiesta di installazione.':'Review the installation request.';
      } catch (error) { feedback.textContent = String(error); }
      finally { requesting = false; updateSelection(); }
    }, 'btn solid');
    const plan = node('div', '', 'kit-plan');
    plan.append(picked, request);
    const header = node('div', '', 'kit-heading');
    header.append(heading, plan);
    const rolePanel = node('section');
    rolePanel.append(header, description, feedback, steps);
    const rankings = node('section');
    const studies = node('div', '', 'kit-tabs');
    const rankingTable = node('div', '', 'kit-table-wrap');
    rankings.append(studies, rankingTable);

    function selectedApps() {
      return [...new Set(choices.map(choice => choice.toggle).filter(toggle => toggle.checked && !toggle.disabled).map(toggle => toggle.value))];
    }
    function updateSelection() {
      const inventory = new Map(snapshot?.inventory.apps.map(app => [app.id, app]) || []);
      const platform = snapshot?.platform?.os?.toLowerCase();
      for (const {app, toggle, status} of choices) {
        const native = inventory.get(app.id);
        const available = Boolean(snapshot?.lifecycle_enabled && native?.detected && native.compatible && native.installable && !native.installed && !native.job);
        toggle.disabled = toggle.hidden = !available;
        if (!available) toggle.checked = false;
        status.classList.toggle('acc', available);
        if (native?.job) status.textContent = it ? 'Operazione in corso' : 'In progress';
        else if (native?.installed) status.textContent = it ? 'Installata' : 'Installed';
        else if (available) status.textContent = it ? 'Installabile' : 'Ready to install';
        else if (app.delivery === 'self-hosted') status.textContent = 'Self-hosted';
        else if (app.delivery === 'web') status.textContent = it ? 'Via browser' : 'In browser';
        else if (platform && app.platform_scope_complete && !supportsPlatform(app, platform)) status.textContent = it ? '🔒 Non supportata' : '🔒 Unsupported';
        else if (snapshot?.install_checks_pending?.includes(app.id)) status.textContent = it ? 'Controllo pacchetto' : 'Checking package';
        else status.textContent = it ? 'Download dalla fonte' : 'Download from source';
      }
      const count = selectedApps().length;
      picked.textContent = `${it ? 'App selezionate' : 'Selected apps'} (${count})`;
      request.disabled = requesting || Boolean(snapshot?.pending) || !count;
    }

    function table(headers, rows) {
      const element = node('table', '', 'kit-table');
      const headerRow = element.createTHead().insertRow();
      for (const text of headers) {
        const title = node('th', text);
        title.scope = 'col';
        headerRow.append(title);
      }
      const body = element.createTBody();
      for (const cells of rows) {
        const row = body.insertRow();
        row.append(...cells);
      }
      return element;
    }
    function summary(app) {
      const identity = node('span', '', 'kit-app-identity');
      const logo = node('span', [...app.name].slice(0, 2).join('').toUpperCase(), 'kit-logo kit-initials');
      logo.setAttribute('aria-hidden', 'true');
      logo.dataset.kitLogo = app.id;
      identity.append(logo, node('strong', app.name));
      if (app.stars?.count) identity.append(node('span', `★ ${compact.format(app.stars.count)}`, 'kit-popularity'));
      else if (app.popularity?.kind === 'flathub_installs') identity.append(node('span', `${compact.format(app.popularity.count)} Flathub`, 'kit-popularity'));
      return identity;
    }
    function appDetails(app) {
      const detail = details(app.id, detailLabel);
      detail.setAttribute('aria-label', `${detailLabel} · ${app.name}`);
      return detail;
    }
    function optionRow(app, option) {
      const row = node('div', '', 'kit-app');
      const main = node('div', '', 'kit-app-main');
      main.append(summary(app), node('span', label(option.use), 'kit-use'));
      const status = node('span', '', 'bdg');
      const toggle = node('input');
      toggle.type = 'checkbox';
      toggle.value = app.id;
      toggle.setAttribute('aria-label', `${it?'Seleziona per installazione':'Select for installation'} · ${app.name}`);
      toggle.addEventListener('change', updateSelection);
      choices.push({app, toggle, status});
      const actions = node('div', '', 'kit-app-actions');
      const proof = source(it?'Funzioni documentate':'Documented features', app.id, option.source);
      proof.className = 'btn';
      proof.setAttribute('aria-label', `${proof.textContent} · ${app.name}`);
      actions.append(status, toggle, proof, appDetails(app));
      row.append(main, actions);
      return row;
    }

    function renderKit(kit) {
      if (currentKit === kit.id) return;
      currentKit = kit.id;
      choices = [];
      heading.textContent = label(kit.name);
      description.textContent = label(kit.description);
      feedback.textContent = '';
      const rows = kit.steps.map(step => {
        const phase = node('th', label(step.name));
        phase.scope = 'row';
        const paid = node('td', step.paid.join(' · '));
        if (step.compare) paid.append(compare(step.compare));
        const alternatives = node('td');
        const options = node('div', '', 'kit-apps');
        for (const option of step.options) {
          const app = catalog.get(option.app);
          if (app) options.append(optionRow(app, option));
        }
        alternatives.append(options);
        return [phase, paid, alternatives];
      });
      steps.replaceChildren(table([phaseHeading, paidHeading, freeHeading], rows));
      updateSelection();
      steps.dispatchEvent(new CustomEvent('kit-logos-rendered', {bubbles:true}));
    }
    function renderRanking(sample) {
      const intro = node('p', label(sample.scope), 'kit-intro');
      intro.append(' ', source(sample.publisher, 'survey:'+sample.id, sample.source));
      const rows = sample.entries.map(entry => {
        const paid = node('th', entry.paid);
        paid.scope = 'row';
        if (entry.compare) paid.append(compare(entry.compare));
        const value = node('td', percentage.format(entry.value / 100), 'kit-metric');
        const alternatives = node('td');
        const options = node('div', '', 'kit-rank-apps');
        for (const id of entry.apps) {
          const app = catalog.get(id);
          if (!app) continue;
          const item = node('span');
          item.append(summary(app), appDetails(app));
          options.append(item);
        }
        alternatives.append(options);
        return [paid, value, alternatives];
      });
      rankingTable.replaceChildren(intro, table([paidHeading, label(sample.metric), freeHeading], rows));
      rankingTable.dispatchEvent(new CustomEvent('kit-logos-rendered', {bubbles:true}));
    }
    function addTab(group, name, activate) {
      const tab = button(name, () => {
        for (const item of group.children) item.setAttribute('aria-pressed', String(item === tab));
        activate();
      });
      tab.setAttribute('aria-pressed', 'false');
      group.append(tab);
      return tab;
    }
    const roles = node('div', '', 'kit-tabs');
    const roleTabs = work.kits.map(kit => {
      const tab = addTab(roles, label(kit.name), () => {
        ranking.setAttribute('aria-pressed', 'false');
        rankings.hidden = true;
        rolePanel.hidden = false;
        renderKit(kit);
      });
      const searchable = [kit.name, kit.description, ...kit.steps.map(step => step.name)].flatMap(value => [value.it, value.en]);
      return {tab, fields:searchable.map(value => ' ' + normalize(value))};
    });
    const ranking = command(rankingHeading, 'ranking', () => {
      for (const item of roles.children) item.setAttribute('aria-pressed', 'false');
      ranking.setAttribute('aria-pressed', 'true');
      rolePanel.hidden = true;
      rankings.hidden = false;
    });
    ranking.setAttribute('aria-pressed', 'false');
    for (const sample of work.rankings) addTab(studies, label(sample.name), () => renderRanking(sample));
    const kitSearch = node('input');
    kitSearch.type = 'search';
    kitSearch.id = 'kit-search';
    kitSearch.maxLength = 100;
    kitSearch.setAttribute('aria-controls', 'kit-roles');
    roles.id = 'kit-roles';
    const kitSearchLabel = node('label', it?'Cerca per ruolo o attività':'Search by role or activity');
    kitSearchLabel.htmlFor = kitSearch.id;
    kitSearchLabel.append(kitSearch);
    const kitControls = node('div', '', 'discovery-controls kit-controls');
    kitControls.append(kitSearchLabel, ranking);
    const noKits = node('p', it?'Nessun kit corrispondente.':'No matching kits.', 'discovery-feedback');
    noKits.setAttribute('role', 'status');
    noKits.hidden = true;
    kitSearch.addEventListener('input', () => {
      const phrase = ' ' + normalize(kitSearch.value);
      for (const {tab, fields} of roleTabs) tab.hidden = !fields.some(field => field.includes(phrase));
      const firstMatch = roleTabs.find(({tab}) => !tab.hidden);
      noKits.hidden = Boolean(firstMatch);
      if (!rankings.hidden) return;
      const selected = roleTabs.find(({tab}) => tab.getAttribute('aria-pressed') === 'true');
      if (selected?.tab.hidden && firstMatch) firstMatch.tab.click();
      else rolePanel.hidden = !firstMatch;
    });
    kits.append(kitControls, roles, noKits, rolePanel, rankings);
    roles.firstElementChild.click();
    studies.firstElementChild?.click();
    renderKits = {updateSelection};
  }

  Promise.all([invoke('discovery_data'), window.shiftCatalogData]).then(([data, entries]) => {
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
