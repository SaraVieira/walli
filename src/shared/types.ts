export type SourceKind = "unsplash" | "wallhaven" | "bing" | "apod" | "local";

export interface Wallpaper {
  id: number;
  source: SourceKind;
  source_id: string;
  photographer: string | null;
  source_url: string | null;
  file_path: string;
  is_local: boolean;
  width: number | null;
  height: number | null;
  fetched_at: number;
}

export interface HistoryEntry {
  history_id: number;
  wallpaper: Wallpaper;
  set_at: number;
  display_id: string | null;
  is_favorite: boolean;
}

export interface Collection {
  id: number;
  name: string;
  tags: string[];
}

export interface Settings {
  interval_seconds: number;
  per_display_mode: boolean;
  paused: boolean;
  active_collection_id: number | null;
  source_unsplash_enabled: boolean;
  source_wallhaven_enabled: boolean;
  source_bing_enabled: boolean;
  source_apod_enabled: boolean;
  source_local_enabled: boolean;
  local_folder_path: string | null;
  unsplash_key_set: boolean;
  wallhaven_key_set: boolean;
  login_at_startup: boolean;
}

export interface AppState {
  current: Wallpaper | null;
  paused: boolean;
  interval_seconds: number;
  active_collection_id: number | null;
  error_banner: string | null;
}
