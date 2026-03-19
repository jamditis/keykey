<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { keystream } from '../stores/keystream';
  import TextStream from './TextStream.svelte';
  import { applyTheme, getThemeByName } from '../theme-engine';
  import type { DisplayEvent } from '../types';

  let displayMode = $state('text_stream');
  let unlistenDisplay: (() => void) | null = null;
  let unlistenConfig: (() => void) | null = null;
  let unlistenSwitchMode: (() => void) | null = null;

  function loadConfig(config: any) {
    displayMode = config.display.mode;
    const theme = getThemeByName(config.appearance.active_theme);
    if (theme) applyTheme(theme);
  }

  onMount(async () => {
    try {
      const config: any = await invoke('get_config');
      loadConfig(config);
    } catch (e) {
      console.error('failed to load config:', e);
    }

    unlistenDisplay = await listen<DisplayEvent>('display-event', (event) => {
      keystream.push(event.payload);
    });

    unlistenConfig = await listen<any>('config-changed', (event) => {
      loadConfig(event.payload);
    });

    unlistenSwitchMode = await listen('switch-display-mode', () => {
      displayMode = displayMode === 'text_stream' ? 'visual_keyboard' : 'text_stream';
    });
  });

  onDestroy(() => {
    if (unlistenDisplay) unlistenDisplay();
    if (unlistenConfig) unlistenConfig();
    if (unlistenSwitchMode) unlistenSwitchMode();
    keystream.destroy();
  });
</script>

<div class="overlay-container">
  <TextStream />
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
  }
</style>
