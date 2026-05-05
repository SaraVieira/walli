export type SourceKind = "unsplash" | "bing";

export interface Wallpaper {
  id: number;
  source: SourceKind;
  source_id: string;
  photographer: string | null;
  title: string | null;
  source_url: string | null;
  file_path: string;
  is_local: boolean;
  width: number | null;
  height: number | null;
  fetched_at: number;
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
  source_bing_enabled: boolean;
  unsplash_key_set: boolean;
  login_at_startup: boolean;
}

export interface AppState {
  current: Wallpaper | null;
  paused: boolean;
  interval_seconds: number;
  active_collection_id: number | null;
  error_banner: string | null;
}
