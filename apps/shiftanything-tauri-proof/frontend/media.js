/* Catalog pictures are resolved by Rust using the curated ID and slot. */
(() => {
  const invoke = window.__TAURI__.core.invoke;
  const detail = document.querySelector('.catalog-detail');
  const detailContent = detail.querySelector('[data-app-content]');
  const maxWorkers = 3;
  const maxBytes = 2 * 1024 * 1024;
  const visible = new Set();
  const tasks = new WeakMap();
  const active = new Map();

  async function imageSource(raw) {
    const bytes = new Uint8Array(raw);
    if (bytes.length > maxBytes) return null;
    const png = bytes[0] === 0x89 && bytes[1] === 0x50;
    const jpeg = bytes[0] === 0xff && bytes[1] === 0xd8;
    if (!png && !jpeg) return null;
    let binary = '';
    for (let start = 0; start < bytes.length; start += 8192) {
      binary += String.fromCharCode(...bytes.subarray(start, start + 8192));
    }
    const source = `data:image/${png ? 'png' : 'jpeg'};base64,${btoa(binary)}`;
    const picture = new Image();
    picture.src = source;
    await picture.decode();
    return source;
  }

  function pump() {
    for (const target of visible) {
      const task = tasks.get(target);
      if (!task || !target.isConnected) {
        visible.delete(target);
        continue;
      }
      const { appId, screenshot, show } = task;
      const key = `${appId}/${screenshot ?? 'logo'}`;
      let load = active.get(key);
      if (!load) {
        if (active.size >= maxWorkers) continue;
        load = invoke('fetch_catalog_media', { appId, screenshot }).then(imageSource).catch(() => null);
        active.set(key, load);
        load.then(() => {
          active.delete(key);
          pump();
        });
      }
      visible.delete(target);
      observer.unobserve(target);
      tasks.delete(target);
      load.then(source => { if (target.isConnected) show(source); });
    }
  }

  const observer = new IntersectionObserver(entries => {
    for (const entry of entries) {
      if (entry.isIntersecting) visible.add(entry.target);
      else visible.delete(entry.target);
    }
    pump();
  });

  const placeholder = image => image?.getAttribute('src')?.startsWith('data:image/svg+xml,');
  const queue = (target, appId, screenshot, show) => {
    tasks.set(target, { appId, screenshot, show });
    observer.observe(target);
  };
  function render(image, source) {
    if (source) {
      image.dataset.mediaOrigin = image.getAttribute('src');
      image.src = source;
      return;
    }
    const preview = image.closest('[data-preview-error]');
    if (preview) preview.classList.add('preview-error');
    if (image.matches('.product-icon')) {
      const name = image.closest('.cell,.catalog-row,.detail-head').querySelector('.c-name,strong,h1').textContent.trim();
      const fallback = document.createElement('span');
      fallback.className = `${image.className} logo-fallback`;
      fallback.setAttribute('aria-hidden', 'true');
      fallback.textContent = [...name].slice(0, 2).join('').toUpperCase();
      image.replaceWith(fallback);
    }
  }

  function watch(image, appId, screenshot) {
    if (placeholder(image)) queue(image, appId, screenshot, source => render(image, source));
  }
  for (const card of document.querySelectorAll('.cell[data-app],.catalog-row[data-app]')) {
    const logo = card.querySelector('img.product-icon');
    const preview = card.querySelector('.preview-shot img');
    watch(logo, card.dataset.app, null);
    watch(preview, card.dataset.app, preview?.matches('.preview-logo') ? null : 0);
  }

  let comparisonLogos = [];
  document.addEventListener('comparison-rows-rendered', event => {
    for (const fallback of comparisonLogos) {
      observer.unobserve(fallback);
      visible.delete(fallback);
      tasks.delete(fallback);
    }
    comparisonLogos = [...event.target.querySelectorAll('[data-sh-logo]')];
    for (const fallback of comparisonLogos) {
      queue(fallback, fallback.dataset.shLogo, null, source => {
        if (!source) return;
        const image = document.createElement('img');
        image.className = 'icoapp product-icon';
        image.alt = '';
        image.src = source;
        fallback.replaceWith(image);
      });
    }
  });

  function watchDetail() {
    const appId = detailContent.querySelector('[data-native-app]')?.dataset.nativeApp;
    if (!appId) return;
    const preview = detailContent.querySelector('.detail-shot');
    const main = preview?.querySelector('img');
    const shots = [...detailContent.querySelectorAll('[data-shot]')];
    watch(detailContent.querySelector('img.product-icon'), appId, null);
    if (!shots.length) {
      watch(main, appId, null);
      return;
    }
    shots.forEach((shot, index) => {
      const thumb = shot.querySelector('img');
      const fallback = thumb.getAttribute('src');
      // Keep gallery clicks entirely local while the thumbnail is waiting or unavailable.
      shot.dataset.shotOrigin = fallback;
      shot.dataset.shot = fallback;
      if (!placeholder(thumb)) return;
      const selected = () => shot.getAttribute('aria-pressed') === 'true';
      let resolved = false;
      const show = source => {
        if (resolved) return;
        resolved = true;
        observer.unobserve(thumb);
        tasks.delete(thumb);
        visible.delete(thumb);
        shot.dataset.shot = source ?? fallback;
        thumb.classList.toggle('media-failed', !source);
        if (source) render(thumb, source);
        if (selected()) {
          if (source) main.dataset.mediaOrigin = fallback;
          else delete main.dataset.mediaOrigin;
          main.src = shot.dataset.shot;
          preview.classList.toggle('preview-error', !source);
        }
      };
      queue(thumb, appId, index, show);
      if (selected() && placeholder(main)) queue(main, appId, index, show);
    });
  }

  new MutationObserver(watchDetail).observe(detailContent, { childList: true });
  detail.addEventListener('click', event => {
    const shot = event.target.closest('[data-shot]');
    if (shot?.querySelector('img.media-failed')) detailContent.querySelector('.detail-shot').classList.add('preview-error');
  });
})();
