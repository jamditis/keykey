<script lang="ts">
  import type { AppConfig } from '../stores/config';

  let { config = $bindable(), onSave }: { config: AppConfig; onSave: () => void } = $props();

  const shortcuts: { key: keyof AppConfig['shortcuts']; label: string; desc: string }[] = [
    { key: 'toggle_capture', label: 'Toggle capture', desc: 'Start or stop key capture' },
    { key: 'switch_mode', label: 'Switch mode', desc: 'Cycle between display modes' },
    { key: 'toggle_overlay', label: 'Toggle overlay', desc: 'Show or hide the overlay window' },
  ];
</script>

<div class="panel">
  <h2 class="panel-title">Shortcuts</h2>

  <p class="intro">
    Global hotkeys work system-wide, even when KeyKey is not in focus. Enter a key combination
    like <code>Ctrl+Shift+K</code> or leave blank to disable.
  </p>

  <div class="shortcut-list">
    {#each shortcuts as shortcut}
      <div class="shortcut-row">
        <div class="shortcut-info">
          <span class="shortcut-label">{shortcut.label}</span>
          <span class="shortcut-desc">{shortcut.desc}</span>
        </div>
        <div class="shortcut-input-wrap">
          <input
            type="text"
            class="shortcut-input"
            placeholder="Not set"
            value={config.shortcuts[shortcut.key] ?? ''}
            oninput={(e) => {
              const val = (e.currentTarget as HTMLInputElement).value.trim();
              config.shortcuts[shortcut.key] = val || null;
            }}
            onblur={onSave}
          />
          {#if config.shortcuts[shortcut.key]}
            <button
              class="clear-btn"
              onclick={() => {
                config.shortcuts[shortcut.key] = null;
                onSave();
              }}
              aria-label="Clear shortcut"
            >
              x
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>

  <div class="notice">
    <span class="notice-icon">i</span>
    Hotkey recording is not yet implemented. Type combinations manually
    (e.g., <code>Ctrl+Alt+K</code>). Changes take effect after restart.
  </div>
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

  .intro {
    font-size: 13px;
    color: #8888aa;
    margin: 0;
    line-height: 1.6;
  }

  code {
    font-family: 'Cascadia Code', 'Consolas', monospace;
    background: #2a2a4a;
    border-radius: 3px;
    padding: 1px 5px;
    font-size: 12px;
    color: #9988ee;
  }

  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    background: #1e1e38;
    border: 1px solid #2a2a4a;
    border-radius: 8px;
    padding: 12px 14px;
  }

  .shortcut-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .shortcut-label {
    font-size: 13px;
    font-weight: 500;
    color: #e0e0e0;
  }

  .shortcut-desc {
    font-size: 11px;
    color: #6666aa;
  }

  .shortcut-input-wrap {
    position: relative;
    flex-shrink: 0;
  }

  .shortcut-input {
    background: #2a2a4a;
    border: 1px solid #3a3a5a;
    border-radius: 6px;
    color: #e0e0e0;
    font-family: 'Cascadia Code', 'Consolas', monospace;
    font-size: 12px;
    padding: 7px 28px 7px 10px;
    outline: none;
    width: 160px;
    transition: border-color 0.15s;
  }

  .shortcut-input:focus {
    border-color: #6655dd;
  }

  .shortcut-input::placeholder {
    color: #44446a;
  }

  .clear-btn {
    position: absolute;
    right: 6px;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    color: #6666aa;
    font-size: 12px;
    cursor: pointer;
    padding: 2px 4px;
    line-height: 1;
    border-radius: 3px;
    transition: color 0.15s, background 0.15s;
  }

  .clear-btn:hover {
    color: #e0e0e0;
    background: #3a3a5a;
  }

  .notice {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    background: #1e1e30;
    border: 1px solid #3a3a5a;
    border-left: 3px solid #6655dd;
    border-radius: 6px;
    padding: 12px 14px;
    font-size: 12px;
    color: #8888aa;
    line-height: 1.6;
  }

  .notice-icon {
    flex-shrink: 0;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #3a3a5a;
    color: #9988ee;
    font-size: 10px;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 1px;
  }
</style>
