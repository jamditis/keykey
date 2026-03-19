<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  interface KeyEvent {
    key: string;
    key_code: string;
    event_type: string;
    timestamp: number;
  }

  let lastKey = $state('waiting for input...');
  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<KeyEvent>('key-event', (event) => {
      if (event.payload.event_type === 'press') {
        lastKey = event.payload.key;
      }
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<div class="overlay-container">
  <div class="debug-label">{lastKey}</div>
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .debug-label {
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    padding: 8px 16px;
    border-radius: 6px;
    font-family: system-ui, sans-serif;
    font-size: 14px;
  }
</style>
