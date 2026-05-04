import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ipc } from "../../../shared/ipc";
import type { HistoryEntry } from "../../../shared/types";

export default function HistoryList() {
  const [items, setItems] = useState<HistoryEntry[]>([]);
  const [favOnly, setFavOnly] = useState(false);
  useEffect(() => {
    ipc.listHistory(20, 0, favOnly).then(setItems);
  }, [favOnly]);
  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between text-xs">
        <span className="font-medium">History</span>
        <label className="flex items-center gap-1 text-muted">
          <input
            type="checkbox"
            checked={favOnly}
            onChange={(e) => setFavOnly(e.target.checked)}
          />
          Favorites only
        </label>
      </div>
      <div className="grid grid-cols-3 gap-1">
        {items.map((h) => (
          <button
            key={h.history_id}
            className="aspect-video overflow-hidden rounded bg-neutral-900"
            onClick={() => ipc.setWallpaperFromHistory(h.wallpaper.id)}
          >
            <img
              src={convertFileSrc(h.wallpaper.file_path)}
              alt=""
              className="h-full w-full object-cover"
            />
          </button>
        ))}
      </div>
    </div>
  );
}
