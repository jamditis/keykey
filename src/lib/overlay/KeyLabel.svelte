<script lang="ts">
  import type { StreamEntry } from '../types';

  interface Props {
    entry: StreamEntry;
    fadeMs: number;
  }

  let { entry, fadeMs }: Props = $props();
  let opacity = $state(1);

  $effect(() => {
    const interval = setInterval(() => {
      const age = Date.now() - entry.created_at;
      const fadeStart = fadeMs * 0.6;
      if (age > fadeStart) {
        opacity = Math.max(0, 1 - (age - fadeStart) / (fadeMs * 0.4));
      }
    }, 50);
    return () => clearInterval(interval);
  });
</script>

<div
  class="key-label"
  class:combo={entry.is_combo}
  style="opacity: {opacity}"
>
  {entry.label}
</div>

<style>
  .key-label {
    background: var(--kk-bg, rgba(0, 0, 0, 0.75));
    color: var(--kk-text, #ffffff);
    padding: var(--kk-padding-y, 6px) var(--kk-padding-x, 14px);
    border-radius: var(--kk-radius, 6px);
    font-family: var(--kk-font, system-ui, -apple-system, sans-serif);
    font-size: var(--kk-font-size, 15px);
    font-weight: var(--kk-font-weight, 500);
    border: var(--kk-border-width, 0px) solid var(--kk-border-color, transparent);
    box-shadow: var(--kk-shadow, none);
    white-space: nowrap;
    transition: opacity 0.15s ease-out;
    animation: kk-enter 0.15s ease-out;
  }

  .combo {
    font-weight: var(--kk-combo-font-weight, 600);
  }

  @keyframes kk-enter {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
