import { writable } from 'svelte/store';
import type { DisplayEvent, StreamEntry } from '../types';

const MAX_VISIBLE = 5;
const FADE_DURATION_MS = 2000;

function createKeystream() {
  const { subscribe, update } = writable<StreamEntry[]>([]);
  let cleanupTimer: ReturnType<typeof setInterval> | null = null;

  function startCleanup() {
    if (cleanupTimer) return;
    cleanupTimer = setInterval(() => {
      const now = Date.now();
      update((entries) =>
        entries.filter((e) => now - e.created_at < FADE_DURATION_MS)
      );
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
    push(event: DisplayEvent) {
      startCleanup();
      update((entries) => {
        if (entries.length > 0 && entries[entries.length - 1].id === event.id) {
          const updated = [...entries];
          updated[updated.length - 1] = {
            ...updated[updated.length - 1],
            label: event.label,
            repeat_count: updated[updated.length - 1].repeat_count + 1,
          };
          return updated;
        }

        const newEntry: StreamEntry = {
          label: event.label,
          id: event.id,
          is_combo: event.is_combo,
          created_at: Date.now(),
          repeat_count: 1,
        };

        const updated = [...entries, newEntry];
        return updated.slice(-MAX_VISIBLE);
      });
    },
    clear() { update(() => []); },
    destroy() { stopCleanup(); },
  };
}

export const keystream = createKeystream();
