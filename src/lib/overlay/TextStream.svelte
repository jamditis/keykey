<script lang="ts">
  import { keystream } from '../stores/keystream';
  import KeyLabel from './KeyLabel.svelte';

  const FADE_MS = 2000;
</script>

<div class="text-stream">
  {#each $keystream as entry (entry.id)}
    <KeyLabel {entry} fadeMs={FADE_MS} />
  {/each}
</div>

<style>
  .text-stream {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--kk-gap, 4px);
  }
</style>
