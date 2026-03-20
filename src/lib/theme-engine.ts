export interface Theme {
  name: string;
  label: string;
  vars: Record<string, string>;
}

export const BUILT_IN_THEMES: Theme[] = [
  {
    name: 'dark-glass',
    label: 'Dark glass',
    vars: {
      '--kk-bg': 'rgba(12, 12, 18, 0.92)',
      '--kk-text': '#f0f0f0',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '20px',
      '--kk-font-weight': '600',
      '--kk-combo-font-weight': '700',
      '--kk-padding-x': '18px',
      '--kk-padding-y': '10px',
      '--kk-radius': '10px',
      '--kk-border-width': '1px',
      '--kk-border-color': 'rgba(255, 255, 255, 0.12)',
      '--kk-shadow': '0 4px 20px rgba(0, 0, 0, 0.5)',
      '--kk-gap': '6px',
    },
  },
  {
    name: 'light-minimal',
    label: 'Light minimal',
    vars: {
      '--kk-bg': 'rgba(255, 255, 255, 0.9)',
      '--kk-text': '#1a1a1a',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '13px',
      '--kk-font-weight': '400',
      '--kk-combo-font-weight': '600',
      '--kk-padding-x': '10px',
      '--kk-padding-y': '5px',
      '--kk-radius': '4px',
      '--kk-border-width': '0px',
      '--kk-border-color': 'transparent',
      '--kk-shadow': '0 1px 4px rgba(0, 0, 0, 0.1)',
      '--kk-gap': '3px',
    },
  },
  {
    name: 'terminal',
    label: 'Terminal',
    vars: {
      '--kk-bg': 'rgba(0, 0, 0, 0.9)',
      '--kk-text': '#33ff33',
      '--kk-font': "'Cascadia Code', 'Consolas', monospace",
      '--kk-font-size': '14px',
      '--kk-font-weight': '400',
      '--kk-combo-font-weight': '700',
      '--kk-padding-x': '12px',
      '--kk-padding-y': '6px',
      '--kk-radius': '0px',
      '--kk-border-width': '1px',
      '--kk-border-color': '#33ff33',
      '--kk-shadow': '0 0 8px rgba(51, 255, 51, 0.2)',
      '--kk-gap': '2px',
    },
  },
  {
    name: 'neon',
    label: 'Neon',
    vars: {
      '--kk-bg': 'rgba(10, 5, 20, 0.85)',
      '--kk-text': '#ff44ff',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '16px',
      '--kk-font-weight': '600',
      '--kk-combo-font-weight': '700',
      '--kk-padding-x': '16px',
      '--kk-padding-y': '8px',
      '--kk-radius': '10px',
      '--kk-border-width': '1px',
      '--kk-border-color': 'rgba(255, 68, 255, 0.4)',
      '--kk-shadow': '0 0 20px rgba(255, 68, 255, 0.3), 0 0 40px rgba(255, 68, 255, 0.1)',
      '--kk-gap': '6px',
    },
  },
  {
    name: 'subtle',
    label: 'Subtle',
    vars: {
      '--kk-bg': 'rgba(0, 0, 0, 0.0)',
      '--kk-text': 'rgba(255, 255, 255, 0.7)',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '14px',
      '--kk-font-weight': '400',
      '--kk-combo-font-weight': '500',
      '--kk-padding-x': '8px',
      '--kk-padding-y': '4px',
      '--kk-radius': '4px',
      '--kk-border-width': '0px',
      '--kk-border-color': 'transparent',
      '--kk-shadow': '0 1px 8px rgba(0, 0, 0, 0.5)',
      '--kk-gap': '3px',
    },
  },
];

export function applyTheme(theme: Theme) {
  const root = document.documentElement;
  for (const [key, value] of Object.entries(theme.vars)) {
    root.style.setProperty(key, value);
  }
}

export function getThemeByName(name: string): Theme | undefined {
  return BUILT_IN_THEMES.find((t) => t.name === name);
}
