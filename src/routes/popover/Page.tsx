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
    <div className="flex h-full flex-col gap-3">
      <CurrentCard />
      <div className="px-4 flex flex-col ">
        <CollectionSwitcher />
        <div className="flex text-xs ttext-muted-foreground justify-between mt-4">
          <button
            onClick={() => ipc.openSettings()}
            className="hover:text-foreground cursor-pointer"
          >
            Settings
          </button>
          <button
            onClick={() => ipc.quitApp()}
            className=" hover:text-foreground cursor-pointer"
          >
            Quit
          </button>
        </div>
      </div>
    </div>
  );
}
