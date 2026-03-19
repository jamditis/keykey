<script lang="ts">
  import type { AppConfig } from '../stores/config';
  import { BUILT_IN_THEMES } from '../theme-engine';

  let { config = $bindable(), onSave }: { config: AppConfig; onSave: () => void } = $props();

  let selectedTheme = $derived(
    BUILT_IN_THEMES.find((t) => t.name === config.appearance.active_theme)
  );
</script>

<div class="panel">
  <h2 class="panel-title">Appearance</h2>

  <div class="field">
    <label for="theme">Overlay theme</label>
    <select id="theme" bind:value={config.appearance.active_theme} onchange={onSave}>
      {#each BUILT_IN_THEMES as theme}
        <option value={theme.name}>{theme.label}</option>
      {/each}
    </select>
  </div>

  {#if selectedTheme}
    <div class="preview-section">
      <span class="preview-label">Preview</span>
      <div
        class="preview-pill"
        style:background={selectedTheme.vars['--kk-bg']}
        style:color={selectedTheme.vars['--kk-text']}
        style:font-family={selectedTheme.vars['--kk-font']}
        style:font-size={selectedTheme.vars['--kk-font-size']}
        style:padding="{selectedTheme.vars['--kk-padding-y']} {selectedTheme.vars['--kk-padding-x']}"
        style:border-radius={selectedTheme.vars['--kk-radius']}
        style:border="1px solid {selectedTheme.vars['--kk-border-color']}"
        style:box-shadow={selectedTheme.vars['--kk-shadow']}
      >
        Ctrl + Shift + S
      </div>
    </div>

    <div class="vars-grid">
      {#each Object.entries(selectedTheme.vars) as [key, value]}
        <div class="var-row">
          <span class="var-key">{key.replace('--kk-', '')}</span>
          <span class="var-value">{value}</span>
          {#if key.includes('color') || key === '--kk-bg' || key === '--kk-text'}
            <span class="color-swatch" style:background={value}></span>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .panel-title {
    font-family: 'DM Sans', 'Segoe UI', sans-serif;
    font-size: 18px;
    font-weight: 600;
    color: #e0e0e0;
    margin: 0 0 4px 0;
    padding-bottom: 12px;
    border-bottom: 1px solid #2a2a4a;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  label {
    font-size: 12px;
    font-weight: 500;
    color: #8888aa;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  select {
    background: #2a2a4a;
    border: 1px solid #3a3a5a;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 13px;
    padding: 8px 10px;
    outline: none;
    transition: border-color 0.15s;
    width: 100%;
    box-sizing: border-box;
  }

  select:focus {
    border-color: #6655dd;
  }

  .preview-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .preview-label {
    font-size: 12px;
    font-weight: 500;
    color: #8888aa;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .preview-pill {
    display: inline-block;
    align-self: flex-start;
    font-weight: 500;
    white-space: nowrap;
    transition: all 0.2s;
  }

  .vars-grid {
    display: flex;
    flex-direction: column;
    gap: 4px;
    background: #1a1a30;
    border: 1px solid #2a2a4a;
    border-radius: 8px;
    padding: 12px;
    overflow: hidden;
  }

  .var-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    padding: 3px 0;
  }

  .var-key {
    color: #7777bb;
    font-family: 'Cascadia Code', 'Consolas', monospace;
    min-width: 120px;
    flex-shrink: 0;
  }

  .var-value {
    color: #9988ee;
    font-family: 'Cascadia Code', 'Consolas', monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .color-swatch {
    width: 12px;
    height: 12px;
    border-radius: 3px;
    flex-shrink: 0;
    border: 1px solid rgba(255, 255, 255, 0.15);
  }
</style>
