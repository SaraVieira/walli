import { useEffect } from "react";
import { useWallpaperStore } from "../../store/wallpaper";
import { ipc } from "../../shared/ipc";
import CurrentCard from "./components/CurrentCard";
import CollectionSwitcher from "./components/CollectionSwitcher";

export default function PopoverPage() {
  const refresh = useWallpaperStore((s) => s.refresh);

  useEffect(() => {
    refresh();
  }, [refresh]);

  return (
    <div className="flex h-full flex-col gap-3 p-3">
      <CurrentCard />
      <CollectionSwitcher />
      <div className="flex text-xs ttext-muted-foreground justify-between">
        <button
          onClick={() => ipc.openSettings()}
          className="hover:text-foreground cursor-pointer"
        >
          Settings
        </button>
        <button
          onClick={() => ipc.openHistory()}
          className="hover:text-foreground cursor-pointer"
        >
          History
        </button>
        <button
          onClick={() => ipc.quitApp()}
          className=" hover:text-foreground cursor-pointer"
        >
          Quit
        </button>
      </div>
    </div>
  );
}
