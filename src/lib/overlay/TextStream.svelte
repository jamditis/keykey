<script lang="ts">
  import { keystream } from '../stores/keystream';
  import KeyLabel from './KeyLabel.svelte';

  interface Props {
    fadeMs?: number;
    stackDirection?: string;
  }

  let { fadeMs = 2000, stackDirection = 'up' }: Props = $props();
</script>

<div class="text-stream" style="flex-direction: {stackDirection === 'down' ? 'column' : 'column-reverse'}">
  {#each $keystream as entry (entry.id)}
    <KeyLabel {entry} {fadeMs} />
  {/each}
</div>

<style>
  .text-stream {
    display: flex;
    align-items: flex-end;
    gap: var(--kk-gap, 4px);
  }
</style>
