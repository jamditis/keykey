<script lang="ts">
  import type { AppConfig } from '../stores/config';

  let { config = $bindable(), onSave }: { config: AppConfig; onSave: () => void } = $props();

  const modifierModes = [
    { value: 'smart', label: 'Smart (based on context)' },
    { value: 'always_show', label: 'Always show' },
    { value: 'never_show', label: 'Never show' },
    { value: 'combo_only', label: 'Show only in combos' },
  ];
</script>

<div class="panel">
  <h2 class="panel-title">Input</h2>

  <section class="section">
    <h3 class="section-title">Smart mode</h3>
    <div class="field toggle-field">
      <div class="toggle-label">
        <span class="label-main">Enable smart mode</span>
        <span class="label-sub">Groups keystrokes into combos based on timing</span>
      </div>
      <label class="toggle">
        <input
          type="checkbox"
          bind:checked={config.input.smart_mode}
          onchange={onSave}
        />
        <span class="track"></span>
      </label>
    </div>

    {#if config.input.smart_mode}
      <div class="field">
        <label for="smart-threshold">Combo threshold</label>
        <div class="slider-row">
          <input
            id="smart-threshold"
            type="range"
            min="50"
            max="500"
            step="10"
            bind:value={config.input.smart_threshold_ms}
            onchange={onSave}
          />
          <span class="value-badge">{config.input.smart_threshold_ms}ms</span>
        </div>
        <span class="hint">Keys pressed within this window are grouped as a combo</span>
      </div>

      <div class="field">
        <label for="repeat-window">Repeat window</label>
        <div class="slider-row">
          <input
            id="repeat-window"
            type="range"
            min="50"
            max="1000"
            step="50"
            bind:value={config.input.repeat_window_ms}
            onchange={onSave}
          />
          <span class="value-badge">{config.input.repeat_window_ms}ms</span>
        </div>
        <span class="hint">Repeated keys within this window are collapsed</span>
      </div>
    {/if}

    <div class="field toggle-field">
      <div class="toggle-label">
        <span class="label-main">Show all keystrokes</span>
        <span class="label-sub">Include modifier-only keypresses</span>
      </div>
      <label class="toggle">
        <input
          type="checkbox"
          bind:checked={config.input.show_all_keystrokes}
          onchange={onSave}
        />
        <span class="track"></span>
      </label>
    </div>
  </section>

  <section class="section">
    <h3 class="section-title">Modifier display</h3>
    <p class="section-desc">How each modifier key label appears in the overlay</p>

    <div class="modifier-grid">
      <div class="field">
        <label for="mod-ctrl">Ctrl</label>
        <select id="mod-ctrl" bind:value={config.input.modifier_overrides.ctrl} onchange={onSave}>
          {#each modifierModes as m}
            <option value={m.value}>{m.label}</option>
          {/each}
        </select>
      </div>

      <div class="field">
        <label for="mod-alt">Alt</label>
        <select id="mod-alt" bind:value={config.input.modifier_overrides.alt} onchange={onSave}>
          {#each modifierModes as m}
            <option value={m.value}>{m.label}</option>
          {/each}
        </select>
      </div>

      <div class="field">
        <label for="mod-shift">Shift</label>
        <select id="mod-shift" bind:value={config.input.modifier_overrides.shift} onchange={onSave}>
          {#each modifierModes as m}
            <option value={m.value}>{m.label}</option>
          {/each}
        </select>
      </div>

      <div class="field">
        <label for="mod-win">Win</label>
        <select id="mod-win" bind:value={config.input.modifier_overrides.win} onchange={onSave}>
          {#each modifierModes as m}
            <option value={m.value}>{m.label}</option>
          {/each}
        </select>
      </div>
    </div>
  </section>
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 28px;
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

  .section {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .section-title {
    font-size: 13px;
    font-weight: 600;
    color: #aaaacc;
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .section-desc {
    font-size: 12px;
    color: #6666aa;
    margin: -8px 0 0 0;
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

  .toggle-field {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    background: #1e1e38;
    border: 1px solid #2a2a4a;
    border-radius: 8px;
    padding: 12px 14px;
  }

  .toggle-label {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .label-main {
    font-size: 13px;
    color: #e0e0e0;
    font-weight: 500;
  }

  .label-sub {
    font-size: 11px;
    color: #6666aa;
  }

  .toggle {
    position: relative;
    display: inline-block;
    width: 40px;
    height: 22px;
    flex-shrink: 0;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
    position: absolute;
  }

  .track {
    position: absolute;
    inset: 0;
    background: #2a2a4a;
    border-radius: 11px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .track::after {
    content: '';
    position: absolute;
    left: 3px;
    top: 3px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #6666aa;
    transition: transform 0.2s, background 0.2s;
  }

  .toggle input:checked + .track {
    background: #6655dd;
  }

  .toggle input:checked + .track::after {
    transform: translateX(18px);
    background: #fff;
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  input[type='range'] {
    flex: 1;
    -webkit-appearance: none;
    appearance: none;
    height: 4px;
    background: #2a2a4a;
    border-radius: 2px;
    outline: none;
  }

  input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #6655dd;
    cursor: pointer;
  }

  .value-badge {
    font-size: 12px;
    font-weight: 600;
    color: #9988ee;
    background: #2a2a4a;
    border-radius: 4px;
    padding: 2px 8px;
    min-width: 52px;
    text-align: center;
  }

  .hint {
    font-size: 11px;
    color: #555588;
    margin-top: -2px;
  }

  .modifier-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
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
</style>
