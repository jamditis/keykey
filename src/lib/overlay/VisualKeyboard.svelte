<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { ANSI_COMPACT } from '../keyboard-layouts/ansi-104';
  import KeyCap from './KeyCap.svelte';

  interface RawKeyEvent {
    key: string;
    key_code: string;
    event_type: string;
    timestamp: number;
  }

  let pressedKeys = $state<Set<string>>(new Set());
  let unlisten: (() => void) | null = null;

  const rows = $derived(
    Array.from(new Set(ANSI_COMPACT.map((k) => k.row)))
      .sort((a, b) => a - b)
      .map((row) => ANSI_COMPACT.filter((k) => k.row === row))
  );

  onMount(async () => {
    unlisten = await listen<RawKeyEvent>('key-event', (event) => {
      const code = event.payload.key_code;
      if (event.payload.event_type === 'press') {
        pressedKeys = new Set([...pressedKeys, code]);
      } else {
        const next = new Set(pressedKeys);
        next.delete(code);
        pressedKeys = next;
      }
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<div class="visual-keyboard">
  {#each rows as row, i}
    <div class="keyboard-row">
      {#each row as keyDef}
        <KeyCap {keyDef} pressed={pressedKeys.has(keyDef.code)} />
      {/each}
    </div>
  {/each}
</div>

<style>
  .visual-keyboard {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 10px;
    background: var(--kk-kb-bg, rgba(15, 15, 22, 0.88));
    border: 1px solid var(--kk-kb-border, rgba(255, 255, 255, 0.08));
    border-radius: 10px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(12px);
  }

  .keyboard-row {
    display: flex;
    flex-direction: row;
    gap: 2px;
  }
</style>
