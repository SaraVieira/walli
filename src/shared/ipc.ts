import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  AppState,
  Collection,
  HistoryEntry,
  Settings,
  SourceKind,
  Wallpaper,
} from "./types";

export const ipc = {
  getState: () => invoke<AppState>("get_state"),
  nextNow: () => invoke<void>("next_now"),
  setPaused: (paused: boolean) => invoke<void>("set_paused", { paused }),
  toggleFavorite: (wallpaperId: number) =>
    invoke<void>("toggle_favorite", { wallpaperId }),
  setWallpaperFromHistory: (wallpaperId: number) =>
    invoke<void>("set_wallpaper_from_history", { wallpaperId }),
  listHistory: (limit: number, offset: number, favoritesOnly: boolean) =>
    invoke<HistoryEntry[]>("list_history", { limit, offset, favoritesOnly }),
  listCollections: () => invoke<Collection[]>("list_collections"),
  createCollection: (name: string, tags: string[]) =>
    invoke<Collection>("create_collection", { name, tags }),
  updateCollection: (id: number, name: string, tags: string[]) =>
    invoke<Collection>("update_collection", { id, name, tags }),
  deleteCollection: (id: number) => invoke<void>("delete_collection", { id }),
  setActiveCollection: (id: number) =>
    invoke<void>("set_active_collection", { id }),
  getSettings: () => invoke<Settings>("get_settings"),
  updateSettings: (patch: Partial<Settings>) =>
    invoke<Settings>("update_settings", { patch }),
  setApiKey: (source: SourceKind, key: string) =>
    invoke<void>("set_api_key", { source, key }),
  clearApiKey: (source: SourceKind) =>
    invoke<void>("clear_api_key", { source }),
  pickLocalFolder: () => invoke<string | null>("pick_local_folder"),
  setLoginAtStartup: (enabled: boolean) =>
    invoke<void>("set_login_at_startup", { enabled }),
  openSettings: () => invoke<void>("open_settings_window"),
  quitApp: () => invoke<void>("quit_app"),
};

export function onWallpaperChanged(
  handler: (w: Wallpaper) => void,
): Promise<UnlistenFn> {
  return listen<Wallpaper>("wallpaper-changed", (e) => handler(e.payload));
}
export function onError(handler: (msg: string) => void): Promise<UnlistenFn> {
  return listen<string>("error", (e) => handler(e.payload));
}
export function onSettingsChanged(handler: () => void): Promise<UnlistenFn> {
  return listen("settings-changed", () => handler());
}
