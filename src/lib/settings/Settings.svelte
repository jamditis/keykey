<script lang="ts">
  import { onMount } from 'svelte';
  import { configStore, type AppConfig } from '../stores/config';
  import DisplaySettings from './DisplaySettings.svelte';
  import InputSettings from './InputSettings.svelte';
  import AppearanceSettings from './AppearanceSettings.svelte';
  import ShortcutSettings from './ShortcutSettings.svelte';
  import AboutSettings from './AboutSettings.svelte';

  type Tab = 'display' | 'input' | 'appearance' | 'shortcuts' | 'about';

  let activeTab = $state<Tab>('display');
  let config = $state<AppConfig | null>(null);
  let saving = $state(false);
  let saveError = $state<string | null>(null);

  const tabs: { id: Tab; label: string; icon: string }[] = [
    { id: 'display', label: 'Display', icon: 'D' },
    { id: 'input', label: 'Input', icon: 'I' },
    { id: 'appearance', label: 'Appearance', icon: 'A' },
    { id: 'shortcuts', label: 'Shortcuts', icon: 'S' },
    { id: 'about', label: 'About', icon: '?' },
  ];

  onMount(async () => {
    try {
      config = await configStore.load();
    } catch (e) {
      saveError = 'Failed to load config: ' + String(e);
    }
  });

  async function handleSave() {
    if (!config) return;
    saving = true;
    saveError = null;
    try {
      await configStore.save(config);
    } catch (e) {
      saveError = 'Failed to save: ' + String(e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="settings-root">
  <aside class="sidebar">
    <div class="sidebar-header">
      <div class="logo-mark">KK</div>
      <span class="sidebar-title">KeyKey</span>
    </div>

    <nav class="nav">
      {#each tabs as tab}
        <button
          class="nav-item"
          class:active={activeTab === tab.id}
          onclick={() => (activeTab = tab.id)}
        >
          <span class="nav-icon">{tab.icon}</span>
          <span class="nav-label">{tab.label}</span>
        </button>
      {/each}
    </nav>

    <div class="sidebar-footer">
      {#if saving}
        <span class="status saving">Saving...</span>
      {:else if saveError}
        <span class="status error">{saveError}</span>
      {:else}
        <span class="status ok">All changes saved</span>
      {/if}
    </div>
  </aside>

  <main class="content">
    {#if config === null}
      <div class="loading">
        <div class="spinner"></div>
        <span>Loading config...</span>
      </div>
    {:else}
      <div class="panel-wrap">
        {#if activeTab === 'display'}
          <DisplaySettings bind:config={config} onSave={handleSave} />
        {:else if activeTab === 'input'}
          <InputSettings bind:config={config} onSave={handleSave} />
        {:else if activeTab === 'appearance'}
          <AppearanceSettings bind:config={config} onSave={handleSave} />
        {:else if activeTab === 'shortcuts'}
          <ShortcutSettings bind:config={config} onSave={handleSave} />
        {:else if activeTab === 'about'}
          <AboutSettings />
        {/if}
      </div>
    {/if}
  </main>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600;700;800&display=swap');

  :global(body) {
    margin: 0;
    padding: 0;
    background: #13132a;
    color: #e0e0e0;
    font-family: 'DM Sans', 'Segoe UI', system-ui, sans-serif;
    font-size: 14px;
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
  }

  .settings-root {
    display: flex;
    height: 100vh;
    width: 100vw;
    background: #13132a;
    overflow: hidden;
  }

  /* Sidebar */
  .sidebar {
    width: 190px;
    flex-shrink: 0;
    background: #0f0f22;
    border-right: 1px solid #1e1e38;
    display: flex;
    flex-direction: column;
    padding: 0;
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 20px 16px 16px;
    border-bottom: 1px solid #1e1e38;
  }

  .logo-mark {
    width: 30px;
    height: 30px;
    background: linear-gradient(135deg, #6655dd, #9944cc);
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 800;
    color: #fff;
    letter-spacing: -0.04em;
    flex-shrink: 0;
  }

  .sidebar-title {
    font-size: 14px;
    font-weight: 700;
    color: #c0c0dd;
    letter-spacing: -0.01em;
  }

  .nav {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 12px 8px;
    overflow-y: auto;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 10px;
    border-radius: 7px;
    border: none;
    background: none;
    color: #6666aa;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: background 0.15s, color 0.15s;
    font-family: inherit;
  }

  .nav-item:hover {
    background: #1e1e38;
    color: #c0c0dd;
  }

  .nav-item.active {
    background: #221e44;
    color: #e0e0ff;
  }

  .nav-icon {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    background: #2a2a4a;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 700;
    color: #8888cc;
    flex-shrink: 0;
    transition: background 0.15s, color 0.15s;
  }

  .nav-item.active .nav-icon {
    background: #6655dd;
    color: #fff;
  }

  .nav-label {
    flex: 1;
  }

  .sidebar-footer {
    padding: 12px 14px;
    border-top: 1px solid #1e1e38;
    min-height: 40px;
    display: flex;
    align-items: center;
  }

  .status {
    font-size: 11px;
  }

  .status.ok {
    color: #44884a;
  }

  .status.saving {
    color: #8888aa;
  }

  .status.error {
    color: #cc4444;
  }

  /* Content */
  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .panel-wrap {
    flex: 1;
    overflow-y: auto;
    padding: 28px 32px;
    scrollbar-width: thin;
    scrollbar-color: #2a2a4a transparent;
  }

  .panel-wrap::-webkit-scrollbar {
    width: 5px;
  }

  .panel-wrap::-webkit-scrollbar-thumb {
    background: #2a2a4a;
    border-radius: 3px;
  }

  .loading {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 14px;
    color: #6666aa;
    font-size: 13px;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid #2a2a4a;
    border-top-color: #6655dd;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
