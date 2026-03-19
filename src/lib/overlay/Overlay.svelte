<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { keystream } from '../stores/keystream';
  import TextStream from './TextStream.svelte';
  import VisualKeyboard from './VisualKeyboard.svelte';
  import { applyTheme, getThemeByName } from '../theme-engine';
  import type { DisplayEvent } from '../types';

  let displayMode = $state('text_stream');
  let fadeDurationMs = $state(2000);
  let stackDirection = $state('up');
  let marginX = $state(32);
  let marginY = $state(32);
  let corner = $state('bottom_right');
  let unlistenDisplay: (() => void) | null = null;
  let unlistenConfig: (() => void) | null = null;
  let unlistenSwitchMode: (() => void) | null = null;

  function loadConfig(config: any) {
    displayMode = config.display.mode;
    fadeDurationMs = config.display.fade_duration_ms ?? 2000;
    stackDirection = config.display.stack_direction ?? 'up';
    marginX = config.display.margins?.x ?? 32;
    marginY = config.display.margins?.y ?? 32;
    corner = config.display.corner ?? 'bottom_right';

    keystream.configure({
      maxVisible: config.display.max_visible ?? 5,
      fadeDurationMs: fadeDurationMs,
    });

    const theme = getThemeByName(config.appearance.active_theme);
    if (theme) applyTheme(theme);
  }

  function containerStyle(): string {
    const styles: string[] = ['position: fixed'];
    if (corner.includes('bottom')) styles.push(`bottom: ${marginY}px`);
    else styles.push(`top: ${marginY}px`);
    if (corner.includes('right')) styles.push(`right: ${marginX}px`);
    else styles.push(`left: ${marginX}px`);
    return styles.join('; ');
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

<div class="overlay-container" style={containerStyle()}>
  {#if displayMode === 'visual_keyboard'}
    <VisualKeyboard />
  {:else}
    <TextStream fadeMs={fadeDurationMs} {stackDirection} />
  {/if}
</div>

<style>
  .overlay-container {
    position: fixed;
  }
</style>
