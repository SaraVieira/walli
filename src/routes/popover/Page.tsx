import { useEffect } from "react";
import { useWallpaperStore } from "../../store/wallpaper";
import { ipc } from "../../shared/ipc";
import CurrentCard from "./components/CurrentCard";
import Controls from "./components/Controls";
import CollectionSwitcher from "./components/CollectionSwitcher";
import HistoryList from "./components/HistoryList";

export default function PopoverPage() {
  const refresh = useWallpaperStore((s) => s.refresh);
  const errorBanner = useWallpaperStore((s) => s.errorBanner);
  const clearError = useWallpaperStore((s) => s.clearError);
  useEffect(() => {
    refresh();
  }, [refresh]);
  return (
    <div className="flex h-full flex-col gap-3 p-3">
      {errorBanner && (
        <div className="flex items-start gap-2 rounded bg-red-950/60 px-2 py-1.5 text-xs text-red-200">
          <span className="flex-1">{errorBanner}</span>
          <button
            onClick={clearError}
            className="text-red-300 hover:text-red-100"
          >
            ×
          </button>
        </div>
      )}
      <CurrentCard />
      <Controls />
      <CollectionSwitcher />
      <div className="flex-1 overflow-auto">
        <HistoryList />
      </div>
      <div className="flex gap-2 text-xs text-muted">
        <button onClick={() => ipc.openSettings()} className="hover:text-fg">
          Settings…
        </button>
        <button onClick={() => ipc.quitApp()} className="ml-auto hover:text-fg">
          Quit
        </button>
      </div>
    </div>
  );
}
