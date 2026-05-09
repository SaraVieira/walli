import { create } from "zustand";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { ipc, onWallpaperChanged, onError } from "../shared/ipc";
import type { AppState, Wallpaper } from "../shared/types";

type State = AppState & {
  loading: boolean;
  errorBanner: string | null;
  refresh: () => Promise<void>;
  next: () => Promise<void>;
  setPaused: (p: boolean) => Promise<void>;
  clearError: () => void;
  beginLoading: () => void;
};

let nextTimeout: ReturnType<typeof setTimeout> | null = null;

function clearNextTimeout() {
  if (nextTimeout !== null) {
    clearTimeout(nextTimeout);
    nextTimeout = null;
  }
}

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
    set({ loading: true });
    clearNextTimeout();
    nextTimeout = setTimeout(() => {
      nextTimeout = null;
      useWallpaperStore.setState({ loading: false });
    }, 30000);
    try {
      await ipc.nextNow();
    } catch (e) {
      clearNextTimeout();
      set({ loading: false, errorBanner: String(e) });
    }
  },
  setPaused: async (p) => {
    await ipc.setPaused(p);
    set({ paused: p });
  },
  clearError: () => set({ errorBanner: null }),
  beginLoading: () => {
    set({ loading: true });
    clearNextTimeout();
    nextTimeout = setTimeout(() => {
      nextTimeout = null;
      useWallpaperStore.setState({ loading: false });
    }, 30000);
  },
}));

let bound = false;
const unlistens: Array<Promise<UnlistenFn>> = [];

export function bindWallpaperEvents() {
  if (bound) return;
  bound = true;
  unlistens.push(
    onWallpaperChanged((w: Wallpaper) => {
      clearNextTimeout();
      useWallpaperStore.setState({ current: w, loading: false });
    }),
  );
  unlistens.push(
    onError((m) => {
      clearNextTimeout();
      useWallpaperStore.setState({ errorBanner: m, loading: false });
    }),
  );
}

if (import.meta.hot) {
  import.meta.hot.dispose(() => {
    unlistens.forEach((p) => p.then((u) => u()).catch(() => {}));
    unlistens.length = 0;
    bound = false;
  });
}
