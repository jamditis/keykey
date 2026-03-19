<script lang="ts">
  import type { KeyDef } from '../keyboard-layouts/ansi-104';

  interface Props {
    keyDef: KeyDef;
    pressed: boolean;
  }

  let { keyDef, pressed }: Props = $props();

  const BASE_SIZE = 40;
  const GAP = 2;
</script>

<div
  class="keycap"
  class:pressed
  style="width: {keyDef.width * BASE_SIZE + (keyDef.width - 1) * GAP}px"
>
  {keyDef.label}
</div>

<style>
  .keycap {
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--kk-kb-key-bg, rgba(40, 40, 50, 0.85));
    color: var(--kk-kb-key-text, #d0d0d0);
    border: 1px solid var(--kk-kb-key-border, rgba(255, 255, 255, 0.12));
    border-radius: var(--kk-kb-key-radius, 5px);
    font-family: var(--kk-font, 'Segoe UI', system-ui, sans-serif);
    font-size: var(--kk-kb-font-size, 11px);
    font-weight: 500;
    box-shadow: var(--kk-kb-key-shadow, 0 2px 0 rgba(0, 0, 0, 0.4), inset 0 1px 0 rgba(255, 255, 255, 0.08));
    user-select: none;
    flex-shrink: 0;
    transition: background 0.08s ease, transform 0.08s ease, box-shadow 0.08s ease;
  }

  .keycap.pressed {
    background: var(--kk-kb-key-pressed-bg, rgba(120, 160, 255, 0.75));
    color: var(--kk-kb-key-pressed-text, #ffffff);
    border-color: var(--kk-kb-key-pressed-border, rgba(150, 190, 255, 0.5));
    box-shadow: var(--kk-kb-key-pressed-shadow, 0 0 8px rgba(100, 140, 255, 0.4));
    transform: scale(0.95);
  }
</style>
