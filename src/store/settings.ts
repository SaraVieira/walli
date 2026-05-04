import { create } from "zustand";
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

export function bindSettingsEvents() {
  onSettingsChanged(() => useSettingsStore.getState().refresh());
}
