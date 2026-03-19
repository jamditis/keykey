import { writable } from 'svelte/store';
import type { DisplayEvent, StreamEntry } from '../types';

function createKeystream() {
  const { subscribe, update } = writable<StreamEntry[]>([]);
  let cleanupTimer: ReturnType<typeof setInterval> | null = null;
  let maxVisible = 5;
  let fadeDurationMs = 2000;

  function startCleanup() {
    if (cleanupTimer) return;
    cleanupTimer = setInterval(() => {
      const now = Date.now();
      update((entries) => {
        const filtered = entries.filter((e) => now - e.created_at < fadeDurationMs);
        // Stop polling when stream is empty
        if (filtered.length === 0) {
          stopCleanup();
        }
        return filtered;
      });
    }, 100);
  }

  function stopCleanup() {
    if (cleanupTimer) {
      clearInterval(cleanupTimer);
      cleanupTimer = null;
    }
  }

  return {
    subscribe,
    configure(opts: { maxVisible?: number; fadeDurationMs?: number }) {
      if (opts.maxVisible !== undefined) maxVisible = opts.maxVisible;
      if (opts.fadeDurationMs !== undefined) fadeDurationMs = opts.fadeDurationMs;
    },
    push(event: DisplayEvent) {
      startCleanup();
      update((entries) => {
        // Repeat compression: compare by base label (strip " xN" suffix)
        const baseLabel = event.label.replace(/ x\d+$/, '');
        if (entries.length > 0) {
          const last = entries[entries.length - 1];
          const lastBase = last.label.replace(/ x\d+$/, '');
          if (lastBase === baseLabel) {
            const updated = [...entries];
            updated[updated.length - 1] = {
              ...last,
              label: event.label,
              repeat_count: last.repeat_count + 1,
            };
            return updated;
          }
        }

        const newEntry: StreamEntry = {
          label: event.label,
          id: event.id,
          is_combo: event.is_combo,
          created_at: Date.now(),
          repeat_count: 1,
        };

        const updated = [...entries, newEntry];
        return updated.slice(-maxVisible);
      });
    },
    clear() { update(() => []); stopCleanup(); },
    destroy() { stopCleanup(); },
  };
}

export const keystream = createKeystream();
