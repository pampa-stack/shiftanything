/* Promptica’s Doop comparison view. All coverage and source resolution run in Rust. */
(() => {
  const invoke = window.__TAURI__.core.invoke;
  const lang = document.documentElement.lang === 'it' ? 'it' : 'en';
  const labels = Object.fromEntries(Object.entries({
    tagline: ['Trova alternative open source per le funzioni che usi.', 'Find open-source alternatives for the features you use.'],
    software: ['Software a pagamento', 'Paid software'],
    search: ['Cerca, es. Photoshop', 'Search, e.g. Photoshop'],
    choose: ['Confronta', 'Compare'],
    browse: ['Elenco software A–Z', 'Software list A–Z'],
    noMatch: ['Nessuna corrispondenza. Premi Invio per il confronto manuale.', 'No matches. Press Enter to compare manually.'],
    features: ['Funzioni che usi', 'Features you use'],
    required: ['Indispensabile', 'Essential'],
    results: ['Alternative open source', 'Open-source alternatives'],
    candidate: ['Alternativa open source', 'Open-source alternative'],
    coverage: ['Copertura documentata', 'Documented coverage'],
    essentials: ['Indispensabili', 'Essentials'],
    clearGaps: ['Indispensabili coperti', 'Essentials covered'],
    noEssentials: ['Nessun indispensabile', 'No essentials selected'],
    missingEssential: ['indispensabile scoperto', 'essential uncovered'],
    missingEssentials: ['indispensabili scoperti', 'essentials uncovered'],
    unverified: ['Non verificato', 'Unverified'],
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
    noCandidates: ['Aggiungi dal catalogo un’app da confrontare.', 'Add an app from the catalog to compare.'],
    manual: ['Confronto manuale', 'Manual comparison'],
    knownFunction: ['Scegli una funzione', 'Choose a feature'],
    addFunction: ['Aggiungi funzione', 'Add feature'],
    customFunction: ['Scrivi una funzione', 'Enter a feature'],
    addApp: ['Aggiungi app', 'Add app'],
    selectApp: ['Scegli un’app del catalogo', 'Choose a catalog app'],
    remove: ['Rimuovi', 'Remove'],
    about: ['La copertura conta le funzioni selezionate supportate per intero. Fonti controllate il', 'Coverage counts selected features with full support. Sources checked on'],
    customNote: ['La funzione è stata aggiunta manualmente.', 'This feature was added manually.'],
    relatedNote: ['La funzione va verificata per questa app.', 'This feature needs verification for this app.'],
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
            <input id="sh-software" class="inp" maxlength="100" autocomplete="off"
              placeholder="${labels.search}" role="combobox" aria-autocomplete="list" aria-expanded="false"
              aria-controls="sh-software-options" data-software-name required>
            <button type="button" class="btn" tabindex="-1" aria-label="${labels.browse}" aria-expanded="false"
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
        <div class="sh-features" data-features></div>
        <div class="sh-manual" data-manual hidden>
          <form data-add-known><select class="inp" data-known-feature aria-label="${labels.knownFunction}" required></select><button type="submit" class="btn">${labels.addFunction}</button></form>
          <form data-add-feature><input class="inp" data-feature-name maxlength="110" aria-label="${labels.customFunction}"
            placeholder="${labels.customFunction}" required><button type="submit" class="btn">${labels.addFunction}</button></form>
        </div>
      </section>
      <section class="sh-wing" aria-labelledby="sh-results-title">
        <header><h2 id="sh-results-title">${labels.results}</h2><span data-comparison-name class="dim"></span></header>
        <div class="sh-scroll">
          <table data-results hidden>
            <thead class="mono"><tr>
              <th scope="col">${labels.candidate}</th><th scope="col">${labels.coverage}</th>
              <th scope="col">${labels.essentials}</th><th scope="col">${labels.proofs} / ${labels.app}</th>
            </tr></thead>
            <tbody data-candidates></tbody>
          </table>
        </div>
        <p class="sh-empty" data-sh-status role="status"></p>
        <p class="sh-announce" data-sh-announce role="status"></p>
        <div class="sh-manual" data-manual-apps hidden>
          <form data-add-app><select class="inp" data-catalog-app aria-label="${labels.selectApp}" required></select>
            <button type="submit" class="btn">${labels.addApp}</button></form>
        </div>
        <p class="sh-note" data-sh-note hidden>${labels.about} <time></time>.</p>
      </section>
    </div>`;
  catalog.after(root);

  const dialog = document.createElement('dialog');
  dialog.className = 'shifter-dialog';
  dialog.setAttribute('aria-labelledby', 'sh-dialog-title');
  dialog.innerHTML = `<div class="detail-toolbar"><button type="button" class="btn" data-sh-close>${labels.close}</button></div>
    <div class="sh-dialog-body" data-sh-content></div><p role="status" class="dim" data-source-status></p>`;
  document.body.append(dialog);

  const find = key => root.querySelector('[data-' + key + ']');
  const status = find('sh-status');
  const search = find('software-name');
  const picker = find('software-picker');
  const popup = find('software-popup');
  const options = find('software-options');
  const toggle = find('software-toggle');
  const results = find('results');
  const collator = new Intl.Collator(lang, { sensitivity: 'base' });
  const normalize = value => value.replace(/[™®℠]/g, '').normalize('NFKD').replace(/[^\p{L}\p{N}]/gu, '').toLowerCase();
  const activeTab = () => !root.hidden;
  const known = new Map();
  const evidenceByApp = new Map();
  let workspace;
  let profiles = [];
  let apps = [];
  let byId = new Map();
  let suggestions = [];
  let matches = [];
  let highlighted = -1;
  let startingName = '';
  let activeProfile;
  let features = [];
  let candidates = [];
  let customSequence = 0;
  let renderRevision = 0;
  let dialogTrigger;
  let icons;

  function node(tag, text, className) {
    const element = document.createElement(tag);
    if (text !== undefined) element.textContent = text;
    if (className) element.className = className;
    return element;
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

  function updateAppChoices() {
    selectOptions(find('catalog-app'), labels.selectApp, apps.filter(app => !candidates.includes(app.id))
      .map(app => [app.id, app.name]));
  }

  function updateFeatureChoices() {
    selectOptions(find('known-feature'), labels.knownFunction, [...known.values()]
      .filter(feature => !features.some(current => current.key === feature.key))
      .sort((a, b) => collator.compare(a.name[lang], b.name[lang]))
      .map(feature => [feature.key, feature.name[lang]]));
  }

  function choose(name) {
    const term = normalize(name);
    if (!term || !workspace) return;
    activeProfile = profiles.find(profile => [profile.name, ...profile.aliases].some(alias => normalize(alias) === term));
    startingName = activeProfile?.name ?? name.trim();
    search.value = startingName;
    features = (activeProfile?.features ?? []).map(feature => ({
      ...feature, key: activeProfile.id + ':' + feature.id, selected: true, essential: false,
    }));
    const documented = new Set(activeProfile?.candidates.map(candidate => candidate.app_id));
    const names = new Set(activeProfile ? [activeProfile.name, ...activeProfile.aliases].map(normalize) : [term]);
    candidates = apps.filter(app => documented.has(app.id) || app.alternatives.some(name => names.has(normalize(name))))
      .map(app => app.id);
    updateAppChoices();
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
      section.append(node('strong', feature.name[lang] + ' · ' + descriptor),
        node('span', proof?.note[lang] || (feature.source ? labels.relatedNote : labels.customNote), 'dim'));
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
      if (original?.getAttribute('src')?.startsWith('media/')) {
        const logo = original.cloneNode(true);
        logo.loading = 'eager';
        identity.append(logo);
      } else {
        const initials = node('span', [...app.name].slice(0, 2).join('').toUpperCase(), 'sh-initials');
        initials.setAttribute('aria-hidden', 'true');
        identity.append(initials);
      }
      identity.append(node('strong', app.name));
      name.append(identity);

      const coverage = node('td', undefined, 'sh-rate');
      if (score.coverage === null) coverage.textContent = labels.unverified;
      else coverage.append(node('strong', score.coverage + '%'), node('span', score.supported + '/' + selected.length, 'dim'));
      const gapCount = score.missing_required.length;
      const gapText = gapCount ? gapCount + ' ' + (gapCount === 1 ? labels.missingEssential : labels.missingEssentials)
        : required.length ? labels.clearGaps : labels.noEssentials;
      const gap = node('td', gapText, gapCount ? 'sh-gap' : 'dim');
      if (gapCount) gap.title = selected.filter(feature => score.missing_required.includes(activeProfile ? feature.id : feature.key))
        .map(feature => feature.name[lang]).join(' · ');

      const actions = node('div', undefined, 'sh-proof-actions');
      const proofs = button(labels.proofs, event => showProofs(app, selected, event.currentTarget));
      proofs.setAttribute('aria-label', labels.proofs + ' · ' + app.name);
      const appDetail = button(labels.app);
      appDetail.dataset.appDetail = app.id;
      appDetail.setAttribute('aria-label', labels.app + ' · ' + app.name);
      appDetail.disabled = !document.getElementById('app-detail-' + app.id);
      actions.append(proofs, appDetail);
      if (!activeProfile) {
        const remove = button(labels.remove, () => {
          candidates = candidates.filter(id => id !== app.id);
          updateAppChoices();
          find('catalog-app').focus();
          render();
        });
        remove.setAttribute('aria-label', labels.remove + ' · ' + app.name);
        actions.append(remove);
      }
      const actionCell = node('td');
      actionCell.append(actions);
      row.append(name, coverage, gap, actionCell);
      rows.append(row);
    }
  }

  async function render() {
    const revision = ++renderRevision;
    const selected = features.filter(feature => feature.selected);
    const manual = Boolean(startingName && !activeProfile);
    find('feature-count').textContent = features.length ? selected.length + ' / ' + features.length : '';
    find('comparison-name').textContent = startingName ? startingName + (manual ? ' · ' + labels.manual : '') : '';
    find('manual').hidden = !manual;
    find('manual-apps').hidden = !manual;
    drawFeatures();
    results.hidden = true;
    find('candidates').replaceChildren();
    find('sh-announce').textContent = '';

    if (!startingName || !selected.length || !candidates.length) {
      setStatus(!startingName ? labels.empty : !selected.length ? labels.nothing : labels.noCandidates);
      return;
    }
    setStatus(labels.loading);
    const featureId = feature => activeProfile ? feature.id : feature.key;
    const required = selected.filter(feature => feature.essential).map(featureId);
    const comparison = activeProfile
      ? { mode: 'profile', profile_id: activeProfile.id } : { mode: 'manual', app_ids: candidates };
    try {
      const scores = await invoke('compare_shifter', { comparison, selected: selected.map(featureId), required });
      if (revision !== renderRevision) return;
      drawRows(scores, selected, required);
      results.hidden = !scores.length;
      setStatus(scores.length ? '' : labels.noCandidates);
      find('sh-announce').textContent = scores.length ? labels.results + ' · ' + scores.length : '';
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

  function showOptions(query) {
    if (!workspace) return;
    const term = normalize(query);
    matches = suggestions.filter(software => software.aliases.some(alias => normalize(alias).includes(term)));
    highlighted = -1;
    search.removeAttribute('aria-activedescendant');
    options.replaceChildren(...matches.map(({ name }, index) => {
      const option = node('div', name, 'sh-option');
      option.id = 'sh-software-option-' + index;
      option.setAttribute('role', 'option');
      option.setAttribute('aria-selected', 'false');
      option.addEventListener('pointerdown', event => event.preventDefault());
      option.addEventListener('click', () => applySoftware(name));
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
  function applySoftware(name) {
    setPopup(false);
    choose(name);
  }

  toggle.addEventListener('click', () => {
    if (popup.hidden) openOptions();
    else setPopup(false);
    search.focus({ preventScroll: true });
  });
  search.addEventListener('click', () => { if (popup.hidden) openOptions(); });
  search.addEventListener('input', () => showOptions(search.value));
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
      if (!search.reportValidity()) return;
      const option = matches[highlighted] ?? (matches.length === 1 ? matches[0] : null);
      applySoftware(!popup.hidden && option ? option.name : search.value);
    }
  });

  function onSubmit(form, action) {
    find(form).addEventListener('submit', event => {
      event.preventDefault();
      action();
    });
  }

  onSubmit('software', () => applySoftware(search.value));
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
  onSubmit('add-app', () => {
    const id = find('catalog-app').value;
    if (!byId.has(id) || candidates.includes(id)) return;
    candidates.push(id);
    updateAppChoices();
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
    applySoftware(profile.name);
    return true;
  }

  async function loadWorkspace() {
    if (workspace || opening) return;
    opening = true;
    setStatus(labels.loading);
    try {
      const loaded = await invoke('shifter_workspace');
      profiles = loaded.dataset.profiles;
      apps = loaded.apps.sort((a, b) => collator.compare(a.name, b.name));
      byId = new Map(apps.map(app => [app.id, app]));
      icons = new Map([...catalog.querySelectorAll('.catalog-grid [data-app]')]
        .map(card => [card.dataset.app, card.querySelector('.product-icon')]));
      const names = new Map(loaded.dataset.suggested_software.map(name => [normalize(name), { name, aliases: [name] }]));
      for (const profile of profiles) {
        const { id, name, aliases } = profile;
        names.set(normalize(name), { name, aliases: [name, ...aliases] });
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
      suggestions = [...names.values()].sort((a, b) => collator.compare(a.name, b.name));
      const note = find('sh-note');
      const time = note.querySelector('time');
      time.dateTime = loaded.dataset.checked_on;
      time.textContent = loaded.dataset.checked_on;
      note.hidden = false;
      workspace = loaded;
      if (!selectRequestedProfile()) {
        if (profiles.length) choose(profiles[0].name);
        else setStatus(labels.empty);
      }
    } catch {
      setStatus(labels.loadError);
    } finally {
      opening = false;
    }
  }

  function close() {
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
