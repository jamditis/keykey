<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { keystream } from '../stores/keystream';
  import TextStream from './TextStream.svelte';
  import type { DisplayEvent } from '../types';

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<DisplayEvent>('display-event', (event) => {
      keystream.push(event.payload);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
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
