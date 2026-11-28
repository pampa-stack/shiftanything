/* Promptica’s Doop comparison view. All coverage and source resolution run in Rust. */
(() => {
  const invoke = window.__TAURI__.core.invoke;
  const lang = document.documentElement.lang === 'it' ? 'it' : 'en';
  const labels = Object.fromEntries(Object.entries({
    tagline: ['Trova alternative open source per le funzioni che usi.', 'Find open-source alternatives for the features you use.'],
    software: ['Software a pagamento', 'Paid software'],
    search: ['Cerca, es. Photoshop', 'Search, e.g. Photoshop'],
    choose: ['Scegli software', 'Choose software'],
    browse: ['Elenco software A–Z', 'Software list A–Z'],
    noMatch: ['Nessun profilo pronto. Premi Invio per scegliere le funzioni a mano.', 'No prepared profile. Press Enter to choose features manually.'],
    features: ['Funzioni che usi', 'Features you use'],
    chooseFeatures: ['Spunta le funzioni che usi per aggiornare il confronto.', 'Check the features you use to update the comparison.'],
    manualFeatures: ['Scegli una funzione documentata per trovare alternative.', 'Choose a documented feature to find alternatives.'],
    selectAll: ['Seleziona tutte', 'Select all'],
    clearAll: ['Deseleziona tutte', 'Clear all'],
    required: ['Indispensabile', 'Essential'],
    results: ['Confronto con il catalogo', 'Catalog comparison'],
    candidate: ['Applicazione', 'App'],
    coverage: ['Copertura documentata', 'Documented coverage'],
    coverageInfo: ['Dettaglio delle funzioni', 'Feature coverage details'],
    essentials: ['Indispensabili', 'Essentials'],
    clearGaps: ['Tutti coperti', 'All covered'],
    missingEssential: ['funzione da coprire', 'feature to cover'],
    missingEssentials: ['funzioni da coprire', 'features to cover'],
    unverified: ['Da documentare', 'Needs a source'],
    supported: ['Supportata', 'Supported'],
    partial: ['Parziale', 'Partial'],
    unsupported: ['Assente nell’edizione gratuita', 'Unavailable in free edition'],
    proofs: ['Fonti', 'Sources'],
    source: ['Fonte dell’alternativa', 'Alternative source'],
    baseline: ['Fonte del software di partenza', 'Source for original software'],
    app: ['Scheda', 'App details'],
    close: ['Chiudi', 'Close'],
    nothing: ['Scegli almeno una funzione per calcolare la copertura.', 'Choose at least one feature to calculate coverage.'],
    empty: ['Cerca un software oppure scegli un confronto pronto.', 'Search for software or choose an existing comparison.'],
    noResults: ['Nessuna alternativa con supporto completo documentato per almeno una delle funzioni selezionate.', 'No alternatives have documented full support for any selected feature.'],
    noAppMatches: ['Nessuna app corrisponde alla ricerca.', 'No apps match your search.'],
    findApp: ['Cerca app nel confronto', 'Search apps in comparison'],
    appCount: ['app nel confronto', 'apps in comparison'],
    previousPage: ['Precedente', 'Previous'],
    nextPage: ['Successiva', 'Next'],
    resultPages: ['Pagine delle app', 'App result pages'],
    rangeOf: ['di', 'of'],
    manual: ['Confronto manuale', 'Manual comparison'],
    knownFunction: ['Scegli una funzione', 'Choose a feature'],
    addFunction: ['Aggiungi funzione', 'Add feature'],
    customFunction: ['Scrivi una funzione', 'Enter a feature'],
    about: ['La copertura conta le funzioni selezionate supportate per intero. Fonti controllate il', 'Coverage counts selected features with full support. Sources checked on'],
    customNote: ['La funzione è stata aggiunta manualmente.', 'This feature was added manually.'],
    loading: ['Caricamento del confronto…', 'Loading comparison…'],
    loadError: ['Impossibile caricare il confronto. Riprova dal tab SH*FTER.', 'Unable to load the comparison. Try opening SH*FTER again.'],
    compareError: ['Impossibile aggiornare il confronto.', 'Unable to update the comparison.'],
    sourceError: ['Impossibile aprire la fonte.', 'Unable to open the source.'],
    sourceSent: ['Fonte inviata al browser.', 'Source sent to the browser.'],
    duplicateFeature: ['La funzione è già presente.', 'This feature is already present.'],
    featureLimit: ['Puoi confrontare fino a 100 funzioni.', 'You can compare up to 100 features.'],
  }).map(([key, pair]) => [key, pair[lang === 'en' ? 1 : 0]]));

  const tab = document.querySelector('.bar [data-view="shifter"]');
  const catalog = document.querySelector('.body');
  const bar = tab?.closest('.bar');
  if (!tab || !catalog || !bar) return;

  const root = document.createElement('main');
  root.className = 'shifter';
  root.id = 'shifter-view';
  root.hidden = true;
  tab.setAttribute('aria-controls', root.id);
  root.setAttribute('aria-labelledby', 'shifter-title');
  root.innerHTML = `
    <header>
      <div><h1 id="shifter-title" tabindex="-1">SH*FTER</h1><p class="dim">${labels.tagline}</p></div>
      <form class="sh-search" data-software>
        <div class="sh-picker" data-software-picker>
          <label for="sh-software" class="mono">${labels.software}</label>
          <div class="sh-picker-field">
            <span class="sh-picker-brand" data-software-brand aria-hidden="true" hidden></span>
            <input id="sh-software" class="inp" maxlength="100" autocomplete="off"
              placeholder="${labels.search}" role="combobox" aria-autocomplete="list" aria-expanded="false"
              aria-controls="sh-software-options" data-software-name required>
            <button type="button" class="btn" aria-label="${labels.browse}" aria-expanded="false"
              aria-controls="sh-software-options" data-software-toggle>▾</button>
          </div>
          <div class="sh-options" data-software-popup hidden>
            <div id="sh-software-options" role="listbox" aria-label="${labels.browse}" data-software-options></div>
            <div class="sh-no-match" role="status" data-software-status hidden>${labels.noMatch}</div>
          </div>
        </div>
        <button type="submit" class="btn solid">${labels.choose}</button>
      </form>
    </header>
    <div class="sh-layout">
      <section class="sh-wing" aria-labelledby="sh-features-title">
        <header><h2 id="sh-features-title">${labels.features}</h2><span data-feature-count class="dim" aria-live="polite"></span></header>
        <div class="sh-feature-tools" data-feature-tools hidden>
          <span data-feature-help class="dim"></span><button type="button" class="btn" data-toggle-features></button>
        </div>
        <div class="sh-features" data-features></div>
        <div class="sh-manual" data-manual hidden>
          <span class="dim">${labels.manualFeatures}</span>
          <form data-add-known><select class="inp" data-known-feature aria-label="${labels.knownFunction}" required></select><button type="submit" class="btn">${labels.addFunction}</button></form>
          <form data-add-feature><input class="inp" data-feature-name maxlength="110" aria-label="${labels.customFunction}"
            placeholder="${labels.customFunction}" required><button type="submit" class="btn">${labels.addFunction}</button></form>
        </div>
      </section>
      <section class="sh-wing" aria-labelledby="sh-results-title">
        <header class="sh-results-heading">
          <h2 id="sh-results-title">${labels.results}</h2><span data-comparison-name class="dim"></span>
        </header>
        <div class="sh-result-tools" data-result-tools hidden>
          <input type="search" class="inp" data-app-search aria-label="${labels.findApp}"
            placeholder="${labels.findApp}" aria-controls="sh-results-table">
          <strong data-result-total class="mono"></strong>
          <div class="sh-pagination" data-result-pager role="group" aria-label="${labels.resultPages}">
            <button type="button" class="btn" data-result-previous aria-controls="sh-results-table">${labels.previousPage}</button>
            <span class="mono" data-result-range></span>
            <button type="button" class="btn" data-result-next aria-controls="sh-results-table">${labels.nextPage}</button>
          </div>
        </div>
        <p class="sh-empty" data-sh-status role="status"></p>
        <div class="sh-scroll">
          <table id="sh-results-table" data-results hidden>
            <thead class="mono"><tr>
              <th scope="col">${labels.candidate}</th><th scope="col">${labels.coverage}</th>
              <th scope="col" data-essentials-column hidden>${labels.essentials}</th><th scope="col">${labels.proofs} / ${labels.app}</th>
            </tr></thead>
            <tbody data-candidates></tbody>
          </table>
        </div>
        <p class="sh-announce" data-sh-announce role="status"></p>
        <footer class="sh-note" data-sh-note hidden>
          <span>${labels.about} <time></time>.</span>
        </footer>
      </section>
    </div>`;
  catalog.after(root);

  const dialog = document.createElement('dialog');
  dialog.className = 'shifter-dialog';
  dialog.setAttribute('aria-labelledby', 'sh-dialog-title');
  dialog.innerHTML = `<div class="detail-toolbar"><button type="button" class="btn" data-sh-close>${labels.close}</button></div>
    <div class="sh-dialog-body" data-sh-content></div><p role="status" class="dim" data-source-status></p>`;
  const coverageTooltip = document.createElement('aside');
  coverageTooltip.id = 'sh-coverage-tooltip';
  coverageTooltip.className = 'sh-coverage-tooltip';
  coverageTooltip.setAttribute('role', 'tooltip');
  coverageTooltip.hidden = true;
  document.body.append(dialog, coverageTooltip);

  const find = key => root.querySelector('[data-' + key + ']');
  const status = find('sh-status');
  const search = find('software-name');
  const picker = find('software-picker');
  const popup = find('software-popup');
  const options = find('software-options');
  const toggle = find('software-toggle');
  const results = find('results');
  const appSearch = find('app-search');
  const previousPage = find('result-previous');
  const nextPage = find('result-next');
  const pageSize = 12;
  const collator = new Intl.Collator(lang, { sensitivity: 'base' });
  const { format } = new Intl.NumberFormat(lang);
  const normalize = value => value.replace(/[™®℠]/g, '').normalize('NFKD').replace(/[^\p{L}\p{N}]/gu, '').toLowerCase();
  const brands = new Map(Object.entries({
    '1password': ['1Password'],
    adobe: ['Adobe Acrobat Pro', 'Adobe After Effects', 'Adobe Audition', 'Adobe Illustrator', 'Adobe InDesign',
      'Adobe Lightroom Classic', 'Adobe Media Encoder', 'Adobe Photoshop', 'Adobe Premiere Pro'],
    affinity: ['Affinity Publisher'],
    anthropic: ['Claude Code'],
    arq: ['Arq Backup'],
    asana: ['Asana'],
    atlassian: ['Confluence', 'Jira', 'Trello'],
    autodesk: ['AutoCAD', 'Autodesk Inventor', 'Revit'],
    avid: ['Avid Pro Tools'],
    backblaze: ['Backblaze Computer Backup'],
    'capture-one': ['Capture One'],
    clickup: ['ClickUp'],
    cursor: ['Cursor'],
    dassault: ['SolidWorks'],
    devon: ['DEVONthink'],
    doist: ['Todoist'],
    esri: ['ArcGIS Pro'],
    evernote: ['Evernote'],
    figma: ['Figma'],
    fireflies: ['Fireflies.ai'],
    freshbooks: ['FreshBooks'],
    goodnotes: ['Goodnotes'],
    google: ['Google Antigravity'],
    granola: ['Granola'],
    hubspot: ['HubSpot CRM'],
    'image-line': ['FL Studio'],
    linear: ['Linear'],
    loom: ['Loom'],
    maxon: ['Cinema 4D'],
    microsoft: ['Microsoft 365', 'Microsoft Excel', 'Microsoft OneNote', 'Microsoft Outlook', 'Microsoft PowerPoint',
      'Microsoft Project', 'Microsoft Teams', 'Microsoft Visio', 'Microsoft Word'],
    notion: ['Notion'],
    obdev: ['Little Snitch'],
    obsidian: ['Obsidian'],
    otter: ['Otter.ai'],
    salesforce: ['Salesforce'],
    shopify: ['Shopify'],
    sketch: ['Sketch'],
    slack: ['Slack'],
    teamviewer: ['TeamViewer', 'TeamViewer Remote', 'TeamViewer Classic'],
    trimble: ['SketchUp'],
    wispr: ['Wispr Flow'],
    zoom: ['Zoom'],
  }).flatMap(([brand, names]) => names.map(name => [normalize(name), brand])));
  const activeTab = () => !root.hidden;
  const known = new Map();
  const evidenceByApp = new Map();
  let workspace;
  let profiles = [];
  let byId = new Map();
  let suggestions = [];
  let matches = [];
  let highlighted = -1;
  let startingName = '';
  let activeProfile;
  let features = [];
  let comparisonResult;
  let resultPage = 0;
  let customSequence = 0;
  let renderRevision = 0;
  let dialogTrigger;
  let coverageTrigger;
  let coverageHideTimer;
  let icons;

  function node(tag, text, className) {
    const element = document.createElement(tag);
    if (text !== undefined) element.textContent = text;
    if (className) element.className = className;
    return element;
  }

  function softwareBrand(name) {
    const mark = node('span', undefined, 'sh-brand');
    mark.setAttribute('aria-hidden', 'true');
    const brand = brands.get(normalize(name));
    const initials = () => {
      mark.classList.add('sh-brand-initials');
      mark.textContent = [...name].slice(0, 2).join('').toUpperCase();
    };
    if (brand) {
      const logo = node('img');
      logo.alt = '';
      mark.dataset.brand = brand;
      logo.addEventListener('error', initials, { once: true });
      logo.src = 'shifter-brands/' + brand + '.png';
      mark.append(logo);
    } else {
      initials();
    }
    return mark;
  }

  function updatePickerBrand() {
    const mark = find('software-brand');
    mark.hidden = !startingName || search.value !== startingName;
    mark.replaceChildren(...(mark.hidden ? [] : [softwareBrand(startingName)]));
  }

  function button(label, click, className = 'btn') {
    const control = node('button', label, className);
    control.type = 'button';
    if (click) control.addEventListener('click', click);
    return control;
  }

  function setStatus(message) {
    status.textContent = message;
    status.hidden = !message;
  }

  function selectOptions(select, placeholder, choices) {
    select.replaceChildren(new Option(placeholder, '', true, true));
    select.options[0].disabled = true;
    for (const [id, name] of choices) select.add(new Option(name, id));
  }

  function updateFeatureChoices() {
    selectOptions(find('known-feature'), labels.knownFunction, [...known.values()]
      .filter(feature => !features.some(current => current.key === feature.key))
      .sort((a, b) => collator.compare(a.name[lang], b.name[lang]))
      .map(feature => [feature.key, feature.name[lang]]));
  }

  function choose(name, profile) {
    if (!workspace) return;
    const newName = profile?.name ?? name.trim();
    if (!normalize(newName)) return;
    const unchanged = activeProfile === profile && startingName === newName;
    startingName = newName;
    search.value = startingName;
    updatePickerBrand();
    if (unchanged) return;
    activeProfile = profile;
    features = (activeProfile?.features ?? []).map(feature => ({
      ...feature, key: activeProfile.id + ':' + feature.id, selected: false, essential: false,
    }));
    appSearch.value = '';
    resultPage = 0;
    updateFeatureChoices();
    render();
  }

  function sourceButton(label, source) {
    return button(label, async () => {
      const output = dialog.querySelector('[data-source-status]');
      output.textContent = '';
      try {
        await invoke('open_shifter_source', { source });
        output.textContent = labels.sourceSent;
      } catch {
        output.textContent = labels.sourceError;
      }
    }, 'sh-source');
  }

  function showProofs(app, selected, trigger) {
    const title = node('h2', app.name + ' · ' + labels.proofs);
    title.id = 'sh-dialog-title';
    const sections = selected.map(feature => {
      const proof = evidenceByApp.get(app.id)?.get(feature.key);
      const descriptor = labels[proof?.state ?? 'unverified'];
      const section = node('section', undefined, 'sh-proof');
      section.append(node('strong', feature.name[lang] + ' · ' + descriptor));
      const note = proof?.note[lang] || (!feature.source ? labels.customNote : '');
      if (note) section.append(node('span', note, 'dim'));
      const actions = node('div', undefined, 'sh-proof-actions');
      if (feature.source) actions.append(sourceButton(labels.baseline, { kind: 'baseline', feature_id: feature.key }));
      if (proof?.source) actions.append(sourceButton(labels.source, { kind: 'evidence', app_id: app.id, feature_id: feature.key }));
      if (actions.childElementCount) section.append(actions);
      return section;
    });
    dialog.querySelector('[data-sh-content]').replaceChildren(title, ...sections);
    dialog.querySelector('[data-source-status]').textContent = '';
    dialogTrigger = trigger;
    dialog.showModal();
  }

  function drawFeatures() {
    const list = find('features');
    const previous = document.activeElement?.dataset.shFocus;
    list.replaceChildren();
    for (const feature of features) {
      const row = node('div', undefined, 'sh-feature');
      const name = node('label');
      const selected = node('input');
      selected.type = 'checkbox';
      selected.checked = feature.selected;
      selected.dataset.shFocus = feature.key + '-check';
      selected.addEventListener('change', () => {
        feature.selected = selected.checked;
        if (!feature.selected) feature.essential = false;
        render();
      });
      name.append(selected, node('span', feature.name[lang]));
      const mark = button(labels.required, () => {
        feature.essential = !feature.essential;
        render();
      });
      mark.disabled = !feature.selected;
      mark.dataset.shFocus = feature.key + '-essential';
      mark.setAttribute('aria-pressed', String(feature.essential));
      mark.setAttribute('aria-label', labels.required + ' · ' + feature.name[lang]);
      row.append(name, mark);
      list.append(row);
    }
    if (previous) list.querySelector('[data-sh-focus="' + CSS.escape(previous) + '"]')?.focus({ preventScroll: true });
  }

  function hideCoverage() {
    clearTimeout(coverageHideTimer);
    coverageTrigger?.removeAttribute('aria-describedby');
    coverageTrigger = undefined;
    coverageTooltip.hidden = true;
  }

  function showCoverage(trigger, app, features) {
    clearTimeout(coverageHideTimer);
    if (coverageTrigger === trigger && !coverageTooltip.hidden) return;
    hideCoverage();
    coverageTrigger = trigger;
    const items = node('ul');
    for (const feature of features) {
      const state = evidenceByApp.get(app.id)?.get(feature.key)?.state ?? 'unverified';
      const supported = state === 'supported';
      const mark = node('span', supported ? '✓' : '×', supported ? 'is-covered' : 'is-missing');
      mark.setAttribute('aria-hidden', 'true');
      const description = node('span', feature.name[lang]);
      description.append(node('small', labels[state]));
      const item = node('li');
      item.append(mark, description);
      items.append(item);
    }
    coverageTooltip.replaceChildren(node('strong', app.name), items);
    coverageTooltip.hidden = false;
    trigger.setAttribute('aria-describedby', coverageTooltip.id);
    const anchor = trigger.getBoundingClientRect();
    const tooltip = coverageTooltip.getBoundingClientRect();
    const margin = 8;
    const left = Math.max(margin, Math.min(anchor.left, window.innerWidth - tooltip.width - margin));
    const top = anchor.bottom + tooltip.height + margin <= window.innerHeight
      ? anchor.bottom + margin : Math.max(margin, anchor.top - tooltip.height - margin);
    coverageTooltip.style.left = left + 'px';
    coverageTooltip.style.top = top + 'px';
  }

  function coverageInfo(app, selected) {
    const control = button('i', undefined, 'sh-coverage-info');
    control.setAttribute('aria-label', labels.coverageInfo + ' · ' + app.name);
    const show = () => showCoverage(control, app, selected);
    control.addEventListener('focus', show);
    control.addEventListener('click', show);
    control.addEventListener('pointerenter', show);
    control.addEventListener('blur', hideCoverage);
    control.addEventListener('pointerleave', () => {
      if (document.activeElement !== control) coverageHideTimer = setTimeout(hideCoverage, 160);
    });
    return control;
  }

  document.addEventListener('keydown', event => {
    if (event.key === 'Escape' && coverageTrigger) {
      hideCoverage();
      event.preventDefault();
    }
  }, true);
  document.addEventListener('pointerdown', event => {
    if (coverageTrigger && !coverageTrigger.contains(event.target) && !coverageTooltip.contains(event.target)) hideCoverage();
  });
  coverageTooltip.addEventListener('pointerenter', () => clearTimeout(coverageHideTimer));
  coverageTooltip.addEventListener('pointerleave', () => {
    if (document.activeElement !== coverageTrigger) coverageHideTimer = setTimeout(hideCoverage, 160);
  });
  find('results').parentElement.addEventListener('scroll', hideCoverage);
  window.addEventListener('resize', hideCoverage);

  function drawRows(scores, selected, required) {
    const rows = find('candidates');
    for (const score of scores) {
      const app = byId.get(score.app_id);
      if (!app) continue;
      const row = node('tr');
      row.dataset.candidate = app.id;
      const name = node('td');
      const identity = node('div', undefined, 'sh-identity');
      const original = icons.get(app.id);
      const image = original?.getAttribute('src') ?? '';
      if (image.startsWith('media/') || /^data:image\/(png|jpeg|webp);base64,/.test(image)) {
        const logo = original.cloneNode(true);
        logo.loading = 'eager';
        identity.append(logo);
      } else {
        const initials = node('span', [...app.name].slice(0, 2).join('').toUpperCase(), 'sh-initials');
        initials.setAttribute('aria-hidden', 'true');
        initials.dataset.shLogo = app.id;
        identity.append(initials);
      }
      identity.append(node('strong', app.name));
      name.append(identity);

      row.append(name);
      const coverage = node('td', undefined, 'sh-rate');
      coverage.append(node('strong', score.coverage + '%'), ' ', node('span', score.supported + '/' + selected.length, 'dim'), coverageInfo(app, selected));
      row.append(coverage);
      const source = evidenceByApp.get(app.id);
      const evidenceKey = feature => activeProfile ? feature.id : feature.key;
      if (required.length) {
        const gapCount = score.missing_required.length;
        const hasRequiredProof = selected.some(feature => required.includes(evidenceKey(feature)) && source?.has(feature.key));
        const gapText = !hasRequiredProof ? '—' : gapCount
          ? gapCount + ' ' + (gapCount === 1 ? labels.missingEssential : labels.missingEssentials) : labels.clearGaps;
        const gap = node('td', gapText, gapCount && hasRequiredProof ? 'sh-gap' : 'dim');
        if (gapCount) gap.title = selected.filter(feature => score.missing_required.includes(evidenceKey(feature)))
          .map(feature => feature.name[lang]).join(' · ');
        row.append(gap);
      }

      const actions = node('div', undefined, 'sh-result-actions');
      const appDetail = button(labels.app);
      appDetail.dataset.appDetail = app.id;
      appDetail.setAttribute('aria-label', labels.app + ' · ' + app.name);
      appDetail.disabled = !document.getElementById('app-detail-' + app.id);
      if (selected.some(feature => source?.has(feature.key))) {
        const proofs = button(labels.proofs, event => showProofs(app, selected, event.currentTarget));
        proofs.setAttribute('aria-label', labels.proofs + ' · ' + app.name);
        actions.append(proofs);
      }
      actions.append(appDetail);
      const actionCell = node('td');
      actionCell.append(actions);
      row.append(actionCell);
      rows.append(row);
    }
    rows.dispatchEvent(new Event('comparison-rows-rendered', { bubbles: true }));
  }

  function drawResultPage() {
    hideCoverage();
    if (!comparisonResult) return;
    const { scores, selected, required } = comparisonResult;
    const term = normalize(appSearch.value);
    const matched = term ? scores.filter(({ app_id }) => {
      const app = byId.get(app_id);
      return app && (normalize(app.name).includes(term) || normalize(app.id).includes(term));
    }) : scores;
    const lastPage = Math.max(0, Math.ceil(matched.length / pageSize) - 1);
    resultPage = Math.min(resultPage, lastPage);
    const start = resultPage * pageSize;
    const end = Math.min(start + pageSize, matched.length);
    const total = format(scores.length) + ' ' + labels.appCount;
    const range = matched.length ? format(start + 1) + '–' + format(end) + ' ' + labels.rangeOf + ' ' + format(matched.length) : '';
    find('result-total').textContent = total;
    find('essentials-column').hidden = required.length === 0;
    results.toggleAttribute('data-has-essentials', required.length > 0);
    find('result-tools').hidden = scores.length === 0;
    find('result-pager').hidden = matched.length === 0;
    find('result-range').textContent = range;
    const focusedPageButton = document.activeElement;
    previousPage.disabled = resultPage === 0;
    nextPage.disabled = resultPage === lastPage;
    if (focusedPageButton === previousPage && previousPage.disabled && !nextPage.disabled) nextPage.focus();
    if (focusedPageButton === nextPage && nextPage.disabled && !previousPage.disabled) previousPage.focus();
    find('candidates').replaceChildren();
    drawRows(matched.slice(start, end), selected, required);
    results.hidden = matched.length === 0;
    setStatus(!scores.length ? labels.noResults : !matched.length ? labels.noAppMatches : '');
    find('sh-announce').textContent = range ? total + ' · ' + range : '';
  }

  async function render() {
    hideCoverage();
    const revision = ++renderRevision;
    const selected = features.filter(feature => feature.selected);
    const manual = Boolean(startingName && !activeProfile);
    find('feature-count').textContent = features.length ? selected.length + ' / ' + features.length : '';
    const comparisonName = find('comparison-name');
    comparisonName.replaceChildren();
    if (startingName) {
      comparisonName.append(softwareBrand(startingName), node('span', startingName + (manual ? ' · ' + labels.manual : '')));
    }
    find('manual').hidden = !manual;
    find('feature-tools').hidden = features.length === 0;
    find('feature-help').textContent = labels.chooseFeatures;
    find('toggle-features').textContent = selected.length === features.length ? labels.clearAll : labels.selectAll;
    drawFeatures();
    comparisonResult = undefined;
    find('result-total').textContent = '';
    find('result-tools').hidden = true;
    results.hidden = true;
    find('candidates').replaceChildren();
    find('sh-announce').textContent = '';

    if (!startingName) {
      setStatus(labels.empty);
      return;
    }
    if (!selected.length) {
      setStatus(labels.nothing);
      return;
    }
    setStatus(labels.loading);
    const featureId = feature => activeProfile ? feature.id : feature.key;
    const required = selected.filter(feature => feature.essential).map(featureId);
    const comparison = activeProfile
      ? { mode: 'profile', profile_id: activeProfile.id } : { mode: 'manual', app_ids: [] };
    try {
      const scores = await invoke('compare_shifter', { comparison, selected: selected.map(featureId), required });
      if (revision !== renderRevision) return;
      comparisonResult = { scores, selected, required };
      drawResultPage();
    } catch {
      if (revision === renderRevision) setStatus(labels.compareError);
    }
  }

  function setPopup(open) {
    popup.hidden = !open;
    search.setAttribute('aria-expanded', String(open));
    toggle.setAttribute('aria-expanded', String(open));
    if (!open) {
      highlighted = -1;
      search.removeAttribute('aria-activedescendant');
    }
  }

  function matchingProfiles(query) {
    const term = normalize(query);
    return suggestions.filter(profile => [profile.name, ...profile.aliases].some(alias => normalize(alias).includes(term)));
  }

  function showOptions(query) {
    if (!workspace) return;
    matches = matchingProfiles(query);
    highlighted = -1;
    search.removeAttribute('aria-activedescendant');
    options.replaceChildren(...matches.map((profile, index) => {
      const { name } = profile;
      const option = node('div', undefined, 'sh-option');
      option.append(softwareBrand(name), node('span', name));
      option.id = 'sh-software-option-' + index;
      option.setAttribute('role', 'option');
      option.setAttribute('aria-selected', 'false');
      option.addEventListener('pointerdown', event => event.preventDefault());
      option.addEventListener('click', () => applySoftware(name, profile));
      return option;
    }));
    popup.scrollTop = 0;
    find('software-status').hidden = matches.length > 0;
    setPopup(true);
  }

  function highlight(index) {
    if (!matches.length) return;
    options.children[highlighted]?.setAttribute('aria-selected', 'false');
    highlighted = (index + matches.length) % matches.length;
    const option = options.children[highlighted];
    option.setAttribute('aria-selected', 'true');
    search.setAttribute('aria-activedescendant', option.id);
    option.scrollIntoView({ block: 'nearest' });
  }

  const openOptions = () => showOptions(search.value === startingName ? '' : search.value);
  function applySoftware(name, profile) {
    setPopup(false);
    choose(name, profile);
  }

  function submitSoftware() {
    if (!workspace || !search.reportValidity()) return;
    const name = search.value;
    const term = normalize(name);
    if (!term) return;
    const choices = matchingProfiles(name);
    const exact = choices.find(profile => normalize(profile.name) === term)
      ?? choices.find(profile => profile.aliases.some(alias => normalize(alias) === term));
    const profile = exact ?? (choices.length === 1 ? choices[0] : undefined);
    if (profile || choices.length === 0) {
      applySoftware(name, profile);
    } else {
      search.focus({ preventScroll: true });
      showOptions(name);
      highlight(0);
    }
  }

  toggle.addEventListener('click', () => {
    if (popup.hidden) openOptions();
    else setPopup(false);
    search.focus({ preventScroll: true });
  });
  search.addEventListener('click', () => { if (popup.hidden) openOptions(); });
  search.addEventListener('input', () => {
    updatePickerBrand();
    showOptions(search.value);
  });
  picker.addEventListener('focusout', event => { if (!picker.contains(event.relatedTarget)) setPopup(false); });
  search.addEventListener('keydown', event => {
    if (event.isComposing) return;
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      if (popup.hidden) openOptions();
      const next = event.key === 'ArrowDown' ? highlighted + 1 : highlighted < 0 ? matches.length - 1 : highlighted - 1;
      highlight(next);
    } else if (event.key === 'Escape' && !popup.hidden) {
      event.preventDefault();
      event.stopPropagation();
      setPopup(false);
    } else if (event.key === 'Enter') {
      event.preventDefault();
      const option = popup.hidden ? undefined : matches[highlighted];
      if (option) applySoftware(option.name, option);
      else submitSoftware();
    }
  });

  appSearch.addEventListener('input', () => {
    resultPage = 0;
    drawResultPage();
  });
  previousPage.addEventListener('click', () => {
    resultPage--;
    drawResultPage();
  });
  nextPage.addEventListener('click', () => {
    resultPage++;
    drawResultPage();
  });

  function onSubmit(form, action) {
    find(form).addEventListener('submit', event => {
      event.preventDefault();
      action();
    });
  }

  find('toggle-features').addEventListener('click', () => {
    const selected = features.some(feature => !feature.selected);
    for (const feature of features) {
      feature.selected = selected;
      if (!selected) feature.essential = false;
    }
    render();
  });
  onSubmit('software', submitSoftware);
  onSubmit('add-feature', () => {
    if (features.length >= 100) return setStatus(labels.featureLimit);
    const field = find('feature-name');
    const name = field.value.trim();
    if (!normalize(name) || features.some(feature => normalize(feature.name[lang]) === normalize(name))) {
      return setStatus(labels.duplicateFeature);
    }
    features.push({ key: 'custom:' + customSequence++, name: { [lang]: name }, selected: true, essential: false });
    field.value = '';
    render();
  });
  onSubmit('add-known', () => {
    if (features.length >= 100) return setStatus(labels.featureLimit);
    const feature = known.get(find('known-feature').value);
    if (!feature) return;
    features.push({ ...feature, selected: true, essential: false });
    updateFeatureChoices();
    render();
  });
  dialog.querySelector('[data-sh-close]').addEventListener('click', () => dialog.close());
  dialog.addEventListener('close', () => dialogTrigger?.focus());

  let opening = false;
  let requestedProfile;

  function selectRequestedProfile() {
    if (!workspace || !requestedProfile) return false;
    const profile = profiles.find(candidate => candidate.id === requestedProfile);
    requestedProfile = undefined;
    if (!profile) return false;
    applySoftware(profile.name, profile);
    return true;
  }

  async function loadWorkspace() {
    if (workspace || opening) return;
    opening = true;
    setStatus(labels.loading);
    try {
      const loaded = await invoke('shifter_workspace');
      profiles = loaded.dataset.profiles;
      byId = new Map(loaded.apps.map(app => [app.id, app]));
      icons = new Map([...catalog.querySelectorAll('.catalog-grid [data-app]')]
        .map(card => [card.dataset.app, card.querySelector('.product-icon')]));
      for (const profile of profiles) {
        const { id } = profile;
        for (const feature of profile.features) {
          const key = id + ':' + feature.id;
          known.set(key, { ...feature, key });
        }
        for (const candidate of profile.candidates) {
          const evidence = evidenceByApp.get(candidate.app_id) ?? new Map();
          for (const [feature, proof] of Object.entries(candidate.evidence)) evidence.set(id + ':' + feature, proof);
          evidenceByApp.set(candidate.app_id, evidence);
        }
      }
      suggestions = profiles.filter(profile => profile.features.length).sort((a, b) => collator.compare(a.name, b.name));
      const note = find('sh-note');
      const time = note.querySelector('time');
      time.dateTime = loaded.dataset.checked_on;
      time.textContent = loaded.dataset.checked_on;
      note.hidden = false;
      workspace = loaded;
      if (!selectRequestedProfile()) {
        setStatus(labels.empty);
        if (document.activeElement === search) openOptions();
      }
    } catch {
      setStatus(labels.loadError);
    } finally {
      opening = false;
    }
  }

  function close() {
    hideCoverage();
    root.hidden = true;
    bar.removeAttribute('data-shifter-active');
    setPopup(false);
  }

  tab.disabled = false;
  tab.removeAttribute('aria-disabled');
  tab.removeAttribute('data-proof-unavailable');
  tab.removeAttribute('title');
  tab.addEventListener('click', () => {
    root.hidden = false;
    catalog.hidden = true;
    const manage = document.querySelector('.native-apps');
    if (manage) manage.hidden = true;
    bar.setAttribute('data-shifter-active', '');
    for (const item of bar.querySelectorAll('.seg > *')) {
      item.classList.toggle('on', item === tab);
      item.setAttribute('aria-pressed', String(item === tab));
    }
    window.dispatchEvent(new CustomEvent('shiftanything:surface', { detail: 'shifter' }));
    root.querySelector('h1').focus();
    if (!selectRequestedProfile()) loadWorkspace();
  });
  bar.addEventListener('click', event => {
    const choice = event.target.closest('.seg button,[data-view="manage"]');
    if (activeTab() && choice && choice !== tab && !choice.disabled) close();
  });
  window.addEventListener('shiftanything:compare', event => {
    if (typeof event.detail !== 'string' || !/^[-a-z0-9]+$/.test(event.detail)) return;
    requestedProfile = event.detail;
    tab.click();
  });
  window.addEventListener('shiftanything:surface', event => {
    if (activeTab() && event.detail !== 'shifter') close();
  });
})();
