export interface DisplayEvent {
  label: string;
  is_combo: boolean;
  id: number;
  timestamp: number;
}

export interface StreamEntry {
  label: string;
  id: number;
  is_combo: boolean;
  created_at: number;
  repeat_count: number;
}
