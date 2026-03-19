import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface AppConfig {
  version: number;
  display: {
    mode: string;
    position_strategy: string;
    corner: string;
    margin_x: number;
    margin_y: number;
    max_visible: number;
    fade_duration_ms: number;
    stack_direction: string;
  };
  input: {
    smart_mode: boolean;
    smart_threshold_ms: number;
    repeat_window_ms: number;
    show_all_keystrokes: boolean;
    modifier_overrides: {
      ctrl: string;
      alt: string;
      shift: string;
      win: string;
    };
  };
  appearance: {
    active_theme: string;
  };
  shortcuts: {
    toggle_capture: string | null;
    switch_mode: string | null;
    toggle_overlay: string | null;
  };
}

function createConfigStore() {
  const { subscribe, set, update } = writable<AppConfig | null>(null);
  return {
    subscribe,
    async load() {
      const config = await invoke<AppConfig>('get_config');
      set(config);
      return config;
    },
    async save(config: AppConfig) {
      await invoke('save_config', { config });
      set(config);
    },
    update,
  };
}

export const configStore = createConfigStore();
