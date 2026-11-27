(() => {
  const invoke = window.__TAURI__.core.invoke;
  const it = document.documentElement.lang === 'it';
  const t = (italian, english) => it ? italian : english;
  const clients = [
    ['codex', 'Codex'], ['claude_code', 'Claude Code'], ['cursor', 'Cursor'],
    ['vs_code', 'VS Code'], ['open_code', 'OpenCode 2'],
  ];
  const examples = [
    t('Trova alternative gratuite a Granola.', 'Find free alternatives to Granola.'),
    t('Quali app hanno aggiornamenti disponibili?', 'Which apps have updates available?'),
    t('Chiedi di installare Handy.', 'Ask to install Handy.'),
  ];

  function dialog(title, body, footer = '') {
    const panel = document.createElement('dialog');
    panel.className = 'launcher-settings';
    const id = `heading-${document.querySelectorAll('.launcher-settings').length}`;
    panel.setAttribute('aria-labelledby', id);
    panel.innerHTML = `<header class="set-hd"><img src="brand/icon.png" width="30" height="30" alt="">
      <h2 id="${id}">${title}</h2></header>
      <div class="set-b">${body}</div>
      <footer class="set-ft"><p role="status"></p>${footer}<form method="dialog">
        <button class="btn solid">${t('Chiudi', 'Close')}</button>
      </form></footer>`;
    document.body.append(panel);
    return panel;
  }

  function activate(view, panel, onOpen) {
    const button = document.querySelector(`[data-view="${view}"]`);
    button.disabled = false;
    button.removeAttribute('aria-disabled');
    button.removeAttribute('data-proof-unavailable');
    button.removeAttribute('title');
    button.addEventListener('click', () => {
      panel.querySelector('[role=status]').textContent = '';
      panel.showModal();
      onOpen?.();
    });
  }

  async function copy(text) {
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      const focused = document.activeElement;
      const field = document.createElement('textarea');
      field.value = text;
      field.className = 'clipboard-field';
      field.readOnly = true;
      (document.querySelector('.launcher-settings[open]') || document.body).append(field);
      field.select();
      const result = document.execCommand('copy');
      field.remove();
      focused?.focus();
      if (!result) throw new Error(t('Copia non riuscita', 'Copy failed'));
    }
  }

  const agent = dialog(t('Collega agente · MCP', 'Connect agent · MCP'), `
    <section class="card"><h3>${t('Configurazione', 'Configuration')}</h3>
      <p>${t('Scegli il client e copia la configurazione da aggiungere alle sue impostazioni MCP.',
        'Choose a client and copy the configuration into its MCP settings.')}</p>
      <div class="mcp-clients" role="group" aria-label="${t('Client MCP', 'MCP clients')}">
        ${clients.map(([id, name], index) => `<button type="button" class="btn" data-client="${id}" aria-pressed="${index === 0}">${name}</button>`).join('')}
      </div>
      <p data-opencode-help hidden>${t('Aggiungi la configurazione a opencode.jsonc. Il comando opencode debug paths mostra la cartella di configurazione.',
        'Add the configuration to opencode.jsonc. Run opencode debug paths to find the configuration directory.')}</p>
      <pre class="mcp-config" aria-label="${t('Configurazione MCP', 'MCP configuration')}" tabindex="0"></pre>
      <div class="mcp-actions">
        <button class="btn solid" type="button" data-copy-config disabled>${t('Copia configurazione', 'Copy configuration')}</button>
        <button class="btn" type="button" data-mcp-docs>${t('Documentazione', 'Documentation')}</button>
      </div>
    </section>
    <section class="card"><h3>${t('Prova con il tuo agente', 'Try with your agent')}</h3>
      ${examples.map((example, index) => `<div class="mcp-example"><span>${example}</span>
        <button type="button" class="btn" data-example="${index}" aria-label="${t('Copia esempio', 'Copy example')} ${index + 1}">${t('Copia', 'Copy')}</button></div>`).join('')}
      <p>${t('Per installare, aggiornare o rimuovere un’app, approva la richiesta qui nel launcher.',
        'Approve requests to install, update or remove apps here in the launcher.')}</p>
      <button type="button" class="btn" data-check-mcp>${t('Verifica collegamento', 'Check connection')}</button>
    </section>`);
  const agentStatus = agent.querySelector('[role=status]');
  const openCodeHelp = agent.querySelector('[data-opencode-help]');
  const config = agent.querySelector('.mcp-config');
  const configCopy = agent.querySelector('[data-copy-config]');
  let selectedClient = clients[0][0];
  let requestNumber = 0;

  async function showConfig(client) {
    const currentRequest = ++requestNumber;
    selectedClient = client;
    openCodeHelp.hidden = client !== 'open_code';
    for (const button of agent.querySelectorAll('[data-client]')) {
      button.setAttribute('aria-pressed', String(button.dataset.client === client));
    }
    configCopy.disabled = true;
    config.textContent = '';
    agentStatus.textContent = t('Caricamento configurazione…', 'Loading configuration…');
    try {
      const value = await invoke('mcp_configuration', {client});
      if (currentRequest !== requestNumber) return;
      config.textContent = value;
      configCopy.disabled = false;
      agentStatus.textContent = '';
    } catch (error) {
      if (currentRequest !== requestNumber) return;
      agentStatus.textContent = String(error);
    }
  }

  agent.addEventListener('click', async event => {
    const button = event.target.closest('button');
    if (!button || button.closest('form')) return;
    try {
      if (button.dataset.client) await showConfig(button.dataset.client);
      if (button.hasAttribute('data-mcp-docs')) await invoke('open_mcp_docs', {client:selectedClient});
      if (button.hasAttribute('data-copy-config') || button.hasAttribute('data-example')) {
        await copy(button.hasAttribute('data-copy-config') ? config.textContent : examples[Number(button.dataset.example)]);
        agentStatus.textContent = t('Copiato', 'Copied');
      }
      if (button.hasAttribute('data-check-mcp')) {
        agentStatus.textContent = await invoke('mcp_connection')
          ? t('Agente collegato', 'Agent connected')
          : t('Nessuna richiesta recente. Prova uno degli esempi dal tuo agente.', 'No recent request. Try an example from your agent.');
      }
    } catch (error) {
      agentStatus.textContent = String(error);
    }
  });
  activate('mcp_connect', agent, () => showConfig(selectedClient));

  const settings = dialog(t('Impostazioni', 'Settings'), `
    <section class="card"><h3>${t('Lingua', 'Language')}</h3>
      <div class="mcp-actions">
        <button type="button" class="btn" data-language="it" aria-pressed="${it}">Italiano</button>
        <button type="button" class="btn" data-language="en" aria-pressed="${!it}">English</button>
      </div>
    </section>
    <section class="card"><h3>${t('Sistema', 'System')}</h3>
      <p data-platform></p>
    </section>
    <section class="card"><h3>${t('Aggiornamenti', 'Updates')}</h3>
      <button type="button" class="btn" data-check-updates disabled>${t('Cerca aggiornamenti delle app', 'Check for app updates')}</button>
    </section>
    <section class="card"><h3>Promptica</h3><div class="promptica-links"></div></section>`);
  const links = settings.querySelector('.promptica-links');
  for (const link of document.querySelectorAll('.footer-brand a')) {
    const item = link.cloneNode();
    item.textContent = link.getAttribute('aria-label')?.split(' · ')[0] || 'Promptica.it';
    links.append(item);
  }
  settings.addEventListener('click', async event => {
    const button = event.target.closest('button');
    if (button?.dataset.language && button.dataset.language !== document.documentElement.lang) {
      try {
        await invoke('set_language', {language:button.dataset.language});
        location.assign(`${button.dataset.language}.html`);
      } catch (error) {
        settings.querySelector('[role=status]').textContent = String(error);
      }
    }
    if (button?.hasAttribute('data-check-updates')) {
      const status = settings.querySelector('[role=status]');
      try {
        await invoke('check_native_updates');
        status.textContent = t('Ricerca avviata. Vedi Aggiornamenti per i risultati.', 'Check started. See Updates for results.');
      } catch (error) {
        status.textContent = String(error);
      }
    }
  });
  window.addEventListener('native-snapshot', event => {
    const {platform, lifecycle_enabled: enabled} = event.detail;
    const names = {MacOs:'macOS', Linux:'Linux', Windows:'Windows'};
    settings.querySelector('[data-platform]').textContent = platform
      ? `${names[platform.os] || platform.os} · ${platform.arch}` : t('Rilevamento…', 'Detecting…');
    settings.querySelector('[data-check-updates]').disabled = !enabled;
  });
  activate('settings', settings);
})();
