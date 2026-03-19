export interface KeyDef {
  code: string;   // matches rdev Key debug name (e.g., "KeyA", "ControlLeft")
  label: string;  // display text
  width: number;  // relative width (1 = standard key)
  row: number;    // 0=function row, 1-5=main rows
}

export const ANSI_COMPACT: KeyDef[] = [
  // Row 0: function row
  { code: 'Escape',      label: 'Esc',  width: 1,    row: 0 },
  { code: 'F1',          label: 'F1',   width: 1,    row: 0 },
  { code: 'F2',          label: 'F2',   width: 1,    row: 0 },
  { code: 'F3',          label: 'F3',   width: 1,    row: 0 },
  { code: 'F4',          label: 'F4',   width: 1,    row: 0 },
  { code: 'F5',          label: 'F5',   width: 1,    row: 0 },
  { code: 'F6',          label: 'F6',   width: 1,    row: 0 },
  { code: 'F7',          label: 'F7',   width: 1,    row: 0 },
  { code: 'F8',          label: 'F8',   width: 1,    row: 0 },
  { code: 'F9',          label: 'F9',   width: 1,    row: 0 },
  { code: 'F10',         label: 'F10',  width: 1,    row: 0 },
  { code: 'F11',         label: 'F11',  width: 1,    row: 0 },
  { code: 'F12',         label: 'F12',  width: 1,    row: 0 },

  // Row 1: number row
  { code: 'BackQuote',   label: '`',    width: 1,    row: 1 },
  { code: 'Num1',        label: '1',    width: 1,    row: 1 },
  { code: 'Num2',        label: '2',    width: 1,    row: 1 },
  { code: 'Num3',        label: '3',    width: 1,    row: 1 },
  { code: 'Num4',        label: '4',    width: 1,    row: 1 },
  { code: 'Num5',        label: '5',    width: 1,    row: 1 },
  { code: 'Num6',        label: '6',    width: 1,    row: 1 },
  { code: 'Num7',        label: '7',    width: 1,    row: 1 },
  { code: 'Num8',        label: '8',    width: 1,    row: 1 },
  { code: 'Num9',        label: '9',    width: 1,    row: 1 },
  { code: 'Num0',        label: '0',    width: 1,    row: 1 },
  { code: 'Minus',       label: '-',    width: 1,    row: 1 },
  { code: 'Equal',       label: '=',    width: 1,    row: 1 },
  { code: 'BackSpace',   label: 'Bksp', width: 2,    row: 1 },

  // Row 2: QWERTY row
  { code: 'Tab',         label: 'Tab',  width: 1.5,  row: 2 },
  { code: 'KeyQ',        label: 'Q',    width: 1,    row: 2 },
  { code: 'KeyW',        label: 'W',    width: 1,    row: 2 },
  { code: 'KeyE',        label: 'E',    width: 1,    row: 2 },
  { code: 'KeyR',        label: 'R',    width: 1,    row: 2 },
  { code: 'KeyT',        label: 'T',    width: 1,    row: 2 },
  { code: 'KeyY',        label: 'Y',    width: 1,    row: 2 },
  { code: 'KeyU',        label: 'U',    width: 1,    row: 2 },
  { code: 'KeyI',        label: 'I',    width: 1,    row: 2 },
  { code: 'KeyO',        label: 'O',    width: 1,    row: 2 },
  { code: 'KeyP',        label: 'P',    width: 1,    row: 2 },
  { code: 'LeftBracket', label: '[',    width: 1,    row: 2 },
  { code: 'RightBracket',label: ']',    width: 1,    row: 2 },
  { code: 'BackSlash',   label: '\\',   width: 1.5,  row: 2 },

  // Row 3: home row
  { code: 'CapsLock',    label: 'Caps', width: 1.75, row: 3 },
  { code: 'KeyA',        label: 'A',    width: 1,    row: 3 },
  { code: 'KeyS',        label: 'S',    width: 1,    row: 3 },
  { code: 'KeyD',        label: 'D',    width: 1,    row: 3 },
  { code: 'KeyF',        label: 'F',    width: 1,    row: 3 },
  { code: 'KeyG',        label: 'G',    width: 1,    row: 3 },
  { code: 'KeyH',        label: 'H',    width: 1,    row: 3 },
  { code: 'KeyJ',        label: 'J',    width: 1,    row: 3 },
  { code: 'KeyK',        label: 'K',    width: 1,    row: 3 },
  { code: 'KeyL',        label: 'L',    width: 1,    row: 3 },
  { code: 'SemiColon',   label: ';',    width: 1,    row: 3 },
  { code: 'Quote',       label: "'",    width: 1,    row: 3 },
  { code: 'Return',      label: 'Enter',width: 2.25, row: 3 },

  // Row 4: shift row
  { code: 'ShiftLeft',   label: 'Shift',width: 2.25, row: 4 },
  { code: 'KeyZ',        label: 'Z',    width: 1,    row: 4 },
  { code: 'KeyX',        label: 'X',    width: 1,    row: 4 },
  { code: 'KeyC',        label: 'C',    width: 1,    row: 4 },
  { code: 'KeyV',        label: 'V',    width: 1,    row: 4 },
  { code: 'KeyB',        label: 'B',    width: 1,    row: 4 },
  { code: 'KeyN',        label: 'N',    width: 1,    row: 4 },
  { code: 'KeyM',        label: 'M',    width: 1,    row: 4 },
  { code: 'Comma',       label: ',',    width: 1,    row: 4 },
  { code: 'Dot',         label: '.',    width: 1,    row: 4 },
  { code: 'Slash',       label: '/',    width: 1,    row: 4 },
  { code: 'ShiftRight',  label: 'Shift',width: 2.75, row: 4 },

  // Row 5: bottom row
  { code: 'ControlLeft', label: 'Ctrl', width: 1.25, row: 5 },
  { code: 'MetaLeft',    label: 'Win',  width: 1.25, row: 5 },
  { code: 'Alt',         label: 'Alt',  width: 1.25, row: 5 },
  { code: 'Space',       label: '',     width: 6.25, row: 5 },
  { code: 'AltGr',       label: 'AltGr',width: 1.25, row: 5 },
  { code: 'MetaRight',   label: 'Win',  width: 1.25, row: 5 },
  { code: 'ControlRight',label: 'Ctrl', width: 1.25, row: 5 },
];
