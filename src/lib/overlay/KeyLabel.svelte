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
    background: var(--kk-bg, rgba(12, 12, 18, 0.92));
    color: var(--kk-text, #f0f0f0);
    padding: var(--kk-padding-y, 10px) var(--kk-padding-x, 18px);
    border-radius: var(--kk-radius, 10px);
    font-family: var(--kk-font, 'Segoe UI', system-ui, sans-serif);
    font-size: var(--kk-font-size, 20px);
    font-weight: var(--kk-font-weight, 600);
    border: var(--kk-border-width, 1px) solid var(--kk-border-color, rgba(255, 255, 255, 0.12));
    box-shadow: var(--kk-shadow, 0 4px 20px rgba(0, 0, 0, 0.5));
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    white-space: nowrap;
    letter-spacing: 0.01em;
    transition: opacity 0.15s ease-out;
    animation: kk-enter 0.15s ease-out;
  }

  .combo {
    font-weight: var(--kk-combo-font-weight, 700);
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
