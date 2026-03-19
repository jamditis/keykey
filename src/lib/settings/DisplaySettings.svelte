<script lang="ts">
  import type { AppConfig } from '../stores/config';

  let { config = $bindable(), onSave }: { config: AppConfig; onSave: () => void } = $props();
</script>

<div class="panel">
  <h2 class="panel-title">Display</h2>

  <div class="field-group">
    <div class="field">
      <label for="mode">Mode</label>
      <select id="mode" bind:value={config.display.mode} onchange={onSave}>
        <option value="text_stream">Text stream</option>
        <option value="visual_keyboard">Visual keyboard</option>
      </select>
    </div>

    <div class="field">
      <label for="position-strategy">Position strategy</label>
      <select id="position-strategy" bind:value={config.display.position_strategy} onchange={onSave}>
        <option value="corner">Corner</option>
        <option value="center">Center</option>
        <option value="custom">Custom</option>
      </select>
    </div>

    <div class="field">
      <label for="corner">Corner</label>
      <select id="corner" bind:value={config.display.corner} onchange={onSave}>
        <option value="bottom_right">Bottom right</option>
        <option value="bottom_left">Bottom left</option>
        <option value="top_right">Top right</option>
        <option value="top_left">Top left</option>
      </select>
    </div>

    <div class="field">
      <label for="stack-direction">Stack direction</label>
      <select id="stack-direction" bind:value={config.display.stack_direction} onchange={onSave}>
        <option value="up">Up</option>
        <option value="down">Down</option>
      </select>
    </div>
  </div>

  <div class="field-group">
    <div class="field">
      <label for="max-visible">Max visible keys</label>
      <div class="input-row">
        <input
          id="max-visible"
          type="number"
          min="1"
          max="20"
          bind:value={config.display.max_visible}
          onchange={onSave}
        />
        <span class="unit">keys</span>
      </div>
    </div>

    <div class="field">
      <label for="fade-duration">Fade duration</label>
      <div class="input-row">
        <input
          id="fade-duration"
          type="number"
          min="500"
          max="10000"
          step="100"
          bind:value={config.display.fade_duration_ms}
          onchange={onSave}
        />
        <span class="unit">ms</span>
      </div>
    </div>

    <div class="field">
      <label for="margin-x">Margin X</label>
      <div class="input-row">
        <input
          id="margin-x"
          type="number"
          min="0"
          max="500"
          bind:value={config.display.margin_x}
          onchange={onSave}
        />
        <span class="unit">px</span>
      </div>
    </div>

    <div class="field">
      <label for="margin-y">Margin Y</label>
      <div class="input-row">
        <input
          id="margin-y"
          type="number"
          min="0"
          max="500"
          bind:value={config.display.margin_y}
          onchange={onSave}
        />
        <span class="unit">px</span>
      </div>
    </div>
  </div>
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 24px;
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

  .field-group {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
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

  select,
  input[type='number'] {
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

  select:focus,
  input[type='number']:focus {
    border-color: #6655dd;
  }

  .input-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .input-row input {
    flex: 1;
  }

  .unit {
    font-size: 12px;
    color: #6666aa;
    white-space: nowrap;
  }
</style>
