import { create } from "zustand";
import { ipc, onWallpaperChanged, onError } from "../shared/ipc";
import type { AppState, Wallpaper } from "../shared/types";

type State = AppState & {
  loading: boolean;
  errorBanner: string | null;
  refresh: () => Promise<void>;
  next: () => Promise<void>;
  setPaused: (p: boolean) => Promise<void>;
  clearError: () => void;
};

export const useWallpaperStore = create<State>((set) => ({
  current: null,
  paused: false,
  interval_seconds: 3600,
  active_collection_id: null,
  error_banner: null,
  loading: false,
  errorBanner: null,
  refresh: async () => {
    set({ loading: true });
    try {
      const s = await ipc.getState();
      set({ ...s, loading: false });
    } catch (e) {
      set({ loading: false, errorBanner: String(e) });
    }
  },
  next: async () => {
    await ipc.nextNow();
  },
  setPaused: async (p) => {
    await ipc.setPaused(p);
    set({ paused: p });
  },
  clearError: () => set({ errorBanner: null }),
}));

export function bindWallpaperEvents() {
  onWallpaperChanged((w: Wallpaper) =>
    useWallpaperStore.setState({ current: w }),
  );
  onError((m) => useWallpaperStore.setState({ errorBanner: m }));
}
