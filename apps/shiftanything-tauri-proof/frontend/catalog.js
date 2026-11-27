(() => {
  const catalog = document.querySelector('[data-catalog]');
  let favorites = new Set();
  const detail = document.querySelector('.catalog-detail');
  const detailContent = detail?.querySelector('[data-app-content]');
  function updateHearts() {
    for (const heart of document.querySelectorAll('[data-favorite-app]')) {
      const active=favorites.has(heart.dataset.favoriteApp);
      heart.setAttribute('aria-pressed',active);
      const name=heart.getAttribute('aria-label').split(' · ').slice(1).join(' · ');
      const label=document.documentElement.lang==='it' ? (active?'Rimuovi dai preferiti':'Salva nei preferiti') : (active?'Remove from favorites':'Save to favorites');
      heart.setAttribute('aria-label',label+' · '+name);
      heart.title=label+' · '+name;
    }
  }
  document.addEventListener('error', event => {
    const img = event.target;
    if (!(img instanceof HTMLImageElement)) return;
    if (img.dataset.mediaOrigin) {
      const origin = img.dataset.mediaOrigin;
      delete img.dataset.mediaOrigin;
      img.src = origin;
      return;
    }
    const preview = img.closest('[data-preview-error]');
    if (preview) preview.classList.add('preview-error');
    else if (img.matches('.thumbs img')) img.classList.add('media-failed');
    else if (img.matches('.product-icon')) {
      const name = img.closest('.cell,.catalog-row,.detail-head')?.querySelector('.c-name,strong,h1')?.textContent;
      if (!name) return;
      const fallback = document.createElement('span');
      fallback.className = img.className + ' logo-fallback';
      fallback.setAttribute('aria-hidden', 'true');
      fallback.textContent = [...name.trim()].slice(0, 2).join('').toUpperCase();
      img.replaceWith(fallback);
    }
  }, true);
  updateHearts();
  if (detail) {
    document.addEventListener('click', event => {
      const button = event.target.closest('[data-app-detail]');
      if (!button) return;
      const template = document.getElementById(`app-detail-${button.dataset.appDetail}`);
      if (!template) return;
      detailContent.replaceChildren(template.content.cloneNode(true));
      updateHearts();
      detail.showModal();
    });
    detail.addEventListener('click', event => {
      if (event.target.closest('[data-close-detail]')) {
        detail.close();
        return;
      }
      const shot = event.target.closest('[data-shot]');
      if (!shot) return;
      const preview = detailContent.querySelector('.detail-shot');
      const img = preview.querySelector('img');
      preview.classList.remove('preview-error');
      if (shot.dataset.shot !== shot.dataset.shotOrigin) img.dataset.mediaOrigin = shot.dataset.shotOrigin;
      else delete img.dataset.mediaOrigin;
      img.src = shot.dataset.shot;
      for (const item of detailContent.querySelectorAll('[data-shot]')) item.setAttribute('aria-pressed', item === shot);
    });
    detail.addEventListener('close', () => detailContent.replaceChildren());
  }
  document.addEventListener('click', event => {
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
  if (!catalog) return;

  const search = document.querySelector('[data-catalog-search]');
  const select = key => catalog.querySelector(`[data-filter="${key}"]`);
  const provider = select('provider');
  const category = select('category');
  const sort = select('sort');
  const initialLevel = catalog.dataset.level;
  let level = initialLevel;
  const levelButtons = [...catalog.querySelectorAll('[data-catalog-level]')];
  const categoryButtons = [...document.querySelectorAll('[data-catalog-category]')];
  const grid = catalog.querySelector('[data-catalog-grid]');
  const table = catalog.querySelector('[data-catalog-table]');
  const count = catalog.querySelector('[data-result-count]');
  const empty = catalog.querySelector('[data-empty]');
  const emptyFilters = empty.textContent;
  const title = catalog.querySelector('[data-catalog-title]');
  const catalogTitle = title.textContent;
  const favoritesButton = document.querySelector('[data-favorites-only]');
  const catalogButton = document.querySelector('[data-catalog-home]');
  const initialCategory = category.value;
  let favoritesOnly = catalog.dataset.startFavorites === 'true';
  const groups = [grid, table.querySelector('[data-catalog-rows]')].map(container => ({
    container, elements: [...container.children],
  }));
  const language = document.documentElement.lang;
  const compareText = new Intl.Collator(language, { sensitivity: 'base', numeric: true });
  const normalize = text => text.normalize('NFD').replace(/\p{Diacritic}/gu, '').toLowerCase();
  const categories = new Map([...category.options].map(option => [option.value, option.textContent]));
  const index = new Map(groups[0].elements.map((item, position) => [item.dataset.app, position]));
  const sortBy = {
    featured: (a, b) => index.get(a.app) - index.get(b.app),
    name: (a, b) => compareText.compare(a.name, b.name),
    stars: (a, b) => Number(b.stars) - Number(a.stars),
    latest: (a, b) => b.release.localeCompare(a.release),
  };
  function loadVisibleMedia() {
    const tableActive = !table.hidden;
    const container = groups[Number(tableActive)].container;
    const visible = [...container.children].filter(item => !item.hidden).slice(0, tableActive ? 12 : 6);
    for (const item of visible) {
      for (const img of item.querySelectorAll('img[loading=lazy]')) img.loading = 'eager';
    }
  }

  function render({ reorder = false } = {}) {
    const q = normalize(search.value).trim().split(/\s+/).filter(Boolean);
    const matchesScope = item => {
      const data = item.dataset;
      return (level === 'all_apps' || data.level === level)
        && (!favoritesOnly || favorites.has(data.app))
        && (!provider.value || data.provider === provider.value)
        && q.every(word => normalize(`${data.search} ${categories.get(data.category)}`).includes(word));
    };
    const counts = new Map();
    const matched = new Set();
    let totalInScope = 0;
    for (const item of groups[0].elements) {
      if (!matchesScope(item)) continue;
      const { app, category: itemCategory } = item.dataset;
      totalInScope++;
      counts.set(itemCategory, (counts.get(itemCategory) || 0) + 1);
      if (!category.value || category.value === itemCategory) matched.add(app);
    }
    for (const { container, elements } of groups) {
      for (const element of elements) element.hidden = !matched.has(element.dataset.app);
      if (reorder) {
        const order = sortBy[sort.value];
        const sorted = [...elements].sort((a, b) => order(a.dataset, b.dataset) || sortBy.name(a.dataset, b.dataset));
        container.append(...sorted);
      }
    }
    count.textContent = matched.size;
    empty.hidden = matched.size !== 0;
    empty.textContent = favoritesOnly && !groups[0].elements.some(item => favorites.has(item.dataset.app))
      ? empty.dataset.emptyFavorites : emptyFilters;
    title.textContent = favoritesOnly ? title.dataset.favoritesTitle : catalogTitle;
    favoritesButton.setAttribute('aria-pressed', favoritesOnly);
    favoritesButton.classList.toggle('on', favoritesOnly);
    catalogButton.setAttribute('aria-pressed', !favoritesOnly);
    catalogButton.classList.toggle('on', !favoritesOnly);
    for (const button of levelButtons) {
      const active = button.dataset.catalogLevel === level;
      button.classList.toggle('on', active);
      button.setAttribute('aria-pressed', active);
    }
    for (const button of categoryButtons) {
      const key = button.dataset.catalogCategory;
      const active = !favoritesOnly && key === category.value;
      button.classList.toggle('on', active);
      button.setAttribute('aria-pressed', active);
      button.querySelector('[data-category-count]').textContent = key ? (counts.get(key) || 0) : totalInScope;
    }
    loadVisibleMedia();
  }

  function switchView(showFavorites) {
    if (favoritesOnly === showFavorites) return;
    favoritesOnly = showFavorites;
    search.value = provider.value = '';
    category.value = favoritesOnly ? '' : initialCategory;
    level = favoritesOnly ? 'all_apps' : initialLevel;
    catalog.scrollTop = 0;
    render();
  }
  catalog.addEventListener('favorites-changed', () => render());
  favoritesButton.addEventListener('click', () => switchView(true));
  catalogButton.addEventListener('click', () => switchView(false));
  search.addEventListener('input', () => render());
  provider.addEventListener('change', () => render());
  category.addEventListener('change', () => {
    const selected = categoryButtons.find(button => button.dataset.catalogCategory === category.value);
    const group = selected?.closest('.category-group');
    if (group) group.open = true;
    render();
  });
  sort.addEventListener('change', () => render({ reorder: true }));

  for (const button of levelButtons) button.addEventListener('click', () => {
    level = button.dataset.catalogLevel;
    render();
  });
  for (const button of categoryButtons) button.addEventListener('click', () => {
    if (favoritesOnly) level = initialLevel;
    favoritesOnly = false;
    category.value = button.dataset.catalogCategory;
    render();
  });
  catalog.querySelector('[data-reset-filters]').addEventListener('click', () => {
    search.value = provider.value = category.value = '';
    sort.value = 'featured';
    level = favoritesOnly ? 'all_apps' : initialLevel;
    render({ reorder: true });
  });

  const layouts = [...document.querySelectorAll('[data-layout]')];
  for (const button of layouts) button.addEventListener('click', () => {
    const tableActive = button.dataset.layout === 'table';
    grid.hidden = tableActive;
    table.hidden = !tableActive;
    for (const item of layouts) item.setAttribute('aria-pressed', item === button);
    catalog.scrollTop = 0;
    loadVisibleMedia();
  });

  render();
})();
