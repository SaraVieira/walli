import { create } from "zustand";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { ipc, onSettingsChanged } from "../shared/ipc";
import type { Collection, Settings } from "../shared/types";

type State = {
  settings: Settings | null;
  collections: Collection[];
  refresh: () => Promise<void>;
  patch: (p: Partial<Settings>) => Promise<void>;
};

export const useSettingsStore = create<State>((set) => ({
  settings: null,
  collections: [],
  refresh: async () => {
    const [s, c] = await Promise.all([
      ipc.getSettings(),
      ipc.listCollections(),
    ]);
    set({ settings: s, collections: c });
  },
  patch: async (p) => {
    const next = await ipc.updateSettings(p);
    set({ settings: next });
  },
}));

let bound = false;
const unlistens: Array<Promise<UnlistenFn>> = [];

export function bindSettingsEvents() {
  if (bound) return;
  bound = true;
  unlistens.push(
    onSettingsChanged(() => useSettingsStore.getState().refresh()),
  );
}

if (import.meta.hot) {
  import.meta.hot.dispose(() => {
    unlistens.forEach((p) => p.then((u) => u()).catch(() => {}));
    unlistens.length = 0;
    bound = false;
  });
}
